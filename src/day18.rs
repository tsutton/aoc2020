//! Check the comments on parse_expr_1 and parse_expr_2 for main details.
//! Before we get to evaluating an expression, it is convenient to make a pass over
//! The input and tokenize it. It's not necessary, in fact it might be a bit slower, but
//! It makes the parsing logic a lot simpler if we can operate on tokens instead of strings.
//!
//! I could have used any of the lovely parsing libs for
//! Rust (e.g. lalrpop or pest) but it was more fun to do it myself.

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Token {
    LParen,
    RParen,
    Plus,
    Times,
    Value(i64),
}

fn tokenize(mut input: &str) -> Vec<Token> {
    let mut input = &mut input;
    let mut result = vec![];
    while let Some(t) = next_token(&mut input) {
        result.push(t)
    }
    result
}

fn next_token(input: &mut &str) -> Option<Token> {
    *input = &mut input.trim_start();
    if input.is_empty() {
        return None;
    }
    match input.chars().next().unwrap() {
        '(' => {
            *input = &input[1..];
            Some(Token::LParen)
        }
        ')' => {
            *input = &input[1..];
            Some(Token::RParen)
        }
        '+' => {
            *input = &input[1..];
            Some(Token::Plus)
        }
        '*' => {
            *input = &input[1..];
            Some(Token::Times)
        }
        '0'..='9' => {
            let last_idx = input.find(|c| c < '0' || c > '9').unwrap_or(input.len());
            let num_str = &input[..last_idx];
            let ret = Token::Value(num_str.parse().unwrap());
            *input = &input[last_idx..];
            Some(ret)
        }
        _ => panic!(),
    }
}

fn evaluate_1(input: &str) -> i64 {
    let tokens = tokenize(input);
    parse_expr_1(&mut tokens.as_ref())
}

fn evaluate_2(input: &str) -> i64 {
    let tokens = tokenize(input);
    parse_expr_2(tokens.as_ref())
}

/// For part 1, we can implement a simple right-recursive parser.
/// This parser consumes tokens right to left (where here "consume" means update the slice's bounds to
///  no longer include them).
/// We expect an expr to start with a RHS which may be a value or parenthesized expr (recursively)
/// then an op, then an LHS (recursively).
/// Starting from the right ensures the precendence is set properly.
/// Expressions go as far left as they can, stopping either and beginning of input or an LParen
/// The parentheses work because the RParen parser eats the RParen, then eats an expr, then
/// makes sure to eat the LParen at the other side. If there were parenthesis inside that group
/// They will have been consumed recursively.
fn parse_expr_1(input: &mut &[Token]) -> i64 {
    let len = input.len();
    let last = input[len - 1];
    let rhs = match last {
        Token::Value(v) => {
            *input = &input[..len - 1];
            v
        }
        Token::RParen => {
            *input = &input[..len - 1];
            let r = parse_expr_1(input);
            *input = &input[..input.len() - 1]; // lparen
            r
        }
        _ => panic!("bad token in final position of expr {:?}", last),
    };
    if input.is_empty() {
        return rhs;
    }
    let op = input[input.len() - 1];
    if let Token::LParen = op {
        return rhs;
    }
    *input = &input[..input.len() - 1];
    let lhs = parse_expr_1(input);
    match op {
        Token::Plus => lhs + rhs,
        Token::Times => lhs * rhs,
        _ => panic!("bad token in op position: {:?}", op),
    }
}

/// This time is different, we are going to consume tokens left to right, keeping them in a stack
/// As we go, we will always pause for opportunities to reduce the stack by  executing an operation.
/// The reduction points are:
/// - Stack is topped by Value + Value, next token is anything
/// - Stack is topped by Right Paren, next token is anything
/// - Stack is topped by Value * Value, next token is empty or *
/// Strictly speaking, doing it this way (consuming left to right) is determining the associativity
/// (it's making a + b + c work out as (a+b) + c )
/// But since we're working with normal + and *, they are actually associative, so it doesn't matter how we do it.
///
/// In a real parser, there are a few key differences:
/// - a real parser doesn't have to constantly check for every possible reduction. Rather,
///   the real parser is a *state machine* - it always knows which reduction is coming next
/// - In some ways the truly hard part of parsing is error handling - how can you give the user
///   a meaningful error message, guess what they were trying to do, or ignore a piece of the input and
///   continue onward, so that you can e.g. catch multiple errors in a single compile.
/// We don't have to do any of that :)
fn parse_expr_2(input: &[Token]) -> i64 {
    let mut stack = vec![];
    for (i, &token) in input.iter().enumerate() {
        stack.push(token);
        let peek = if i + 1 < input.len() {
            Some(input[i + 1])
        } else {
            None
        };
        println!("{:?}, {:?}", stack, peek);
        while reduce_once(&mut stack, peek) {
            println!("{:?}, {:?}", stack, peek);
        }
    }
    let t = stack.pop().unwrap();
    if let Token::Value(v) = t {
        v
    } else {
        panic!()
    }
}

