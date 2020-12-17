use regex::Regex;
use std::collections::HashMap;

const KEYS: [&str; 8] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];

struct Passport(HashMap<String, String>);
impl Passport {
    fn from_str(s: &str) -> Option<Passport> {
        let mut passport = HashMap::new();
        for pair in s.split_whitespace() {
            let pair: Vec<_> = pair.split(':').collect();
            let key = pair[0];
            let value = pair[1];
            if !KEYS.contains(&key) {
                return None;
            }
            passport.insert(key.to_owned(), value.to_owned());
        }
        let expected_length = if passport.get("cid").is_some() { 8 } else { 7 };
        if passport.len() == expected_length {
            Some(Passport(passport))
        } else {
            None
        }
    }

    fn from_str_with_validation(s: &str) -> Option<Passport> {
        let mut passport = HashMap::new();
        for pair in s.split_whitespace() {
            let pair: Vec<_> = pair.split(':').collect();
            let key = pair[0];
            let value = pair[1];
            if !KEYS.contains(&key) {
                return None;
            }
            let valid = Passport::field_is_valid(key, value);
            // DEBUG println!("{}:{} is {}", key, value, valid);
            if !valid {
                return None;
            }
            passport.insert(key.to_owned(), value.to_owned());
        }
        let expected_length = if passport.get("cid").is_some() { 8 } else { 7 };
        if passport.len() == expected_length {
            Some(Passport(passport))
        } else {
            None
        }
    }

    fn field_is_valid(key: &str, value: &str) -> bool {
        match key {
            "byr" => value
                .parse::<u32>()
                .map(|x| x >= 1920 && x <= 2002)
                .unwrap_or(false),
            "iyr" => value
                .parse::<u32>()
                .map(|x| x >= 2010 && x <= 2020)
                .unwrap_or(false),
            "eyr" => value
                .parse::<u32>()
                .map(|x| x >= 2020 && x <= 2030)
                .unwrap_or(false),
            "hgt" => {
                let chars = value;
                if chars.len() <= 2 {
                    return false;
                }
                let head = &chars[..chars.len() - 2];
                let tail = &chars[chars.len() - 2..];
                let head = match head.to_string().parse::<i32>() {
                    Ok(val) => val,
                    Err(_) => return false,
                };
                match tail {
                    "in" => head >= 59 && head <= 76,
                    "cm" => head >= 150 && head <= 193,
                    _ => false,
                }
            }
            "hcl" => {
                let color_re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
                color_re.is_match(value)
            }
            "ecl" => {
                let ecl_re = Regex::new("^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
                ecl_re.is_match(value)
            }
            "pid" => {
                let pid_re = Regex::new("^[0-9]{9}$").unwrap();
                pid_re.is_match(value)
            }
            "cid" => true,
            _ => false,
        }
    }
}

#[aoc_generator(day4, part1)]
pub fn gen(input: &str) -> i32 {
    let mut cnt = 0;
    for chunk in input.split("\n\n") {
        if Passport::from_str(chunk).is_some() {
            cnt += 1;
        }
    }
    cnt
}

#[aoc(day4, part1)]
pub fn solve_1(input: &i32) -> i32 {
    *input
}

#[aoc_generator(day4, part2)]
pub fn gen2(input: &str) -> i32 {
    let mut cnt = 0;
    for chunk in input.split("\n\n") {
        if Passport::from_str_with_validation(chunk).is_some() {
            cnt += 1;
        }
    }
    cnt
}

#[aoc(day4, part2)]
pub fn solve_2(input: &i32) -> i32 {
    *input
}
