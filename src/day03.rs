use aoc_runner_derive::{aoc, aoc_generator};

const TRAVERSAL_PATHS: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

#[aoc_generator(day3)]
fn parse_input(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| line.chars().map(|x| x == '#').collect())
        .collect::<Vec<Vec<bool>>>()
}

#[aoc(day3, part1)]
fn solve_part1(input: &Vec<Vec<bool>>) -> usize {
    input
        .iter()
        .enumerate()
        .map(|(i, x)| (x[3 * i % x.len()]) as usize)
        .sum()
}

fn traverse(input: &Vec<Vec<bool>>, right: &usize, down: &usize) -> usize {
    input
        .iter()
        .enumerate()
        .filter(|(i, x)| i % down == 0 && x[i * right / down % x.len()])
        .count()
}

#[aoc(day3, part2)]
fn solve_part2(input: &Vec<Vec<bool>>) -> usize {
    TRAVERSAL_PATHS
        .iter()
        .map(|(right, down)| traverse(input, right, down))
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity() {
        let input = "\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
        assert_eq!(solve_part1(&parse_input(input)), 7);
    }

    #[test]
    fn solve() {
        let input = include_str!("../input/2020/day3.txt");
        let input = &parse_input(input);

        assert_eq!(solve_part1(input), 282);
        assert_eq!(solve_part2(input), 958815792);
    }
}
