//! Ok, I hate problems like this because they're really tedious,
//! really cliche (this isn't even the first cellular automata this year!)
//! and just not that interesting.
//! I almost decided to just stop AoC for the year on this problem :/
//! In the end, I decided rather than trying to deal with arrays and bounds checking
//! I would just use a hash map instead, and track the "box" i.e. the min and max coords separately.
use std::collections::HashMap;
use std::convert::TryInto;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Square {
    Inactive,
    Active,
}

#[derive(Debug)]
pub struct Grid {
    grid: HashMap<(i32, i32, i32), Square>,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    min_z: i32,
    max_z: i32,
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for z in self.min_z..=self.max_z {
            writeln!(f, "z={}", z)?;
            for y in self.min_y..=self.max_y {
                for x in self.min_x..=self.max_x {
                    match self.get(x, y, z) {
                        Square::Inactive => write!(f, ".")?,
                        Square::Active => write!(f, "#")?,
                    }
                }
                writeln!(f)?;
            }
            writeln!(f, "\n")?;
        }
        Ok(())
    }
}

impl Grid {
    fn new() -> Self {
        Grid {
            grid: HashMap::new(),
            min_x: 0,
            max_x: 0,
            min_y: 0,
            max_y: 0,
            min_z: 0,
            max_z: 0,
        }
    }

    fn get(&self, x: i32, y: i32, z: i32) -> Square {
        self.grid
            .get(&(x, y, z))
            .cloned()
            .unwrap_or(Square::Inactive)
    }

    fn set(&mut self, x: i32, y: i32, z: i32, sq: Square) {
        self.grid.insert((x, y, z), sq);
    }

    fn active_neighbors(&self, x: i32, y: i32, z: i32) -> usize {
        let mut count = 0;
        for i in 0..27 {
            // these three range range over 0, 1, 2 using the power of base 3 representation
            let dx = i % 3;
            let dy = ((i - dx) / 3) % 3;
            let dz = ((i - dx - 3 * dy) / 9) % 3;

            // convert so they range over -1, 0, 0
            let dx = dx - 1;
            let dy = dy - 1;
            let dz = dz - 1;

            // skip ourselves
            if dx == 0 && dy == 0 && dz == 0 {
                continue;
            }
            if let Square::Active = self.get(x + dx, y + dy, z + dz) {
                count += 1;
            }
        }
        count
    }

    fn next_grid(&self) -> Grid {
        let mut next = Grid::new();
        for x in self.min_x - 1..self.max_x + 2 {
            for y in self.min_y - 1..self.max_y + 2 {
                for z in self.min_z - 1..self.max_z + 2 {
                    let count = self.active_neighbors(x, y, z);
                    let current = self.get(x, y, z);
                    if (current == Square::Active && (count == 3 || count == 2))
                        || (current == Square::Inactive && count == 3)
                    {
                        next.set(x, y, z, Square::Active)
                    } else {
                        next.set(x, y, z, Square::Inactive);
                    }
                }
            }
        }
        next.min_x = self.min_x - 1;
        next.max_x = self.max_x + 1;
        next.min_y = self.min_y - 1;
        next.max_y = self.max_y + 1;
        next.min_z = self.min_z - 1;
        next.max_z = self.max_z + 1;
        next
    }
}

pub fn parse(input: &str) -> Grid {
    let mut grid = Grid::new();
    for (y, line) in input.lines().enumerate() {
        grid.max_x = (line.len() - 1).try_into().unwrap();
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => grid.set(
                    x.try_into().unwrap(),
                    y.try_into().unwrap(),
                    0,
                    Square::Active,
                ),
                '.' => grid.set(
                    x.try_into().unwrap(),
                    y.try_into().unwrap(),
                    0,
                    Square::Inactive,
                ),
                x => panic!("bad input char {}", x),
            }
        }
    }
    grid.max_y = (input.lines().count() - 1).try_into().unwrap();
    grid
}

#[aoc(day17, part1)]
pub fn day17(input: &str) -> usize {
    // example
    //     let input = r".#.
    // ..#
    // ###";
    let mut grid = parse(input);
    println!("{}", grid);

    for _ in 0..6 {
        grid = grid.next_grid();
    }
    grid.grid.values().filter(|x| **x == Square::Active).count()
}

// Ok, and again, with the power of copy paste!
// Trying to make this all parametric over the number of dimensions is just more effor than I
// I feel like putting into this annoying problem.

