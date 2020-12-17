use std::collections::HashMap;
use std::collections::HashSet;

use regex::Regex;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Rule {
    field: String,
    bounds: Vec<(i64, i64)>,
}

impl From<&str> for Rule {
    fn from(s: &str) -> Self {
        let re = Regex::new(r"(.*): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
        let caps = re.captures(s).unwrap();
        let field = caps.get(1).unwrap().as_str().to_owned();
        let bounds = vec![
            (
                caps.get(2).unwrap().as_str().parse().unwrap(),
                caps.get(3).unwrap().as_str().parse().unwrap(),
            ),
            (
                caps.get(4).unwrap().as_str().parse().unwrap(),
                caps.get(5).unwrap().as_str().parse().unwrap(),
            ),
        ];
        Rule { field, bounds }
    }
}

type Ticket = Vec<i64>;

#[aoc_generator(day16)]
pub fn parse(input: &str) -> (Vec<Rule>, Ticket, Vec<Ticket>) {
    let mut groups = input.split("\n\n");
    let rules: Vec<Rule> = groups.next().unwrap().lines().map(|x| x.into()).collect();
    for rule in rules.iter() {
        println!("{:?}", rule);
    }

    let my_ticket_lines = groups.next().unwrap();

    let my_ticket = my_ticket_lines
        .lines()
        .nth(1)
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();
    println!("{:?}", my_ticket);

    let neary_ticket_lines = groups.next().unwrap();
    let nearby_tickets = neary_ticket_lines
        .lines()
        .skip(1)
        .map(|s| s.split(",").map(|x| x.parse().unwrap()).collect())
        .collect();

    (rules, my_ticket, nearby_tickets)
}

/// This one's a bit more complex than the problems so far
/// Our strategy is to deterimne a single list of (start, end) pairs that indicate valid ticketes
/// To do this, we have to merge all of the (start, end) ranges we are given.
/// This is probably faster than checking each field of each ticket against every rule.
/// But it's a bit of a pain.
#[aoc(day16, part1)]
pub fn day1(input: &(Vec<Rule>, Ticket, Vec<Ticket>)) -> i64 {
    let mut merged_bounds = vec![];
    for rule in input.0.iter() {
        merged_bounds = merge(&merged_bounds, &rule.bounds);
    }
    println!("{:?}", merged_bounds);
    let mut ans = 0;
    for ticket in &input.2 {
        for val in ticket.iter() {
            if !in_bounds(*val, &merged_bounds) {
                ans += val;
            }
        }
    }
    ans
}

/// Merge is a bit more general than we need, given that in this problem, we never have
/// more than two elements in either thing.
/// But! It merges to lists of bounds, assuming that each is sorted by first element,
/// AND assuming that within each list, there's no overlap (i.e. [(1, 5), (4, 7)] is not allowed)
/// And produced a merged list of bounds satisfying the same conditions.
fn merge(bounds_1: &[(i64, i64)], bounds_2: &[(i64, i64)]) -> Vec<(i64, i64)> {
    let mut idx_1 = 0;
    let mut idx_2 = 0;
    let mut result = vec![];
    while idx_1 < bounds_1.len() || idx_2 < bounds_2.len() {
        // among the bounds_1 and bounds_2 at indices, pull the bound that starts first
        let next = if idx_1 < bounds_1.len() && idx_2 < bounds_2.len() {
            let b1 = bounds_1[idx_1];
            let b2 = bounds_2[idx_2];
            if b1.0 < b2.0 {
                idx_1 += 1;
                b1
            } else {
                idx_2 += 1;
                b2
            }
        } else if idx_1 < bounds_1.len() {
            idx_1 += 1;
            bounds_1[idx_1 - 1]
        } else {
            idx_2 += 1;
            bounds_2[idx_2 - 1]
        };
        if result.is_empty() {
            result.push(next);
        } else {
            // check if we should merge this with the last element of result
            let last = result.last_mut().unwrap();
            if last.0 <= next.0 && last.1 >= next.0 {
                last.1 = next.1.max(last.1);
            } else {
                result.push(next);
            }
        }
    }
    result
}

fn in_bounds(key: i64, bounds: &[(i64, i64)]) -> bool {
    for (a, b) in bounds {
        if key >= *a && key <= *b {
            return true;
        }
    }
    return false;
}

#[aoc(day16, part2)]
pub fn day2(input: &(Vec<Rule>, Ticket, Vec<Ticket>)) -> i64 {
    // First we need to mimic part 1 to filter out the invalid tickets
    let mut merged_bounds = vec![];
    for rule in input.0.iter() {
        merged_bounds = merge(&merged_bounds, &rule.bounds);
    }

    let mut potential_tickets = vec![];

    for ticket in &input.2 {
        if ticket.iter().all(|&v| in_bounds(v, &merged_bounds)) {
            potential_tickets.push(ticket);
        }
    }

    // next, we need to determine, for each rule, the possibilities for which field it is
    // thus possibilities is a map from Rule => usize (index into ticket)
    let num_fields = input.1.len();
    let mut possibilities = HashMap::new();
    for rule in &input.0 {
        // for each rule, iterate over all possible tickets
        // as we go, narrow down the list of possible fields that the rule could be
        let mut potential_fields: HashSet<_> = (0..num_fields).collect();
        for ticket in &potential_tickets {
            let fields_copy = potential_fields.iter().map(|x| *x).collect::<Vec<_>>();
            for field in fields_copy {
                let value = ticket[field];
                if !in_bounds(value, &rule.bounds) {
                    potential_fields.remove(&field);
                }
            }
        }
        println!("{}: {:?}", rule.field, potential_fields);
        possibilities.insert(rule.clone(), potential_fields);
    }

    // by inspection (i.e those print statement), it wasn't enough to just make that one pass
    // we need another pass to do a logic/constraint solving
    // luckily a pretty simple strategy seems to work:
    // find a rule that has only one possible field, mark that field as unavailble to other fields
    // then repeat until we exhaust all the rules.
    let mut final_mapping: HashMap<&Rule, usize> = HashMap::new();
    while possibilities.len() > 0 {
        // find something with only one possibility
        let only_one = possibilities.iter().find(|(_, v)| v.len() == 1).unwrap();
        // add it to final mapping
        let rule = (*only_one.0).clone();
        let values: Vec<_> = only_one.1.iter().collect();
        let value = *values[0];
        println!("{} is field {}", rule.field, value);
        final_mapping.insert(rule, value);
        possibilities.remove(rule);
        for (_, v) in possibilities.iter_mut() {
            v.remove(&value);
        }
    }

    // Finally, extraction.
    let mut prod = 1;
    for (k, &v) in final_mapping.iter() {
        if k.field.starts_with("departure") {
            prod *= input.1[v];
        }
    }
    prod
}
