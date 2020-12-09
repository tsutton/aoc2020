#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Instruction {
    Jmp(i32),
    Nop(i32),
    Acc(i32),
}

use std::collections::HashSet;
use std::convert::TryFrom;

use Instruction::*;

impl Instruction {
    fn from_str(input: &str) -> Option<Instruction> {
        if input.len() < 6 {
            return None;
        }
        let body = input[4..].parse();
        let body = match body {
            Ok(v) => v,
            Err(_) => return None,
        };
        match &input[..3] {
            "jmp" => Some(Instruction::Jmp(body)),
            "acc" => Some(Instruction::Acc(body)),
            "nop" => Some(Instruction::Nop(body)),
            _ => None,
        }
    }
}

pub struct Execution {
    program: Vec<Instruction>,
    pc: i32,
    acc: i32,
}

impl Execution {
    fn step(&mut self) {
        let instr = self.program[usize::try_from(self.pc).unwrap()];
        match instr {
            Jmp(arg) => self.pc = self.pc + arg,
            Acc(arg) => {
                self.pc += 1;
                self.acc += arg;
            }
            Nop(_) => self.pc += 1,
        }
    }
}

#[aoc_generator(day8)]
pub fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|x| Instruction::from_str(x).unwrap())
        .collect()
}

#[aoc(day8, part1)]
pub fn day1(input: &Vec<Instruction>) -> i32 {
    let mut execution = Execution {
        acc: 0,
        pc: 0,
        program: input.clone(),
    };
    execution.run_to_completion().unwrap_err()
}

impl Execution {
    pub fn run_to_completion(&mut self) -> Result<i32, i32> {
        let mut visited_pcs: HashSet<i32> = HashSet::new();
        loop {
            if visited_pcs.contains(&self.pc) {
                return Err(self.acc);
            } else if self.pc == i32::try_from(self.program.len()).unwrap() {
                return Ok(self.acc);
            }
            visited_pcs.insert(self.pc);
            self.step()
        }
    }
}
#[aoc(day8, part2)]
pub fn day2(input: &Vec<Instruction>) -> i32 {
    for (i, instr) in input.iter().enumerate() {
        let mut exec = match instr {
            Jmp(v) => {
                let mut program = input.clone();
                program[i] = Nop(*v);
                Execution {
                    program,
                    pc: 0,
                    acc: 0,
                }
            }
            Nop(v) => {
                let mut program = input.clone();
                program[i] = Jmp(*v);
                Execution {
                    program,
                    pc: 0,
                    acc: 0,
                }
            }
            _ => continue,
        };
        if let Ok(ans) = exec.run_to_completion() {
            return ans;
        }
    }
    panic!()
}
