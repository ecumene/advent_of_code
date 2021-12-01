use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use itertools::MinMaxResult::MinMax;

#[aoc_generator(day9)]
fn parse_input(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|line| line.parse().expect("That's not a number..."))
        .collect::<Vec<i64>>()
}

fn find_invalid_number(input: &[i64], preamble_size: usize) -> i64 {
    input
        .windows(preamble_size + 1)
        .find_map(|slice| {
            if let Some(_) = slice
                .iter()
                .tuple_combinations()
                .find(|(a, b)| *a + *b == slice[preamble_size])
            {
                None
            } else {
                Some(slice[preamble_size])
            }
        })
        .expect("All have at least 1 complimentary pair")
}

fn find_sum_of_invalid_number(input: &[i64], preamble_size: usize) -> i64 {
    let preamble = find_invalid_number(input, preamble_size);
    match (2..preamble_size)
        .find_map(|window_size| {
            input
                .windows(window_size)
                .find(|items| items.iter().sum::<i64>() == preamble)
        })
        .unwrap()
        .iter()
        .minmax()
    {
        MinMax(x, y) => x + y,
        _ => panic!("No contiguous series!"),
    }
}

#[aoc(day9, part1)]
fn solve_part1(input: &[i64]) -> i64 {
    find_invalid_number(input, 25)
}

#[aoc(day9, part2)]
fn solve_part2(input: &[i64]) -> i64 {
    find_sum_of_invalid_number(input, 25)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity() {
        let input = "\
        35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        assert_eq!(find_invalid_number(&parse_input(input), 5), 127);
        assert_eq!(find_sum_of_invalid_number(&parse_input(input), 5), 62);
    }

    #[test]
    fn solve() {
        let input = include_str!("../input/2020/day9.txt");
        assert_eq!(solve_part1(&parse_input(input)), 31161678);
        assert_eq!(solve_part2(&parse_input(input)), 5453868);
    }
}
