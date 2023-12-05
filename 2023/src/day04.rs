use aoc_runner_derive::{aoc, aoc_generator};

struct Card {
    winning_numbers: Vec<usize>,
    scratch_numbers: Vec<usize>,
}

#[aoc_generator(day4)]
fn parse_input(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let number = parts.next().unwrap().split_whitespace().nth(1).unwrap();
            let mut number_lists = parts.next().unwrap().split(" | ");
            let winning_numbers = number_lists
                .next()
                .unwrap()
                .split_whitespace()
                .filter_map(|n| n.parse().ok())
                .collect();
            let scratch_numbers = number_lists
                .next()
                .unwrap()
                .split_whitespace()
                .filter_map(|n| n.parse().ok())
                .collect();
            println!("{:?} {:?} {}", winning_numbers, scratch_numbers, number);
            Card {
                winning_numbers,
                scratch_numbers,
            }
        })
        .collect()
}

#[aoc(day4, part1)]
fn solve_part1(input: &[Card]) -> usize {
    let mut total = 0;
    for card in input {
        let matching_winning_numbers = card
            .scratch_numbers
            .iter()
            .filter(|&n| card.winning_numbers.contains(n))
            .count();

        if matching_winning_numbers > 0 {
            let mut running_total = 1;
            for _ in 0..matching_winning_numbers - 1 {
                running_total *= 2;
            }
            total += running_total;
        }
    }
    total as usize
}

#[aoc(day4, part2)]
fn solve_part2(input: &[Card]) -> usize {
    let mut tuples: Vec<(_, _)> = input
        .iter()
        .map(|card| {
            (
                card.scratch_numbers
                    .iter()
                    .filter(|&n| card.winning_numbers.contains(n))
                    .count(),
                1,
            )
        })
        .collect();

    for i in 0..tuples.len() {
        let score = tuples[i].0;
        let count = tuples[i].1;
        for _ in 0..count {
            for k in 1..score + 1 {
                tuples[i + k].1 += 1;
            }
        }
    }

    println!("{:?}", tuples);

    tuples.iter().map(|t| t.1).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let input = parse_input(input);
        assert_eq!(solve_part1(&input), 13);
        assert_eq!(solve_part2(&input), 30);
    }

    #[test]
    fn solve() {
        let input = include_str!("../input/2023/day4.txt");

        let input = parse_input(input);
        assert_eq!(solve_part1(&input), 20407);
        assert_eq!(solve_part2(&input), 55218);
    }
}
