use std::collections::VecDeque;

#[aoc_generator(day9)]
pub fn gen(input: &str) -> Vec<i64> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}

#[aoc(day9, part1)]
pub fn day1(input: &[i64]) -> i64 {
    let mut buf: VecDeque<(i64, VecDeque<i64>)> = VecDeque::new();
    for i in 0..25 {
        let mut row = VecDeque::new();
        for j in (i + 1)..25 {
            row.push_back(input[i] + input[j]);
        }
        println!("{:?}", row);
        buf.push_back((input[i], row));
    }
    for val in &input[25..] {
        // search for val in previous
        if !search(&buf, val) {
            return *val;
        }
        buf.pop_front();
        for (old, row) in &mut buf {
            row.push_back(val + *old);
        }
        buf.push_back((*val, VecDeque::new()));
    }
    0
}

fn search(buf: &VecDeque<(i64, VecDeque<i64>)>, val: &i64) -> bool {
    for (_, row) in buf {
        for v in row {
            if *v == *val {
                return true;
            }
        }
    }
    false
}

/// O(n^2) alg: make one pass through input, computing all possible subsequence sums as we go
/// We maintain a list of all "live" sums, i.e. all sums that include the most recent element we've processed
/// to update this list, we add the next value to every element in it, plus adding the current elemetn
/// E.G for input [10, 9, 8, 7], we start with empty {}
/// Then we process 10 by adding 10 to every element (but there are none), plus inserting 10, so we get {10}
/// then we add 9 to each element, plus insert 9: {19, 9}
/// an so on. At step n, the list has n elements, so 1 + 2 + ... + n = O(n^2)
/// A slight modification is needed to track the min and max element appearng, but only slight.
#[aoc(day9, part2)]
pub fn day2(input: &[i64]) -> i64 {
    let day1_answer = day1(input);

    // (sum, (min in the sequence, max in the sequence))
    let mut live_sums: Vec<(i64, (i64, i64))> = vec![];
    let mut count = 0;
    for &next in input {
        let mut new_live_sums = Vec::with_capacity(live_sums.len());
        for (k, (min, max)) in live_sums {
            let new_min = min.min(next);
            let new_max = max.max(next);
            if next + k == day1_answer {
                return new_min + new_max;
            }
            new_live_sums.push((next + k, (new_min, new_max)));
        }
        new_live_sums.push((next, (next, next)));
        live_sums = new_live_sums;
        if count <= 10 {
            println!("{:?}", live_sums);
            count += 1;
        }
    }
    unreachable!()
}