fn reduce_once(stack: &mut Vec<Token>, peek: Option<Token>) -> bool {
    let token = stack.pop().unwrap();
    if let Token::Value(rhs) = token {
        let l = stack.len();
        if l >= 2 && stack[l - 1] == Token::Plus {
            if let Token::Value(lhs) = stack[l - 2] {
                stack.pop();
                stack.pop();
                stack.push(Token::Value(lhs + rhs));
                println!("reduced by plus, pushing {}", lhs + rhs);
                return true;
            } else {
                panic!()
            }
        } else if l >= 2
            && stack[l - 1] == Token::Times
            && (peek == None || peek == Some(Token::Times) || peek == Some(Token::RParen))
        {
            if let Token::Value(lhs) = stack[l - 2] {
                stack.pop();
                stack.pop();
                stack.push(Token::Value(lhs * rhs));
                println!("reduced by times, pushing {}", lhs * rhs);
                return true;
            } else {
                panic!()
            }
        } else {
            println!("no reduction, pushing {:?}", token);
            stack.push(token);
        }
    } else if let Token::RParen = token {
        // on a well formed input, the stack should be "(" then Value
        let value = stack.pop().unwrap();
        stack.pop();
        println!("Got RParen, flattening to {:?}", value);
        stack.push(value);
        return true;
    } else {
        println!("no reduction, pushing {:?}", token);
        stack.push(token);
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_next() {
        let mut s = "1 + (2 * 3) + (4 * (5 + 6))";
        let t = s.clone();
        assert_eq!(next_token(&mut s), Some(Token::Value(1)));

        assert_eq!(next_token(&mut s), Some(Token::Plus));

        assert_eq!(next_token(&mut s), Some(Token::LParen));
        assert_eq!(next_token(&mut s), Some(Token::Value(2)));
        assert_eq!(next_token(&mut s), Some(Token::Times));
        assert_eq!(next_token(&mut s), Some(Token::Value(3)));
        assert_eq!(next_token(&mut s), Some(Token::RParen));

        assert_eq!(next_token(&mut s), Some(Token::Plus));

        assert_eq!(next_token(&mut s), Some(Token::LParen));
        assert_eq!(next_token(&mut s), Some(Token::Value(4)));
        assert_eq!(next_token(&mut s), Some(Token::Times));
        assert_eq!(next_token(&mut s), Some(Token::LParen));
        assert_eq!(next_token(&mut s), Some(Token::Value(5)));
        assert_eq!(next_token(&mut s), Some(Token::Plus));
        assert_eq!(next_token(&mut s), Some(Token::Value(6)));
        assert_eq!(next_token(&mut s), Some(Token::RParen));
        assert_eq!(next_token(&mut s), Some(Token::RParen));

        assert_eq!(next_token(&mut s), None);
        assert_eq!(s, "");

        assert_eq!(
            tokenize(t),
            vec![
                Token::Value(1),
                Token::Plus,
                Token::LParen,
                Token::Value(2),
                Token::Times,
                Token::Value(3),
                Token::RParen,
                Token::Plus,
                Token::LParen,
                Token::Value(4),
                Token::Times,
                Token::LParen,
                Token::Value(5),
                Token::Plus,
                Token::Value(6),
                Token::RParen,
                Token::RParen
            ]
        );
    }

    #[test]
    fn test_parse() {
        assert_eq!(evaluate_1(&mut "1 + 2 * 3 + 4 * 5 + 6"), 71);
        assert_eq!(evaluate_1(&mut "1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(evaluate_1(&mut "(1 + 2) + 3"), 6);
        assert_eq!(evaluate_1(&mut "2 * 3 + (4 * 5)"), 26);

        assert_eq!(evaluate_1(&mut "5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
        assert_eq!(
            evaluate_1(&mut "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            12240
        );
        assert_eq!(
            evaluate_1(&mut "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            13632
        );
    }

    #[test]
    fn test_parse_2() {
        assert_eq!(evaluate_2("1 + 2 * 3 + 4 * 5 + 6"), 231);

        assert_eq!(evaluate_2("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(evaluate_2("2 * 3 + (4 * 5)"), 46);
        assert_eq!(evaluate_2("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
        assert_eq!(
            evaluate_2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            669060
        );
        assert_eq!(
            evaluate_2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2 "),
            23340
        );
    }
}

#[aoc(day18, part1)]
fn day1(input: &str) -> i64 {
    input.lines().map(|line| evaluate_1(line.clone())).sum()
}

#[aoc(day18, part2)]
fn day2(input: &str) -> i64 {
    input.lines().map(|line| evaluate_2(line.clone())).sum()
}
