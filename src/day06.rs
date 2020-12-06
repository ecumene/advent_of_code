use aoc_runner_derive::aoc;
use itertools::Itertools;

#[aoc(day6, part1)]
fn solve_part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|section| {
            section
                .chars()
                .filter(|c| !c.is_whitespace())
                .unique()
                .count()
        })
        .sum()
}

#[aoc(day6, part2)]
fn solve_part2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            let mut all_answers = group.lines();
            let mut common = all_answers.next().unwrap().chars().collect::<Vec<_>>();

            for answers in all_answers {
                // remove all answers that are not shared
                common.retain(|&c| answers.contains(c));
            }

            common.len()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let input = "\
abc

a
b
c

ab
ac

a
a
a
a

b";

        assert_eq!(solve_part1(&input), 11);
        assert_eq!(solve_part2(&input), 6);
    }

    #[test]
    fn solve() {
        let input = include_str!("../input/2020/day6.txt");

        assert_eq!(solve_part1(&input), 6551);
    }
}
