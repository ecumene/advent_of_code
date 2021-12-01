use aoc_runner_derive::{aoc, aoc_generator};
use serde::Deserialize;
use std::fmt;

type Argument = i64;

#[derive(Deserialize, Clone, Copy, PartialEq, Debug)]
enum Operation {
    #[serde(rename(deserialize = "acc"))]
    Acc,
    #[serde(rename(deserialize = "jmp"))]
    Jmp,
    #[serde(rename(deserialize = "nop"))]
    Nop,
}

#[derive(Deserialize, Clone, Copy, Debug)]
struct Instruction {
    op: Operation,
    arg: Argument,
}

impl Instruction {
    fn parse(instruction: &str) -> Instruction {
        serde_scan::from_str(
            instruction
                .chars()
                .filter(|x| x != &'+')
                .collect::<String>()
                .as_str(),
        )
        .unwrap()
    }

    fn uncorrupt(mut self) -> Self {
        match self.op {
            Operation::Jmp => self.op = Operation::Nop,
            Operation::Nop => self.op = Operation::Jmp,
            _ => {}
        }
        self
    }
}

#[derive(Debug, PartialEq)]
enum ExitType {
    LoopDetected(Argument),
    Finished(Argument),
}

impl fmt::Display for ExitType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            ExitType::LoopDetected(a) => write!(f, "LoopDetected({})", a),
            ExitType::Finished(a) => write!(f, "Finished({})", a),
        }
    }
}

#[aoc_generator(day8)]
fn parse_passwords<'a>(input: &'a str) -> Vec<Instruction> {
    input.lines().map(Instruction::parse).collect()
}

#[aoc(day8, part1)]
fn solve_part1(input: &[Instruction]) -> ExitType {
    let mut i = 0;
    let mut acc = 0;
    let mut seen = vec![false; input.len()];
    loop {
        let instruction = &input[i];
        if seen[i] {
            return ExitType::LoopDetected(acc);
        } else {
            seen[i] = true;
            match instruction.op {
                Operation::Acc => {
                    acc += instruction.arg;
                    i += 1;
                }
                Operation::Nop => {
                    i += 1;
                }
                Operation::Jmp => {
                    i = (i as Argument + instruction.arg) as usize;
                }
            }
            if i >= input.len() {
                return ExitType::Finished(acc);
            }
        }
    }
}

#[aoc(day8, part2)]
fn solve_part2(input: &[Instruction]) -> Argument {
    (0..input.len())
        .find_map(|i| {
            let mut patched = input.clone().to_owned();
            if patched[i].op == Operation::Acc {
                return None;
            }
            patched[i] = patched[i].uncorrupt();
            match solve_part1(patched.as_slice()) {
                ExitType::LoopDetected(_) => None,
                ExitType::Finished(a) => Some(a),
            }
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve() {
        let input = include_str!("../input/2020/day8.txt");
        let input = parse_passwords(input);

        assert_eq!(solve_part1(&input), ExitType::LoopDetected(1859));
        assert_eq!(solve_part2(&input), 1235);
    }
}
