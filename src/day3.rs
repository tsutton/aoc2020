#[derive(Debug, Copy, Clone)]
pub enum Square {
    Open,
    Tree,
}

impl std::fmt::Display for Square {
    fn fmt(&self, f: &'_ mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Square::Open => write!(f, "."),
            Square::Tree => write!(f, "#"),
        }
    }
}

#[derive(Debug)]
pub struct Grid {
    width: usize,
    height: usize,
    grid: Vec<Vec<Square>>,
}

#[aoc_generator(day3)]
pub fn gen(input: &str) -> Grid {
    let mut ret: Vec<Vec<Square>> = vec![];
    for line in input.lines() {
        ret.push(
            line.chars()
                .map(|x| match x {
                    '.' => Square::Open,
                    '#' => Square::Tree,
                    _ => panic!("bad square"),
                })
                .collect(),
        );
    }
    Grid {
        width: ret[0].len(),
        height: ret.len(),
        grid: ret,
    }
}

#[aoc(day3, part1)]
pub fn solve_1(input: &Grid) -> i32 {
    let mut ans = 0;
    for (i, row) in input.grid.iter().enumerate() {
        let col = (3 * i) % input.width;
        let square = row[col];
        if let Square::Tree = square {
            ans += 1;
        }
    }
    ans
}

fn one_slope(input: &Grid, dx: usize, dy: usize) -> i64 {
    let mut ans = 0;
    let mut row = 0;
    let mut col = 0;
    while row < input.grid.len() {
        let square = input.grid[row][col];
        if let Square::Tree = square {
            ans += 1;
        }
        row += dy;
        col = (col + dx) % input.width;
    }
    ans
}

#[aoc(day3, part2)]
pub fn solve_2(input: &Grid) -> i64 {
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut ans = 1;
    for (dx, dy) in slopes {
        let trees = one_slope(input, dx, dy);
        println!("for ({}, {}) got {}", dx, dy, trees);
        ans *= trees;
    }
    ans
}
