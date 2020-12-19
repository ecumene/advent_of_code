use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::ops::RangeInclusive;
use std::{num::ParseIntError, str::FromStr};

type TicketValue = u32;

#[derive(Debug)]
pub struct Rule {
    name: String,
    ranges: [RangeInclusive<TicketValue>; 2],
}

impl Rule {
    pub fn in_range(&self, value: TicketValue) -> bool {
        self.ranges.iter().any(|rule| rule.contains(&value))
    }
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (name, from1, to1, from2, to2) =
            serde_scan::scan!("{}: {}-{} or {}-{}" <- input).unwrap();

        Ok(Self {
            name,
            ranges: [from1..=to1, from2..=to2],
        })
    }
}

type Ticket = Vec<TicketValue>;

#[derive(Debug)]
pub struct PuzzleInput {
    rules: Vec<Rule>,
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

impl FromStr for PuzzleInput {
    type Err = ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let secs = input.split("\n\n").collect_vec();
        let nearby_tickets = secs[2]
            .lines()
            .skip(1)
            .map(|l| l.split(',').map(str::parse).collect())
            .collect::<Result<Vec<Vec<u32>>, _>>()?;
        Ok(Self {
            rules: secs[0]
                .split('\n')
                .map(str::parse)
                .map(|x| x.unwrap())
                .collect(),
            my_ticket: secs[1]
                .lines()
                .nth(1)
                .unwrap()
                .split(',')
                .map(str::parse)
                .map(|x| x.unwrap())
                .collect(),
            nearby_tickets,
        })
    }
}

#[aoc_generator(day16)]
fn parse_input(input: &str) -> PuzzleInput {
    input.parse().expect("Bad input")
}

// Stole tocklime's optimized part1/part2 https://github.com/tocklime

#[aoc(day16, part1)]
fn solve_part1(input: &PuzzleInput) -> u32 {
    input
        .nearby_tickets
        .iter()
        .flatten()
        .filter(|&&v| input.rules.iter().all(|r| !r.in_range(v)))
        .sum()
}

fn get_set_bit(n: u32) -> Option<u32> {
    (0..=32).find(|x| n & (1 << x) != 0)
}

#[aoc(day16, part2)]
fn solve_part2(input: &PuzzleInput) -> u64 {
    let valid_nearbys = input
        .nearby_tickets
        .iter()
        .filter(|t| t.iter().all(|&v| input.rules.iter().any(|r| r.in_range(v))))
        .collect_vec();
    let field_count = input.rules.len();
    assert!(field_count <= 32);
    //theres <32 fields, so lets use a u32 as a bitfield of possibilities..
    //Phase 1: create possibilities based on all field values being valid.
    let mut possible: Vec<u32> = input
        .rules
        .iter()
        .map(|r| {
            let mut i = 0;
            for ix in 0..field_count {
                if valid_nearbys.iter().all(|x| r.in_range(x[ix])) {
                    i |= 1 << ix;
                }
            }
            i
        })
        .collect();
    //Phase 2: eliminate possibilities where another field must have a given index.
    let mut ans = vec![0; field_count];
    while let Some(&x) = possible.iter().find(|&p| p.count_ones() == 1) {
        for (ix, p) in possible.iter_mut().enumerate() {
            if *p == x {
                ans[ix] = get_set_bit(x).unwrap();
            }
            *p &= !x;
        }
    }
    ans[0..6]
        .iter()
        .map(|&v| u64::from(input.my_ticket[v as usize]))
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity() {
        let input = &parse_input(
            "\
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12",
        );

        assert_eq!(solve_part1(input), 71);
    }

    #[test]
    fn solve() {
        let input = include_str!("../input/2020/day16.txt");
        let input = &parse_input(input);

        assert_eq!(solve_part1(input), 26869);
        assert_eq!(solve_part2(input), 855275529001);
    }
}
