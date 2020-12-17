#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Square {
    Floor,
    Seat,
    Occupied,
}

use std::convert::{TryFrom, TryInto};
use Square::*;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Layout {
    grid: Vec<Vec<Square>>,
    width: usize,
    height: usize,
}

impl std::fmt::Display for Layout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for sq in row {
                match sq {
                    Floor => write!(f, ".")?,
                    Seat => write!(f, "L")?,
                    Occupied => write!(f, "#")?,
                }
            }
            writeln!(f)?
        }
        Ok(())
    }
}

impl Layout {
    fn neighbor_counts(&self, row: i32, col: i32) -> (usize, usize, usize) {
        let mut ret = (0, 0, 0);
        let dirs = vec![
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        for (d_row, d_col) in dirs {
            let try_row = match usize::try_from(row + d_row) {
                Ok(v) => v,
                Err(_) => continue,
            };
            let try_col = match usize::try_from(col + d_col) {
                Ok(v) => v,
                Err(_) => continue,
            };
            if try_row < self.height && try_col < self.width {
                match self.grid[try_row][try_col] {
                    Floor => ret.0 += 1,
                    Seat => ret.1 += 1,
                    Occupied => ret.2 += 1,
                }
            }
        }
        ret
    }

    fn next_val(sq: &Square, nbs: (usize, usize, usize)) -> Square {
        if let Seat = sq {
            if nbs.2 == 0 {
                return Occupied;
            }
        }
        if let Occupied = sq {
            if nbs.2 >= 4 {
                return Seat;
            }
        }
        *sq
    }

    fn step(&self) -> Layout {
        let mut new_grid = vec![];
        for (row_num, row) in self.grid.iter().enumerate() {
            let mut new_row = vec![];
            for (col_num, sq) in row.iter().enumerate() {
                let nbs =
                    self.neighbor_counts(row_num.try_into().unwrap(), col_num.try_into().unwrap());
                let next_sq = Self::next_val(sq, nbs);
                new_row.push(next_sq);
            }
            new_grid.push(new_row);
        }
        Layout {
            width: self.width,
            height: self.height,
            grid: new_grid,
        }
    }
}

impl Layout {
    // (seat, occupied)
    fn neighbor_counts_part2(&self, row: i32, col: i32) -> (usize, usize) {
        let mut ret = (0, 0);
        let dirs = vec![
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        for (d_row, d_col) in dirs {
            for i in 1.. {
                let try_row = match usize::try_from(row + i * d_row) {
                    Ok(v) => v,
                    Err(_) => break,
                };
                let try_col = match usize::try_from(col + i * d_col) {
                    Ok(v) => v,
                    Err(_) => break,
                };
                if try_row < self.height && try_col < self.width {
                    match self.grid[try_row][try_col] {
                        Floor => continue,
                        Seat => {
                            ret.0 += 1;
                            break;
                        }
                        Occupied => {
                            ret.1 += 1;
                            break;
                        }
                    }
                } else {
                    break;
                }
            }
        }
        ret
    }

    fn next_val_part2(sq: &Square, nbs: (usize, usize)) -> Square {
        if let Seat = sq {
            if nbs.1 == 0 {
                return Occupied;
            }
        }
        if let Occupied = sq {
            if nbs.1 >= 5 {
                return Seat;
            }
        }
        *sq
    }

    fn step_part2(&self) -> Layout {
        let mut new_grid = vec![];
        for (row_num, row) in self.grid.iter().enumerate() {
            let mut new_row = vec![];
            for (col_num, sq) in row.iter().enumerate() {
                let nbs = self.neighbor_counts_part2(
                    row_num.try_into().unwrap(),
                    col_num.try_into().unwrap(),
                );
                let next_sq = Self::next_val_part2(sq, nbs);
                new_row.push(next_sq);
            }
            new_grid.push(new_row);
        }
        Layout {
            width: self.width,
            height: self.height,
            grid: new_grid,
        }
    }
}

#[aoc_generator(day11)]
pub fn parse(input: &str) -> Layout {
    //     let input = "L.LL.LL.LL
    // LLLLLLL.LL
    // L.L.L..L..
    // LLLL.LL.LL
    // L.LL.LL.LL
    // L.LLLLL.LL
    // ..L.L.....
    // LLLLLLLLLL
    // L.LLLLLL.L
    // L.LLLLL.LL";

    let mut grid: Vec<Vec<Square>> = vec![];
    for line in input.lines() {
        let row: Vec<Square> = line
            .chars()
            .map(|c| match c {
                'L' => Seat,
                '.' => Floor,
                '#' => Occupied,
                x => panic!("bad char {}", x),
            })
            .collect();
        grid.push(row);
    }
    let width = grid[0].len();
    let height = grid.len();
    Layout {
        grid,
        width,
        height,
    }
}

#[aoc(day11, part1)]
pub fn day1(input: &Layout) -> usize {
    let mut layout = input.clone();
    loop {
        let new_layout = layout.step();

        if layout == new_layout {
            break;
        }
        layout = new_layout;
    }
    layout
        .grid
        .into_iter()
        .map(|v| v.iter().filter(|&sq| *sq == Occupied).count())
        .sum()
}

#[aoc(day11, part2)]
pub fn day2(input: &Layout) -> usize {
    let mut layout = input.clone();
    loop {
        let new_layout = layout.step_part2();

        if layout == new_layout {
            break;
        }
        layout = new_layout;
    }
    layout
        .grid
        .into_iter()
        .map(|v| v.iter().filter(|&sq| *sq == Occupied).count())
        .sum()
}
