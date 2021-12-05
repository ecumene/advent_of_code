use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::cmp::Ordering;

type Line = (i32, i32, i32, i32);

#[aoc_generator(day5)]
pub fn parse_input(input: &str) -> Vec<Line> {
   input.lines().map(|line| serde_scan::scan!("{},{} -> {},{}" <- line).unwrap()).collect()
}

fn increase_point(hashmap: &mut HashMap<(i32, i32), i32>, x: i32, y: i32) {
    let key = (x, y);
    let previous_point = hashmap.get(&key);
    let next_point = previous_point.map(|point| point + 1).unwrap_or(1);
    hashmap.insert(key, next_point);
}

#[aoc(day5, part1)]
pub fn solve_part1(inputs: &[Line]) -> usize {
    let mut count = 0;
    let mut points = HashMap::new();

    for (x1, y1, x2, y2) in inputs {
        if x1 != x2 && y1 != y2 {
            continue;
        }

        if x1 == x2 {
            for y in *y1.min(y2)..y1.max(y2)+1 {
                increase_point(&mut points, *x1, y);
            }
        } else if y1 == y2 {
            for x in *x1.min(x2)..x1.max(x2)+1 {
                increase_point(&mut points, x, *y1);
            }
        }
    }
    points.values().filter(|x| x > &&1).count()
}

#[aoc(day5, part2)]
pub fn solve_part2(inputs: &[Line]) -> usize {
    let mut count = 0;
    let mut inputs = inputs.to_vec();
    let mut points = HashMap::new();

    for (x1, y1, x2, y2) in inputs.iter_mut() {
        let mut v1 = 0; 
        let mut v2 = 0;

        v1 = match x1.cmp(&x2) {
            Ordering::Less => 1,
            Ordering::Greater => -1,
            Ordering::Equal => 0,
        };

        v2 = match y1.cmp(&y2) {
            Ordering::Less => 1,
            Ordering::Greater => -1,
            Ordering::Equal => 0,
        };

        while (&x1, &y1) != (&x2, &y2) {
            increase_point(&mut points, *x1, *y1);

            *x1 += v1;
            *y1 += v2;
        }

        increase_point(&mut points, *x2, *y2);
    }
    points.values().filter(|x| x > &&1).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let example = r"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

        let input = parse_input(example);

        assert_eq!(solve_part1(&input), 5);
        assert_eq!(solve_part2(&input), 12);
    }

    #[test]
    fn solve() {
        let input = include_str!("../input/2021/day5.txt");
        let input = parse_input(input);

        assert_eq!(solve_part1(&input), 7414);
        assert_eq!(solve_part2(&input), 19676);
    }
}
