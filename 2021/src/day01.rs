use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Vec<u32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn solve<const N: usize>(inputs: &[u32]) -> usize {
    inputs.windows(N).filter(|w| w[0] < w[N - 1]).count()
}

#[aoc(day1, part1)]
fn solve_part1(input: &[u32]) -> usize {
    solve::<2>(input)
}

#[aoc(day1, part2)]
fn solve_part2(input: &[u32]) -> usize {
    solve::<4>(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "199
200
208
210
200
207
240
269
260
263";
        let input = parse_input(input);

        assert_eq!(solve_part1(&input), 7);
        assert_eq!(solve_part2(&input), 5);
    }

    #[test]
    fn solve() {
        let input = include_str!("../input/2021/day1.txt");
        let input = parse_input(input);

        assert_eq!(solve_part1(&input), 1462);
        assert_eq!(solve_part2(&input), 1497);
    }
}
