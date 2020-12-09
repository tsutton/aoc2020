use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;

#[aoc_generator(day7)]
pub fn gen(input: &str) -> HashMap<String, Vec<(usize, String)>> {
    let mut rules = HashMap::new();
    for line in input.lines() {
        let words: Vec<_> = line.split(" ").collect();
        let source = words[..2].join(" ");
        if words[4] == "no" {
            rules.insert(source, vec![]);
            continue;
        }
        let n_contains = (words.len() - 4) / 4;
        let mut targets = Vec::with_capacity(n_contains);
        for target_num in 0..n_contains {
            let start = target_num * 4 + 4;
            let count: usize = words[start].parse().unwrap();
            let target = words[start + 1..start + 3].join(" ");
            targets.push((count, target));
        }
        rules.insert(source, targets);
    }
    rules
}

#[aoc(day7, part1)]
pub fn day1(input: &HashMap<String, Vec<(usize, String)>>) -> usize {
    let mut reverse_direct_inclusions = HashMap::new();
    for (bag, contains) in input {
        for (_, containee) in contains {
            reverse_direct_inclusions
                .entry(containee.to_owned())
                .or_insert(vec![])
                .push(bag.to_owned());
        }
    }
    let mut answers: HashSet<String> = HashSet::new();
    let mut queue = reverse_direct_inclusions.get("shiny gold").unwrap().clone();
    while let Some(next) = queue.pop() {
        if answers.contains(&next) {
            continue;
        }
        answers.insert(next.to_owned());
        for n in reverse_direct_inclusions.get(&next).unwrap_or(&vec![]) {
            queue.push(n.to_owned());
        }
    }
    answers.len()
}

#[aoc(day7, part2)]
pub fn day2(input: &HashMap<String, Vec<(usize, String)>>) -> usize {
    let cache = Rc::new(RefCell::new(HashMap::new()));
    day2_helper("shiny gold", input, cache)
}

fn day2_helper(
    target: &str,
    input: &HashMap<String, Vec<(usize, String)>>,
    cache: Rc<RefCell<HashMap<String, usize>>>,
) -> usize {
    if let Some(val) = cache.borrow().get(target) {
        return *val;
    }
    let children = input.get(target).cloned().unwrap_or(vec![]);
    let mut acc = 1;

    for child in children {
        acc += child.0 * day2_helper(&child.1, input, cache.clone());
    }
    cache.borrow_mut().insert(target.to_owned(), acc);
    println!("helper for {} got {}", target, acc);
    acc
}
