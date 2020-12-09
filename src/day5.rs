use std::convert::TryInto;

#[aoc_generator(day5)]
pub fn pt1(input: &str) -> Vec<(i64, i64)> {
    input.lines().map(one_line).collect()
}

fn one_line(line: &str) -> (i64, i64) {
    let row = char_binary_search(&line[0..7], 'F', 'B');
    let col = char_binary_search(&line[7..10], 'L', 'R');
    (row, col)
}

fn char_binary_search(input: &str, left: char, right: char) -> i64 {
    let mut low = 0;
    let mut high = 2i64.pow(input.len().try_into().unwrap()) - 1;
    for c in input.chars() {
        if c == left {
            high -= (high - low + 1) / 2;
        } else if c == right {
            low += (high - low + 1) / 2;
        } else {
            panic!()
        }
    }
    assert_eq!(low, high);
    low
}
#[aoc(day5, part1)]
pub fn pt1_go(input: &[(i64, i64)]) -> i64 {
    input.iter().map(|(x, y)| 8 * x + y).max().unwrap()
}

#[aoc(day5, part2)]
pub fn pt2_go(input: &Vec<(i64, i64)>) -> i64 {
    let mut ids: Vec<_> = input.iter().map(|(x, y)| 8 * x + y).collect::<Vec<_>>();
    ids.sort_unstable();
    // println!("{:?}", ids);
    for (i, id) in ids.iter().enumerate().skip(1) {
        if ids[i - 1] == id - 2 {
            return *id - 1;
        }
    }
    unreachable!()
}
