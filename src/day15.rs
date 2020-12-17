use aoc_runner_derive::{aoc, aoc_generator};

type Number = usize;

#[aoc_generator(day15)]
fn parse_input(input: &str) -> Vec<Number> {
    input
        .replace('\n', "")
        .split(',')
        .map(|value| value.parse::<Number>().expect("That's not a number"))
        .collect()
}

fn solve(end: usize, numbers: &[Number]) -> usize {
    let mut map = vec![0; end];
    let mut state = 0;
    for i in 0..numbers.len() {
        let res = numbers[i];
        map[state] = i;
        state = res;
    }
    for i in numbers.len()..end {
        let v = map[state];
        let res = if v == 0 { 0 } else { i - v };
        map[state] = i;
        state = res;
    }
    state
}

#[aoc(day15, part1)]
fn solve_part1(input: &[Number]) -> Number {
    solve(2020, input)
}

#[aoc(day15, part2)]
fn solve_part2(input: &[Number]) -> Number {
    solve(30000000, input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity() {
        let input = &parse_input("0,3,6");
        assert_eq!(solve_part1(input), 436);
    }

    #[test]
    fn solve() {
        let input = include_str!("../input/2020/day15.txt");
        let input = parse_input(input);

        assert_eq!(solve_part1(&input), 866);
        assert_eq!(solve_part2(&input), 1437692);
    }
}
