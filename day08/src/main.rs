//! --- Day 8: Handheld Halting ---
//! https://adventofcode.com/2020/day/8

use std::{collections::HashSet, error::Error, fmt, num::ParseIntError, str::FromStr};

const INPUT: &str = include_str!("../../inputs/day08.txt");

fn main() {
    println!("Part one: {}", part_one(INPUT));
    println!("Part two: {}", part_two(INPUT));
}

fn part_one(input: &str) -> i64 {
    let mut acc = 0;
    let mut seen = HashSet::new();
    let instructions = input
        .lines()
        .map(|s| Instruction::from_str(s).unwrap())
        .collect::<Vec<Instruction>>();

    let mut i = 0;
    while i < instructions.len() {
        if seen.contains(&i) {
            break;
        }
        seen.insert(i);

        match instructions[i] {
            Instruction::Nop(_) => (),
            Instruction::Acc(n) => {
                acc += n;
            }
            Instruction::Jmp(n) => {
                i = (i as i64 + n) as usize;
                continue;
            }
        }

        i += 1;
    }

    acc
}

fn part_two(input: &str) -> i64 {
    let mut acc = 0;
    let mut seen = HashSet::new();
    let instructions = input
        .lines()
        .map(|s| Instruction::from_str(s).unwrap())
        .collect::<Vec<Instruction>>();

    let mut i = 0;
    while i < instructions.len() {
        let mut inst = instructions[i].clone();
        if seen.contains(&i) {
            inst = match inst {
                Instruction::Nop(n) => Instruction::Jmp(n),
                Instruction::Jmp(n) => Instruction::Nop(n),
                _ => panic!("Hmmm"),
            }
        }
        seen.insert(i);

        match inst {
            Instruction::Nop(_) => (),
            Instruction::Acc(n) => {
                acc += n;
            }
            Instruction::Jmp(n) => {
                i = (i as i64 + n) as usize;
                continue;
            }
        }
    }

    acc
}

#[derive(Debug, Clone)]
enum Instruction {
    Nop(i64),
    Acc(i64),
    Jmp(i64),
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim().split(' ');

        let ops = parts.next().to_owned();
        match ops {
            Some(ops) => {
                let num = parts.next().unwrap().parse::<i64>()?;
                match ops {
                    "nop" => Ok(Instruction::Nop(num)),
                    "acc" => Ok(Instruction::Acc(num)),
                    "jmp" => Ok(Instruction::Jmp(num)),
                    _ => Err(ParseInstructionError::Other(
                        "Failed to parse operation".into(),
                    )),
                }
            }
            None => Err(ParseInstructionError::Other(
                "Failed to parse operation".into(),
            )),
        }
    }
}

#[derive(Debug)]
enum ParseInstructionError {
    Other(String),
    WrappedErr(Box<dyn Error>),
}

impl From<ParseIntError> for ParseInstructionError {
    fn from(error: ParseIntError) -> Self {
        ParseInstructionError::WrappedErr(Box::new(error))
    }
}

impl fmt::Display for ParseInstructionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ParseInstructionError::*;

        match self {
            Other(s) => write!(f, "Parse Instuction Error: {}", s),
            WrappedErr(e) => write!(f, "Parse Instuction Error: {}", e),
        }
    }
}

impl Error for ParseInstructionError {}

#[cfg(test)]
mod test {
    use crate::*;

    const INPUT: &str = "\
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn part_one_works() {
        let res = part_one(INPUT);
        assert_eq!(5, res);
    }

    #[test]
    fn part_two_works() {
        let res = part_two(INPUT);
        assert_eq!(8, res);
    }
}
