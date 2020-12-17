use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;

type Value = u64;

#[derive(Debug, Default)]
struct Mask {
    mask: Value,
    write: Value,
    floating: Vec<[Value; 2]>,
}

impl Mask {
    fn apply(&self, value: Value) -> Value {
        value & self.mask | self.write
    }

    fn get_addresses(&self, addr: Value) -> impl Iterator<Item = u64> + '_ {
        let addr = (addr | self.write) & !self.mask;
        self.floating
            .iter()
            .multi_cartesian_product()
            .map(move |p| p.iter().fold(addr, |v, f| v | *f))
    }
}

impl FromStr for Mask {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut mask = Mask::default();
        for (i, c) in s.chars().rev().enumerate() {
            match c {
                '0' => {}
                '1' => mask.write |= 1 << i,
                'X' => {
                    mask.mask |= 1 << i;
                    mask.floating.push([0, 1 << i]);
                }
                c => panic!("Unexpected character {}", c),
            }
        }
        Ok(mask)
    }
}

struct Instruction {
    address: Value,
    value: Value,
}

impl FromStr for Instruction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (address, value): (Value, Value) = serde_scan::scan!("mem[{}] = {}" <- s).unwrap();

        Ok(Instruction { address, value })
    }
}

struct Program {
    mask: Mask,
    instructions: Vec<Instruction>,
}

#[aoc_generator(day14)]
fn parse_input(input: &str) -> Vec<Program> {
    let mut mask: Option<Mask> = None;
    let mut programs = vec![];
    let mut instructions = vec![];
    for line in input.lines() {
        if line.starts_with("mask") {
            if mask.is_some() {
                programs.push(Program {
                    mask: mask.take().unwrap(),
                    instructions,
                });
            }
            mask = Some(Mask::from_str(&line[7..]).expect("Invalid Mask"));
            instructions = vec![];
        } else {
            instructions.push(Instruction::from_str(line).expect("Invalid Instruction"));
        }
    }
    programs.push(Program {
        mask: mask.take().unwrap(),
        instructions,
    });
    programs
}

#[aoc(day14, part1)]
fn solve_part1(input: &[Program]) -> Value {
    let mut memory = HashMap::new();

    for program in input {
        for instruction in &program.instructions {
            memory.insert(instruction.address, program.mask.apply(instruction.value));
        }
    }

    memory.values().sum()
}

#[aoc(day14, part2)]
fn solve_part2(input: &[Program]) -> Value {
    let mut memory = HashMap::new();

    for program in input {
        for instruction in &program.instructions {
            for address in program.mask.get_addresses(instruction.address) {
                memory.insert(address, instruction.value);
            }
        }
    }

    memory.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity() {
        let input = "\
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0
";
        let input = &parse_input(input);
        assert_eq!(solve_part1(input), 165);

        let input = "\
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0
";
        let input = &parse_input(input);
        assert_eq!(solve_part1(input), 165);

        let input = "\
mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1
";
        let input = &parse_input(input);
        assert_eq!(solve_part2(input), 208);
    }

    #[test]
    fn solve() {
        let input = include_str!("../input/2020/day14.txt");
        let input = parse_input(input);

        assert_eq!(solve_part1(&input), 8570568288597);
        assert_eq!(solve_part2(&input), 3289441921203);
    }
}
