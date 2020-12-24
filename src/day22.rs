use std::collections::{HashSet, VecDeque};

fn parse(input: &str) -> (VecDeque<usize>, VecDeque<usize>) {
    let divider = input.find("\n\n").unwrap();
    let player1 = &input[..divider];
    let player2 = &input[divider + 2..];

    (parse_player(player1), parse_player(player2))
}

fn parse_player(input: &str) -> VecDeque<usize> {
    input.lines().skip(1).map(|x| x.parse().unwrap()).collect()
}

fn step(p1: &mut VecDeque<usize>, p2: &mut VecDeque<usize>) -> bool {
    let c1 = p1.pop_front().unwrap();
    let c2 = p2.pop_front().unwrap();
    if c1 < c2 {
        p2.push_back(c2);
        p2.push_back(c1);
        p1.is_empty()
    } else {
        p1.push_back(c1);
        p1.push_back(c2);
        p2.is_empty()
    }
}

fn score(deck: &VecDeque<usize>) -> usize {
    deck.iter()
        .enumerate()
        .map(|(i, &card)| card * (deck.len() - i))
        .sum()
}

#[allow(dead_code)]
const EXAMPLE: &str = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";

#[aoc(day22, part1)]
pub fn part1(input: &str) -> usize {
    // let input = EXAMPLE;
    let (mut p1, mut p2) = parse(input);
    // println!("{:?}", p1);
    // println!("{:?}", p2);
    while !step(&mut p1, &mut p2) {
        // println!("");
        // println!("{:?}", p1);
        // println!("{:?}", p2);
    }
    if p1.is_empty() {
        score(&p2)
    } else {
        score(&p1)
    }
}

enum GameResult {
    P1Game,
    P2Game,
}

// Note: this prints *a lot* which slows it down *a lot*
// On my computer it runs in 10 second swith the prints, and ~700ms with them commented out.
fn play_recursive(p1: &mut VecDeque<usize>, p2: &mut VecDeque<usize>) -> GameResult {
    let mut positions = HashSet::new();
    loop {
        println!();
        println!("P1: {:?}", p1);
        println!("P2: {:?}", p2);
        if positions.contains(&(p1.clone(), p2.clone())) {
            println!("position occured, P1 wins game");
            return GameResult::P1Game;
        }
        positions.insert((p1.clone(), p2.clone()));
        let c1 = p1.pop_front().unwrap();
        let c2 = p2.pop_front().unwrap();
        if c1 <= p1.len() && c2 <= p2.len() {
            println!("entering subgame");
            let mut new_p1 = p1.iter().take(c1).copied().collect();
            let mut new_p2 = p2.iter().take(c2).copied().collect();
            let r = play_recursive(&mut new_p1, &mut new_p2);
            match r {
                GameResult::P1Game => {
                    println!("p1 wins subgame");
                    p1.push_back(c1);
                    p1.push_back(c2);
                }
                GameResult::P2Game => {
                    println!("p2 wins subgame");
                    p2.push_back(c2);
                    p2.push_back(c1);
                }
            }
        } else if c1 < c2 {
            p2.push_back(c2);
            p2.push_back(c1);
        } else {
            p1.push_back(c1);
            p1.push_back(c2);
        }
        if p2.is_empty() {
            return GameResult::P1Game;
        } else if p1.is_empty() {
            return GameResult::P2Game;
        }
    }
}

#[aoc(day22, part2)]
pub fn part2(input: &str) -> usize {
    // let input = EXAMPLE;
    let (mut p1, mut p2) = parse(input);
    // println!("{:?}", p1);
    // println!("{:?}", p2);
    let r = play_recursive(&mut p1, &mut p2);
    match r {
        GameResult::P1Game => score(&p1),
        GameResult::P2Game => score(&p2),
    }
}
