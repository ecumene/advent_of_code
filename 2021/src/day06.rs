use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
pub fn parse_input(input: &str) -> [usize; 9] {
    input
    .split(',')
    .map(|x| x.parse::<usize>().unwrap())
    .fold([0; 9], |mut a, n| {
        a[n] += 1;
        a
    })
}

const EIGHT_AHEAD: [usize; 9] = [7, 8, 0, 1, 2, 3, 4, 5, 6];

fn solve<const I: usize>(mut fish_count: [usize; 9]) -> usize {
    // The first N iterations, 9 at a time
    for _ in 0..(I / 9) {
        for (i, &eight_ahead) in EIGHT_AHEAD.iter().enumerate() {
            fish_count[eight_ahead] += fish_count[i];
        }
    }

    // The rest that didn't fit into the 9 iterations
    for g in 0..(I % 9) {
        fish_count[EIGHT_AHEAD[g]] += fish_count[g];
    }

    fish_count.iter().sum()
}


#[aoc(day6, part1)]
pub fn solve_part1(inputs: &[usize; 9]) -> usize {
    solve::<80>(*inputs)
}

#[aoc(day6, part2)]
pub fn solve_part2(inputs: &[usize; 9]) -> usize {
    solve::<256>(*inputs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let example = "3,4,3,1,2";

        let input = parse_input(example);

        assert_eq!(solve_part1(&input), 5934);
        assert_eq!(solve_part2(&input), 26984457539);
    }

    #[test]
    fn solve() {
        let input = include_str!("../input/2021/day6.txt");
        let input = parse_input(input);

        assert_eq!(solve_part1(&input), 374927);
        assert_eq!(solve_part2(&input), 1687617803407);
    }
}
