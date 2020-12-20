use std::collections::HashSet;
use std::io::{self, Read};
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

fn main() -> Result<(), io::Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let program = parse(&input);

    println!("part 1: {:?}", part1(&program));

    Ok(())
}

#[derive(Default)]
struct Device {
    acc: i32,
    pc: usize,
}

impl Device {
    fn reset(&mut self) {
        self.acc = 0;
        self.pc = 0;
    }

    // Returns the value of the accumulator, immediately before any instruction
    // is executed a second time. If we go out of bounds, return None. Also, you
    // probably want to run .reset() first.
    fn run_until_loop(&mut self, program: &[Instruction]) -> Option<i32> {
        let mut seen = HashSet::new();
        loop {
            match seen.insert(self.pc) {
                true => (),
                false => return Some(self.acc),
            }
            match program.get(self.pc)? {
                Instruction::Nop(_) => self.pc += 1,
                Instruction::Acc(v) => {
                    self.acc += *v;
                    self.pc += 1
                }
                Instruction::Jmp(v) => self.pc += *v as usize,
            }
        }
    }
}

enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref INSTRUCTION_RE: Regex =
                Regex::new(r"^(?P<op>[a-z]{3}) (?P<arg>[+\-]\d+)$").unwrap();
        }
        let caps = INSTRUCTION_RE
            .captures(s)
            .ok_or("unable to find instruction")?;
        let op = caps.name("op").ok_or("unable to find operation")?.as_str();
        let arg = caps
            .name("arg")
            .ok_or("unable to find argument")?
            .as_str()
            .parse::<i32>()
            .map_err(|e| e.to_string())?;
        match op {
            "nop" => Ok(Instruction::Nop(arg)),
            "acc" => Ok(Instruction::Acc(arg)),
            "jmp" => Ok(Instruction::Jmp(arg)),
            _ => Err(format!("unknown operation {}", op)),
        }
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    input.lines().filter_map(|x| x.parse().ok()).collect()
}

fn part1(program: &[Instruction]) -> Option<i32> {
    let mut device = Device {
        ..Default::default()
    };
    device.run_until_loop(program)
}
