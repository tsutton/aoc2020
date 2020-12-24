use std::{convert::TryInto, time::Instant};
// use std::{collections::{hash_map::DefaultHasher, HashSet}, hash::{Hash, Hasher}};

#[aoc(day23, part1)]
pub fn part1(input: &str) -> String {
    #[allow(dead_code)]
    const EXAMPLE: &str = "389125467";
    // let input = EXAMPLE;

    let mut cups: Vec<u32> = input.chars().map(|x| x.to_digit(10).unwrap()).collect();
    let mut current_cup = cups[0];
    for _ in 0..100 {
        current_cup = step(&mut cups, current_cup);
    }

    let mut idx = cups.iter().position(|x| x == &1).unwrap();
    let mut answer = String::new();
    for _ in 0..8 {
        idx += 1;
        idx %= 9;
        answer.push_str(&cups[idx].to_string());
    }
    answer
}

fn step(cups: &mut Vec<u32>, current_cup: u32) -> u32 {
    // println!("{:?}", cups);
    // println!("Current: {}", current_cup);

    let mut group = vec![];
    let current_index = cups.iter().position(|x| x == &current_cup).unwrap();
    let max_label: u32 = cups.len().try_into().unwrap();
    // println!("current_idx: {}", current_index);
    let mut idx = (current_index + 1) % cups.len();
    for _ in 0..3 {
        group.push(cups.remove(idx));
        if idx == cups.len() {
            idx = 0;
        }
    }

    //  println!("group: {:?}", group);

    // this is some modular hackery to avoid underflowing our unsigned ints
    // e.g. (x + 7) mod 9 + 1 is equivalent to (x + 8) mod 9 and thus also (x-1) mod 9  but
    // this way it in the range 1, 2, ..., 9
    let mut target_cup = (current_cup + (max_label - 2)) % max_label + 1;
    // println!("trying target cup {}", target_cup);
    while group.contains(&target_cup) {
        target_cup = (target_cup + (max_label - 2)) % max_label + 1;
        // println!("adjusted target cup to {}", target_cup);
    }
    // println!("target_cup: {:?}", target_cup);

    let target_cup_idx = cups.iter().position(|x| x == &target_cup).unwrap();
    // println!("target_cup_idx: {:?}", target_cup_idx);

    cups.insert(target_cup_idx + 1, group[0]);
    cups.insert(target_cup_idx + 2, group[1]);
    cups.insert(target_cup_idx + 3, group[2]);

    let current_index = cups.iter().position(|x| x == &current_cup).unwrap();
    return cups[(current_index + 1) % 9];
    // println!("{:?}", cups);
    // println!("Current: {}", current_cup);
}

/// I was stuck a long time on this problem and looked up how other people structured
/// their code and found this idea
/// We implement a linked list backed by a Vec, where the index is the cup number
/// and the value is the next cup
/// We could make it a more general linked list by having the indices be arbitrary
/// and the values be (next index, value), but we don't need that here!
#[aoc(day23, part2)]
pub fn part2(input: &str) -> usize {
    #[allow(dead_code)]
    const EXAMPLE: &str = "389125467";
    // let input = EXAMPLE;

    let base_cups: Vec<usize> = input
        .chars()
        .map(|x| x.to_digit(10).unwrap().try_into().unwrap())
        .collect();
    let mut cups = Cups::new(&base_cups, 1_000_000);

    // ok good luck see you tomorrow
    let start = Instant::now();
    for i in 0..10_000_000 {
        cups.step();
        if i % 200_000 == 0 {
            println!("step {}, elapsed {:?}", i, start.elapsed())
        }
    }

    let after_one = cups.nexts[1];
    let after_after_one = cups.nexts[after_one];
    after_after_one * after_one
}

#[derive(Debug)]
struct Cups {
    nexts: Vec<usize>,
    current_cup: usize,
}

impl Cups {
    fn new(inital: &[usize], length: usize) -> Cups {
        let mut nexts = Vec::with_capacity(length + 1);
        nexts.resize(length + 1, 0);
        for i in 0..inital.len() - 1 {
            nexts[inital[i]] = inital[i + 1];
        }
        nexts[inital[inital.len() - 1]] = inital.len() + 1;
        for i in inital.len() + 1..length {
            nexts[i] = i + 1;
        }
        nexts[length] = inital[0];
        let current_cup = inital[0];
        Cups { nexts, current_cup }
    }

    fn step(&mut self) {
        let first_removed = self.nexts[self.current_cup];
        let second_removed = self.nexts[first_removed];
        let third_removed = self.nexts[second_removed];

        let m = self.nexts.len() - 1;
        let mut target_cup = (self.current_cup + (m - 2)) % m + 1;
        while target_cup == first_removed
            || target_cup == second_removed
            || target_cup == third_removed
        {
            target_cup = (target_cup + (m - 2)) % m + 1;
        }

	self.nexts[self.current_cup] = self.nexts[third_removed];
	self.nexts[third_removed] = self.nexts[target_cup];
	self.nexts[target_cup] = first_removed;

	self.current_cup = self.nexts[self.current_cup];
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_cup_new() {
        const EXAMPLE: &str = "389125467";
        let cups: Vec<usize> = EXAMPLE
            .chars()
            .map(|x| x.to_digit(10).unwrap().try_into().unwrap())
            .collect();

        let mut cups = Cups::new(&cups, 20);
        println!("{:?}", cups);
        assert_eq!(
            cups.nexts,
            vec![0, 2, 5, 8, 6, 4, 7, 10, 9, 1, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 3]
        );

	cups.step();
        assert_eq!(
            cups.nexts,
            vec![0, 5, 8, 2, 6, 4, 7, 10, 9, 1, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 3]
        );
	assert_eq!(cups.current_cup, 2);
    }
}
