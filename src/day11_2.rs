use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day11)]
fn parse_input(input: &str) -> Vec<Vec<Option<bool>>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| if x == 'L' { Some(true) } else { None })
                .collect()
        })
        .collect::<Vec<Vec<Option<bool>>>>()
}

#[aoc(day11, part1)]
fn solve_part1(input: &[Vec<Option<bool>>]) -> usize {
    for _ in 0..4 {
        input
            .iter()
            .tuple_windows()
            .map(|(last_line, current_line, next_line)| {
                current_line.clone().iter().enumerate().map(|(i, value)| {
                    if let None = value {
                        return false;
                    }
                    if next_line[i].unwrap_or(false)
                        && last_line[i].unwrap_or(false)
                        && current_line[i + 1].unwrap_or(false)
                        && current_line[i - 1].unwrap_or(false)
                    {
                        return false;
                    }
                    return true;
                })
            });
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity() {
        let input = "\
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
        let input = &parse_input(input);
        assert_eq!(solve_part1(input), 0);
    }
}