#[derive(Debug)]
pub struct Grid4D {
    grid: HashMap<(i32, i32, i32, i32), Square>,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    min_z: i32,
    max_z: i32,
    min_w: i32,
    max_w: i32,
}

impl std::fmt::Display for Grid4D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for w in self.min_w..=self.max_w {
            for z in self.min_z..=self.max_z {
                writeln!(f, "z={}, w={}", z, w)?;
                for y in self.min_y..=self.max_y {
                    for x in self.min_x..=self.max_x {
                        match self.get(x, y, z, w) {
                            Square::Inactive => write!(f, ".")?,
                            Square::Active => write!(f, "#")?,
                        }
                    }
                    writeln!(f)?;
                }
                writeln!(f, "\n")?;
            }
            writeln!(f, "\n")?;
        }
        Ok(())
    }
}

impl Grid4D {
    fn new() -> Self {
        Grid4D {
            grid: HashMap::new(),
            min_x: 0,
            max_x: 0,
            min_y: 0,
            max_y: 0,
            min_z: 0,
            max_z: 0,
            min_w: 0,
            max_w: 0,
        }
    }

    fn get(&self, x: i32, y: i32, z: i32, w: i32) -> Square {
        self.grid
            .get(&(x, y, z, w))
            .cloned()
            .unwrap_or(Square::Inactive)
    }

    fn set(&mut self, x: i32, y: i32, z: i32, w: i32, sq: Square) {
        self.grid.insert((x, y, z, w), sq);
    }

    fn active_neighbors(&self, x: i32, y: i32, z: i32, w: i32) -> usize {
        let mut count = 0;
        for i in 0..81 {
            // these three range range over 0, 1, 2 using the power of base 3 representation
            let dx = i % 3;
            let dy = ((i - dx) / 3) % 3;
            let dz = ((i - dx - 3 * dy) / 9) % 3;
            let dw = ((i - dx - 3 * dy - 9 * dz) / 27) % 3;

            // convert so they range over -1, 0, 0
            let dx = dx - 1;
            let dy = dy - 1;
            let dz = dz - 1;
            let dw = dw - 1;

            // skip ourselves
            if dx == 0 && dy == 0 && dz == 0 && dw == 0 {
                continue;
            }
            if let Square::Active = self.get(x + dx, y + dy, z + dz, w + dw) {
                count += 1;
            }
        }
        count
    }

    fn next_grid(&self) -> Grid4D {
        let mut next = Grid4D::new();
        for x in self.min_x - 1..self.max_x + 2 {
            for y in self.min_y - 1..self.max_y + 2 {
                for z in self.min_z - 1..self.max_z + 2 {
                    for w in self.min_w - 1..self.max_w + 2 {
                        let count = self.active_neighbors(x, y, z, w);
                        let current = self.get(x, y, z, w);
                        if (current == Square::Active && (count == 3 || count == 2))
                            || (current == Square::Inactive && count == 3)
                        {
                            next.set(x, y, z, w, Square::Active)
                        } else {
                            next.set(x, y, z, w, Square::Inactive);
                        }
                    }
                }
            }
        }
        next.min_x = self.min_x - 1;
        next.max_x = self.max_x + 1;
        next.min_y = self.min_y - 1;
        next.max_y = self.max_y + 1;
        next.min_z = self.min_z - 1;
        next.max_z = self.max_z + 1;
        next.min_w = self.min_w - 1;
        next.max_w = self.max_w + 1;
        next
    }
}

pub fn parse4d(input: &str) -> Grid4D {
    let mut grid = Grid4D::new();
    for (y, line) in input.lines().enumerate() {
        grid.max_x = (line.len() - 1).try_into().unwrap();
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => grid.set(
                    x.try_into().unwrap(),
                    y.try_into().unwrap(),
                    0,
                    0,
                    Square::Active,
                ),
                '.' => grid.set(
                    x.try_into().unwrap(),
                    y.try_into().unwrap(),
                    0,
                    0,
                    Square::Inactive,
                ),
                x => panic!("bad input char {}", x),
            }
        }
    }
    grid.max_y = (input.lines().count() - 1).try_into().unwrap();
    grid
}

#[aoc(day17, part2)]
pub fn day17_2(input: &str) -> usize {
    // example:
    // let input = ".#.\n..#\n###";
    let mut grid = parse4d(input);
    println!("{}", grid);

    for i in 0..6 {
        grid = grid.next_grid();
        if i == 0 {
            println!("{}", grid);
        }
    }
    grid.grid.values().filter(|x| **x == Square::Active).count()
}
