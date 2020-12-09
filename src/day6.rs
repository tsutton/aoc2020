use std::collections::HashSet;
use std::convert::TryInto;

type Group = Vec<HashSet<char>>;

#[aoc_generator(day6)]
pub fn gen(input: &str) -> Vec<Group> {
    let mut groups = vec![];
    for group in input.split("\n\n") {
        groups.push(group.lines().map(|line| line.chars().collect()).collect());
    }
    groups
}

#[aoc(day6, part1)]
pub fn part1(groups: &Vec<Group>) -> i64 {
    let mut ans = 0;
    for group in groups {
        let mut acc = HashSet::new();
        for person in group {
            acc = acc.union(person).cloned().collect();
        }
        ans += acc.len();
    }
    ans.try_into().unwrap()
}

#[aoc(day6, part2)]
pub fn part2(groups: &Vec<Group>) -> i64 {
    let mut ans = 0;
    for group in groups {
        let mut acc = group[0].clone();
        for person in &group[1..] {
            acc = acc.intersection(person).cloned().collect();
        }
        ans += acc.len();
    }
    ans.try_into().unwrap()
}
