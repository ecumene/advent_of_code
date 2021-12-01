use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day10)]
fn parse_input(input: &str) -> Vec<i16> {
    let mut output: Vec<i16> = input
        .lines()
        .map(|line| line.parse::<i16>().expect("That's not a number!"))
        .collect();
    output.push(0);
    output.sort_unstable();
    output.push(output.last().unwrap() + 3);
    output
}

#[aoc(day10, part1)]
fn solve_part1(input: &[i16]) -> usize {
    let diffs = input
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect::<Vec<i16>>();
    (diffs.iter().filter(|&diff| *diff == 1).count())
        * (diffs.iter().filter(|&diff| *diff == 3).count())
}

#[aoc(day10, part2)]
fn solve_part2(inputs: &[i16]) -> usize {
    let mut dp = vec![0; inputs.len()];
    dp[0] = 1;
    for (i, input) in inputs.iter().enumerate() {
        for (j, output) in inputs.iter().enumerate().skip(i + 1) {
            if output - input > 3 {
                break;
            }

            dp[j] += dp[i];
        }
    }
    dp.last().copied().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity() {
        let input = "\
16
10
15
5
1
11
7
19
6
12
4";
        let input = parse_input(input);
        assert_eq!(solve_part1(&input), 35);
        assert_eq!(solve_part2(&input), 8);
    }

    #[test]
    fn solve() {
        let input = include_str!("../input/2020/day10.txt");
        let input = parse_input(input);
        assert_eq!(solve_part1(&input), 2310);
        assert_eq!(solve_part2(&input), 64793042714624);
    }
}
