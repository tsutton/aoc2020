use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Instr {
    F(i32),

    L(i32),
    R(i32),

    N(i32),
    E(i32),
    S(i32),
    W(i32),
}

impl TryFrom<&str> for Instr {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() < 2 {
            return Err("too short".into());
        }
        let body: i32 = match value[1..].parse() {
            Ok(v) => v,
            Err(_) => return Err("bad int format".into()),
        };
        match value.chars().next().unwrap() {
            'F' => Ok(Instr::F(body)),

            'L' => Ok(Instr::L(body)),
            'R' => Ok(Instr::R(body)),

            'N' => Ok(Instr::N(body)),
            'E' => Ok(Instr::E(body)),
            'S' => Ok(Instr::S(body)),
            'W' => Ok(Instr::W(body)),

            x => Err(format!("bad letter {}", x).into()),
        }
    }
}

#[aoc_generator(day12)]
pub fn parse(input: &str) -> Vec<Instr> {
    input.lines().map(|line| line.try_into().unwrap()).collect()
}

/// We use standard coordinates with North = Up = positive y direction
/// Heading is a unit vector in the direction we're facing i.e. east is (1, 0)
#[derive(Debug)]
struct State {
    x: i32,
    y: i32,
    heading: (i32, i32),
}

impl State {
    fn step(&mut self, i: &Instr) {
        match i {
            Instr::F(steps) => {
                self.x += self.heading.0 * steps;
                self.y += self.heading.1 * steps;
            }

            Instr::N(steps) => self.y += steps,
            Instr::E(steps) => self.x += steps,
            Instr::S(steps) => self.y -= steps,
            Instr::W(steps) => self.x -= steps,

            Instr::L(deg) => match *deg {
                90 => self.heading = (-self.heading.1, self.heading.0),
                180 => self.heading = (-self.heading.0, -self.heading.1),
                270 => self.heading = (self.heading.1, -self.heading.0),
                x => panic!("unsupported rotation {}", x),
            },
            Instr::R(deg) => match *deg {
                90 => self.heading = (self.heading.1, -self.heading.0),
                180 => self.heading = (-self.heading.0, -self.heading.1),
                270 => self.heading = (-self.heading.1, self.heading.0),
                x => panic!("unsupported rotation {}", x),
            },
        }
    }
}

#[aoc(day12, part1)]
pub fn part1(input: &[Instr]) -> i32 {
    let mut state = State {
        x: 0,
        y: 0,
        heading: (1, 0),
    };
    let mut debug_count = 0;
    for i in input {
        state.step(i);
        if debug_count <= 10 {
            println!("stepped by {:?}", i);
            println!("state: {:?}", state);
            debug_count += 1;
        }
    }
    state.x.abs() + state.y.abs()
}

/// We use standard coordinates with North = Up = positive y direction
#[derive(Debug)]
struct State2 {
    x: i32,
    y: i32,
    waypoint: (i32, i32),
}

impl State2 {
    fn step(&mut self, i: &Instr) {
        match i {
            Instr::F(steps) => {
                self.x += self.waypoint.0 * steps;
                self.y += self.waypoint.1 * steps;
            }

            Instr::N(steps) => self.waypoint.1 += steps,
            Instr::E(steps) => self.waypoint.0 += steps,
            Instr::S(steps) => self.waypoint.1 -= steps,
            Instr::W(steps) => self.waypoint.0 -= steps,

            Instr::L(deg) => match *deg {
                90 => self.waypoint = (-self.waypoint.1, self.waypoint.0),
                180 => self.waypoint = (-self.waypoint.0, -self.waypoint.1),
                270 => self.waypoint = (self.waypoint.1, -self.waypoint.0),
                x => panic!("unsupported rotation {}", x),
            },
            Instr::R(deg) => match *deg {
                90 => self.waypoint = (self.waypoint.1, -self.waypoint.0),
                180 => self.waypoint = (-self.waypoint.0, -self.waypoint.1),
                270 => self.waypoint = (-self.waypoint.1, self.waypoint.0),
                x => panic!("unsupported rotation {}", x),
            },
        }
    }
}

#[aoc(day12, part2)]
pub fn part2(input: &[Instr]) -> i32 {
    let mut state = State2 {
        x: 0,
        y: 0,
        waypoint: (10, 1),
    };
    let mut debug_count = 0;
    for i in input {
        state.step(i);
        if debug_count <= 10 {
            println!("stepped by {:?}", i);
            println!("state: {:?}", state);
            debug_count += 1;
        }
    }
    state.x.abs() + state.y.abs()
}
