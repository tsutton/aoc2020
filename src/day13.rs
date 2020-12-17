use std::convert::TryInto;

pub struct Input {
    time: i32,
    buses: Vec<i32>,
}

#[aoc_generator(day13, part1)]
pub fn parse(input: &str) -> Input {
    let mut lines = input.lines();
    let time: i32 = lines.next().unwrap().parse().unwrap();
    let mut buses = vec![];
    for bus in lines.next().unwrap().split(',') {
        if bus == "x" {
            continue;
        }
        buses.push(bus.parse().unwrap());
    }
    Input { time, buses }
}

#[aoc(day13, part1)]
pub fn day1(input: &Input) -> i32 {
    let ids_and_minutes = input.buses.iter().map(|bus| (bus, bus - input.time % bus));
    ids_and_minutes
        .min_by_key(|(_, y)| *y)
        .map(|(x, y)| x * y)
        .unwrap()
}

pub struct Input2 {
    buses: Vec<(i64, i64)>,
}

#[aoc_generator(day13, part2)]
pub fn parse2(input: &str) -> Input2 {
    // let input = "\n17,x,13,19";
    let mut lines = input.lines();
    lines.next();
    let mut buses = vec![];
    for (i, bus) in lines.next().unwrap().split(',').enumerate() {
        if bus == "x" {
            continue;
        }
        buses.push((i.try_into().unwrap(), bus.parse().unwrap()));
    }
    Input2 { buses }
}

use modinverse::modinverse;

/// Unraveling the problem, we want a number X such that for each bus with ID n in position I,
/// (X + I) mod n = 0
/// Also, upon inspection, all the IDs are prime.
/// I've implemented the extended euclidean algorithm and the chinese remainder theorem before
/// and I'm too lazy to do it again.
/// Thus I'm gonna cheat and use a dependency to do it.
#[aoc(day13, part2)]
pub fn day2(input: &Input2) -> i64 {
    let product: i64 = input.buses.iter().map(|(_, id)| *id).product();
    println!("product: {}", product);
    let mut answer = 0;
    for (val, id) in input.buses.iter() {
        let diff = (product / id) * modinverse(product / id, *id).unwrap();
        println!("diff, diff mod id: {}, {}", diff, diff % id);
        answer += -val * diff;
        answer %= product;
    }
    if answer >= 0 {
        answer
    } else {
        product + answer
    }
}
