#[aoc(day25, part1)]
pub fn part1(input: &str) -> i64 {
    let mut lines = input.lines();
    let door_public_key: i64 = lines.next().unwrap().parse().unwrap();
    let card_public_key: i64 = lines.next().unwrap().parse().unwrap();

    let subject: i64 = 7;
    let modulus: i64 = 20201227;
    let card_loop_size = brute_force_loop_size(subject, modulus, card_public_key);
    encrypt(door_public_key, modulus, card_loop_size)
}

fn brute_force_loop_size(subject: i64, modulus: i64, public_key: i64) -> i64 {
    let mut value = 1;
    for i in 1.. {
        value *= subject;
        value %= modulus;
        if value == public_key {
            return i;
        }
    }
    unreachable!()
}

fn encrypt(subject: i64, modulus: i64, loop_size: i64) -> i64 {
    let mut value = 1;
    for _ in 0..loop_size {
        value *= subject;
        value %= modulus;
    }
    value
}
