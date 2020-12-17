use std::collections::HashMap;
use std::convert::TryInto;

use regex::Regex;

#[derive(Debug)]
pub enum Instr {
    Mask(String),
    Assign(i64, i64),
}

#[aoc_generator(day14)]
pub fn parse(input: &str) -> Vec<Instr> {
    //     let input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
    // mem[8] = 11
    // mem[7] = 101
    // mem[8] = 0";

    //     let input = "mask = 000000000000000000000000000000X1001X
    // mem[42] = 100
    // mask = 00000000000000000000000000000000X0XX
    // mem[26] = 1";

    let mask_regex = Regex::new("mask = ([X01]{36})").unwrap();
    let assign_regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
    let mut result = vec![];
    for line in input.lines() {
        if let Some(caps) = mask_regex.captures(line) {
            result.push(Instr::Mask(caps[1].parse().unwrap()));
        } else if let Some(caps) = assign_regex.captures(line) {
            result.push(Instr::Assign(
                caps[1].parse().unwrap(),
                caps[2].parse().unwrap(),
            ));
        }
    }
    result
}

pub fn int_to_binary_string(i: &i64) -> String {
    format!("{:36b}", i)
}

pub fn apply_mask(s: &i64, mask: &str) -> i64 {
    let binary_str = int_to_binary_string(s);
    let mut result = 0;
    for (i, (c, mask_c)) in binary_str.chars().zip(mask.chars()).enumerate() {
        if mask_c == '1' || (mask_c == 'X' && c == '1') {
            result += 2i64.pow((35 - i).try_into().unwrap());
        }
    }
    result
}

#[aoc(day14, part1)]
pub fn day1(input: &[Instr]) -> i64 {
    let mut mask = "";
    let mut mem: HashMap<i64, i64> = HashMap::new();
    let mut count = 0;
    for instr in input {
        if count <= 10 {
            println!("{:?}", instr);
            count += 1;
        }
        match instr {
            Instr::Mask(s) => mask = s,
            Instr::Assign(addr, val) => {
                let applied = apply_mask(val, &mask);
                if count <= 10 {
                    println!("applied mask {} to {} and got {}", mask, val, applied);
                }
                mem.insert(*addr, applied);
            }
        }
    }
    mem.values().sum()
}

pub fn apply_mask_2(s: &i64, mask: &str) -> Vec<i64> {
    let binary_str = int_to_binary_string(s);
    let mut results = vec![0];
    let mut base = 0;
    for (i, (c, mask_c)) in binary_str.chars().zip(mask.chars()).enumerate() {
        if (mask_c == '0' && c == '1') || mask_c == '1' {
            base += 2i64.pow((35 - i).try_into().unwrap());
        } else if mask_c == 'X' {
            let t = 2i64.pow((35 - i).try_into().unwrap());
            let new_results: Vec<_> = results.iter().map(|x| x + t).collect();
            results.extend(new_results);
        }
    }
    results.iter_mut().for_each(|x| *x += base);
    results
}

#[aoc(day14, part2)]
pub fn day2(input: &[Instr]) -> i64 {
    let mut mask = "";
    let mut mem: HashMap<i64, i64> = HashMap::new();
    let mut count = 0;
    for instr in input {
        if count <= 10 {
            println!("{:?}", instr);
            count += 1;
        }
        match instr {
            Instr::Mask(s) => mask = s,
            Instr::Assign(addr, val) => {
                let addrs = apply_mask_2(addr, &mask);
                if count <= 10 {
                    println!("applied mask {} to {} and got {:?}", mask, val, addrs);
                }
                for addr in addrs {
                    mem.insert(addr, *val);
                }
            }
        }
    }
    mem.values().sum()
}
