use aoc_runner_derive::aoc;

struct PasswordWithPolicy<'a> {
    min: usize,
    max: usize,
    c: char,
    password: &'a str,
}

impl<'a> PasswordWithPolicy<'a> {
    fn parse(password: &'a str) -> Self {
        let (min, max, c, password): (usize, usize, char, &str) =
            serde_scan::scan!("{}-{} {}: {}" <- password).unwrap();

        Self {
            min,
            max,
            c,
            password,
        }
    }

    fn is_valid_v1(&self) -> bool {
        let count = self.password.chars().filter(|&c| c == self.c).count();

        count >= self.min && count <= self.max
    }

    fn is_valid_v2(&self) -> bool {
        self.password
            .char_indices()
            .filter(|&(i, _)| i + 1 == self.min || i + 1 == self.max)
            .filter(|&(_, c)| c == self.c)
            .count()
            == 1
    }
}

fn parse_passwords(input: &str) -> impl Iterator<Item = PasswordWithPolicy> {
    input.lines().map(|line| PasswordWithPolicy::parse(line))
}

#[aoc(day2, part1)]
fn solve_part1(input: &str) -> usize {
    parse_passwords(input)
        .filter(PasswordWithPolicy::is_valid_v1)
        .count()
}

#[aoc(day2, part2)]
fn solve_part2(input: &str) -> usize {
    parse_passwords(input)
        .filter(PasswordWithPolicy::is_valid_v2)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve() {
        let input = include_str!("../input/2020/day2.txt");

        assert_eq!(solve_part1(input), 393);
        assert_eq!(solve_part2(input), 690);
    }
}
