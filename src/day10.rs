use std::collections::HashMap;

#[aoc_generator(day10)]
pub fn gen(input: &str) -> Vec<i64> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}

#[aoc(day10, part1)]
pub fn day10(input: &Vec<i64>) -> i64 {
    let mut input = input.clone();
    input.sort();
    let (ones, threes, _) = input.iter().fold((0, 0, 0), |(ones, threes, prev), next| {
        if next - prev == 1 {
            (ones + 1, threes, *next)
        } else if next - prev == 3 {
            (ones, threes + 1, *next)
        } else {
            (ones, threes, *next)
        }
    });
    ones * (threes + 1)
}

/// Dynamic programmic: sort the input and iterate, computing for each adapter the number
/// of chains ending with that adapter. Our answer is the number of ways for the built-in adapter
#[aoc(day10, part2)]
pub fn day10_2(input: &Vec<i64>) -> i64 {
    let mut input = input.clone();
    input.sort();
    input.push(input[input.len() - 1] + 3);
    let mut n_ways: HashMap<i64, i64> = HashMap::new();
    n_ways.insert(0, 1);
    for &adapter in input.iter() {
        let sum = n_ways.get(&(adapter - 3)).cloned().unwrap_or(0)
            + n_ways.get(&(adapter - 2)).cloned().unwrap_or(0)
            + n_ways.get(&(adapter - 1)).cloned().unwrap_or(0);
        n_ways.insert(adapter, sum);
    }
    n_ways.get(&input[input.len() - 1]).cloned().unwrap()
}
