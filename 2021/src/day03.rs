use std::collections::HashSet;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
fn parse_input(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c == '1').collect::<Vec<_>>())
        .collect()
}

fn gamma_epsilon(input: &[Vec<bool>]) -> (usize, usize) {
    let mut gamma = 0;
    let mut epsilon = 0;
    let width = input[0].len();
    for i in 0..width {
        let mut most_common = 0;
        for row in input {
            if row[i] {
                most_common += 1;
            } else {
                most_common -= 1;
            }
        }
        let place_value: u32 = ( width - 1 - i ).try_into().unwrap();
        if most_common > 0 {
            gamma += 2usize.pow(place_value);
        } else {
            epsilon += 2usize.pow(place_value);
        }
    }
    (gamma, epsilon)
}

#[aoc(day3, part1)]
fn solve_part1(input: &[Vec<bool>]) -> usize {
    let (gamma, epsilon) = gamma_epsilon(input);
    gamma * epsilon
}

fn to_decimal(input: &[bool]) -> usize {
    let mut result = 0;
    let mut place_value = 1;
    for i in input.iter().rev() {
        if *i {
            result += place_value;
        }
        place_value *= 2;
    }
    result
}

#[derive(PartialOrd, PartialEq, Eq, Hash, Clone, Copy)]
enum Value  {
    OxyRating,
    C02Scrubber,
}

fn narrow_down(value: Value, input: &[Vec<bool>], width: usize) -> usize {
    let mut bad_list = HashSet::new();
    for i in 0..width {
        let mut most_common = 0;
        let list = input.iter().enumerate().filter(|(x,_)|!bad_list.contains(x)).collect::<Vec<_>>();
        if list.len() == 1 {
            return to_decimal(list[0].1);
        }
        for ( _, row ) in list.iter() {
            if row[i] {
                most_common += 1;
            } else {
                most_common -= 1;
            }
        }
        for ( ri,row ) in list {
            if value == Value::OxyRating {
                if most_common == 0 {
                    if !row[i] {
                        bad_list.insert(ri);
                    }
                } else if ( most_common > 0 ) != row[i] {
                    bad_list.insert(ri);
                }
            }
            if value == Value::C02Scrubber {
                if most_common == 0 {
                    if row[i] {
                        bad_list.insert(ri);
                    }
                } else if ( most_common < 0 ) != row[i] {
                    bad_list.insert(ri);
                }
            }
        }
    }
    to_decimal(input.iter().enumerate().filter(|(x,_)|!bad_list.contains(x)).collect::<Vec<_>>()[0].1)
}

#[aoc(day3, part2)]
fn solve_part2(input: &[Vec<bool>]) -> usize {
    let width = input[0].len();

    let oxygen_rating = narrow_down(Value::OxyRating, input, width);
    let c02_rating = narrow_down(Value::C02Scrubber, input, width);

    oxygen_rating * c02_rating
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
        let input = parse_input(input);

        assert_eq!(solve_part1(&input), 198);
        assert_eq!(solve_part2(&input), 230);
    }

    #[test]
    fn solve() {
        let input = include_str!("../input/2021/day3.txt");
        let input = parse_input(input);

        assert_eq!(solve_part1(&input), 1997414);
        assert_eq!(solve_part2(&input), 1032597);
    }
}
