use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Vec<(String, i32)> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(' ');
            (
                split.next().unwrap().to_string(),
                split.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

#[aoc(day2, part1)]
fn solve_part1(input: &[(String, i32)]) -> i32 {
    let mut depth = 0;
    let mut forward = 0;
    for (name, value) in input {
        if *name == "down" {
            depth += *value;
        }
        if *name == "up" {
            depth -= *value;
        }
        if *name == "forward" {
            forward += *value;
        }
    }

    forward * depth
}

#[aoc(day2, part2)]
fn solve_part2(input: &[(String, i32)]) -> i32 {
    let mut aim = 0;
    let mut depth = 0;
    let mut forward = 0;
    for (name, value) in input {
        let value = *value;
        if name == "down" {
            aim += value;
        } else if name == "up" {
            aim -= value;
        } else if name == "forward" {
            forward += value;
            depth += aim * value;
        }
    }
    forward * depth
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
        let input = parse_input(input);

        assert_eq!(solve_part1(&input), 150);
        assert_eq!(solve_part2(&input), 900);
    }

    #[test]
    fn solve() {
        let input = include_str!("../input/2021/day2.txt");
        let input = parse_input(input);

        assert_eq!(solve_part1(&input), 2117664);
        assert_eq!(solve_part2(&input), 2073416724);
    }
}
