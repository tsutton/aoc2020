use regex::Regex;

#[derive(Debug)]
pub struct Row {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

#[aoc_generator(day2)]
pub fn gen(input: &str) -> Vec<Row> {
    let mut ret = vec![];
    let re = Regex::new(r"(\d+)-(\d+) (.): (.+)").unwrap();
    for line in input.lines() {
        let captures = re.captures(line).unwrap();
        ret.push(Row {
            min: captures[1].parse().unwrap(),
            max: captures[2].parse().unwrap(),
            letter: captures[3].chars().next().unwrap(),
            password: captures[4].to_string(),
        })
    }
    ret
}

#[aoc(day2, part1)]
pub fn solve_1(input: &[Row]) -> i32 {
    let mut ans = 0;
    for row in input {
        let count = row.password.matches(row.letter).count();
        if count >= row.min && count <= row.max {
            ans += 1;
        }
    }
    ans
}

#[aoc(day2, part2)]
pub fn solve_2(input: &[Row]) -> i32 {
    let mut ans = 0;
    for row in input {
        let mut count = 0;
        let chars: Vec<_> = row.password.chars().collect();
        if chars[row.min - 1] == row.letter {
            count += 1;
        }
        if chars[row.max - 1] == row.letter {
            count += 1;
        }
        if count == 1 {
            ans += 1;
        }
    }
    ans
}
