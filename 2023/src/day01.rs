use aoc_runner_derive::aoc;

fn solve<const N: usize>(input: &str, numbers_map: [&str; N]) -> usize {
    let mut total = 0;

    for line in input.lines() {
        let first_digit = numbers_map
            .iter()
            .filter_map(|item| line.find(item).map(|index| (item, index)))
            .min_by_key(|&(_, index)| index)
            .map(|(item, _)| numbers_map.iter().position(|&digit| digit == *item))
            .map(|index| index.unwrap() % 9 + 1);

        let last_digit = numbers_map
            .iter()
            .filter_map(|item| line.rfind(item).map(|index| (item, index)))
            .max_by_key(|&(_, index)| index)
            .map(|(item, _)| numbers_map.iter().position(|&digit| digit == *item))
            .map(|index| index.unwrap() % 9 + 1);

        total = total + first_digit.unwrap() * 10 + last_digit.unwrap_or(first_digit.unwrap());
    }

    total
}

#[aoc(day1, part1)]
fn solve_part1(input: &str) -> usize {
    solve::<9>(input, ["1", "2", "3", "4", "5", "6", "7", "8", "9"])
}

#[aoc(day1, part2)]
fn solve_part2(input: &str) -> usize {
    solve::<18>(
        input,
        [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3",
            "4", "5", "6", "7", "8", "9",
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(solve_part1(&input), 142);
        let input2 = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!(solve_part2(&input2), 281);
    }

    #[test]
    fn solve() {
        let input = include_str!("../input/2021/day1.txt");

        assert_eq!(solve_part1(&input), 54951);
        assert_eq!(solve_part2(&input), 55218);
    }
}
