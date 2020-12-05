use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

struct BoardingPass {
    row: usize,
    column: usize,
}

impl BoardingPass {
    fn parse(pass: &str) -> Self {
        let pass_binary = pass
            .replace('L', "0")
            .replace('R', "1")
            .replace('F', "0")
            .replace('B', "1");
        let (row, column) = pass_binary.split_at(7);
        Self {
            row: usize::from_str_radix(row, 2).expect("Unusual column"),
            column: usize::from_str_radix(column, 2).expect("Unusual column"),
        }
    }

    fn get_id(&self) -> usize {
        (self.row * 8) + self.column
    }
}

#[aoc_generator(day5)]
fn parse_inputs(input: &str) -> Vec<BoardingPass> {
    input.lines().map(BoardingPass::parse).collect()
}

#[aoc(day5, part1)]
fn solve_part1(input: &Vec<BoardingPass>) -> usize {
    input.iter().map(BoardingPass::get_id).max().unwrap()
}

#[aoc(day5, part2)]
fn solve_part2(input: &Vec<BoardingPass>) -> usize {
    input
        .iter()
        .map(BoardingPass::get_id)
        .sorted()
        .tuple_windows()
        .find_map(|(last, current)| Some(last + 1).filter(|_| current != last + 1))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity() {
        let input = "\
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL";
        let input = parse_inputs(input);

        assert_eq!(solve_part1(&input), 820);
        assert_eq!(solve_part2(&input), 120);
    }

    #[test]
    fn solve() {
        let input = include_str!("../input/2020/day5.txt");
        let input = parse_inputs(input);
        assert_eq!(solve_part2(&input), 678);
    }
}
