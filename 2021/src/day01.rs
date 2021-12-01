use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Vec<u32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
fn solve_part1(input: &[u32]) -> u32 {
    let mut acc = 0;
    for (a, b) in input.iter().tuple_windows() {
        if a < b {
            acc += 1;
        }
    }
    acc
}

#[aoc(day1, part2)]
fn solve_part2(input: &[u32]) -> u32 {
    let mut acc = 0;
    let mut iter = input.iter().tuple_windows::<(_, _, _)>().peekable();
    while let Some((a, b, c)) = iter.next() {
        let next = iter.peek();
        if let Some((i, j, k)) = next {
            if a + b + c < *i + *j + *k {
                acc += 1
            }
        }
    }
    acc
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
