use std::collections::HashMap;
use std::convert::TryInto;

/// We track state as:
/// - The current turn (i.e. how many numbers have been said already, plus 1)
/// - The number said on the previous turn
/// - Map of [number]=>[most recent turn] but NOT yet updated for the previous turn
/// Then, we use those to compute the next number, and update the map with the previous turn.
#[aoc(day15, part1)]
pub fn part1(_: &str) -> i64 {
    let mut base = vec![1, 17, 0, 10, 18, 11, 6];
    // let mut base = vec![0, 3, 6];
    let mut prev = base.pop().unwrap();
    let mut map: HashMap<i64, usize> = base.into_iter().zip(1..).collect();
    let mut turn = map.len() + 2;
    while turn <= 2020 {
        match map.get(&prev) {
            Some(v) => {
                let n = (turn - v - 1).try_into().unwrap();
                map.insert(prev, turn - 1);
                prev = n;
            }
            None => {
                map.insert(prev, turn - 1);
                prev = 0;
            }
        }
        println!("on turn {}, prev: {}", turn, prev);
        turn += 1;
    }
    prev
}

/// Err... is there something clver I'm supposed to do here?
/// This is just copy-paste part 1, but with the turn count ramped up.
/// This ran in under 2 seconds which would be the slowest runtime so far this AoC.
/// But it's not outrageous, either...
#[aoc(day15, part2)]
pub fn part2(_: &str) -> i64 {
    let mut base = vec![1, 17, 0, 10, 18, 11, 6];
    let mut prev = base.pop().unwrap();
    let mut map: HashMap<i64, usize> = base.into_iter().zip(1..).collect();
    let mut turn = map.len() + 2;
    while turn <= 30000000 {
        match map.get(&prev) {
            Some(v) => {
                let n = (turn - v - 1).try_into().unwrap();
                map.insert(prev, turn - 1);
                prev = n;
            }
            None => {
                map.insert(prev, turn - 1);
                prev = 0;
            }
        }
        // println!("on turn {}, prev: {}", turn, prev);
        turn += 1;
    }
    prev
}
