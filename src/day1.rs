use std::collections::HashMap;
use std::collections::HashSet;

#[aoc_generator(day1)]
pub fn day1_parse(input: &str) -> Vec<i32> {
    let mut ret = vec![];
    for line in input.lines() {
        ret.push(line.parse().expect("parses as int"));
    }
    ret
}

#[aoc(day1, part1)]
pub fn day1_solve(vals: &[i32]) -> i32 {
    let mut set = HashSet::new();
    for t in vals {
        if set.contains(&(2020 - t)) {
            return t * (2020 - t);
        } else {
            set.insert(t);
        }
    }
    panic!()
}

#[aoc(day1, part2)]
pub fn day1_solve2(vals: &[i32]) -> i32 {
    let mut twofolds = HashMap::new();
    for t1 in vals {
        for t2 in vals {
            if t1 + t2 < 2020 {
                twofolds.insert(t1 + t2, t1 * t2);
            }
        }
    }
    for z in vals {
        match twofolds.get(&(2020 - z)) {
            Some(product) => return product * z,
            None => continue,
        }
    }
    panic!()
}
