use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day5)]
fn parse(pass: &str) -> Vec<usize> {
    pass.lines()
        .map(|seat| {
            seat.chars()
                .map(|c| match c {
                    'L' | 'F' => '0',
                    'R' | 'B' => '1',
                    _ => c,
                })
                .collect::<String>()
        })
        .map(|bin| usize::from_str_radix(bin.as_str(), 2).expect("Invalid ID"))
        .collect()
}

#[aoc(day5, part1)]
fn solve_part1(input: &[usize]) -> usize {
    *input.iter().max().unwrap()
}

#[aoc(day5, part2)]
fn solve_part2(input: &[usize]) -> usize {
    input
        .iter()
        .sorted()
        .tuple_windows()
        .find_map(|(last, current)| Some(last + 1).filter(|_| *current != last + 1))
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
        let input = parse(input);

        assert_eq!(solve_part1(&input), 820);
        assert_eq!(solve_part2(&input), 120);
    }

    #[test]
    fn solve() {
        let input = include_str!("../input/2020/day5.txt");
        let input = parse(input);
        assert_eq!(solve_part1(&input), 826);
        assert_eq!(solve_part2(&input), 678);
    }
}
