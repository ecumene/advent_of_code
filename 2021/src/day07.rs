use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day7)]
pub fn parse_input(input: &str) -> Vec<usize> {
    input
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

#[aoc(day7, part1)]
pub fn solve_part1(inputs: &[usize]) -> usize {
    let mut lowest_fuel = usize::MAX;
    for position_1 in inputs {
        let mut movement = 0;
        for position_2 in inputs {
            movement += (*position_1 as i32 - *position_2 as i32).abs() as usize;
        }
        if movement < lowest_fuel {
            lowest_fuel = movement;
        }
    }
    lowest_fuel
}

#[aoc(day7, part2)]
pub fn solve_part2(inputs: &[usize]) -> usize {
    let mut lowest_fuel = usize::MAX;
    let min = inputs.iter().min().unwrap();
    let max = inputs.iter().max().unwrap();
    for i in *min..*max {
        let mut movement = 0;
        for value in inputs {
            let movement_dist = (*value as i32 - i as i32).abs() as usize;
            movement += movement_dist * (movement_dist + 1) / 2;
        }
        if movement < lowest_fuel {
            lowest_fuel = movement;
        }
    }
    lowest_fuel
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let example = "16,1,2,0,4,2,7,1,2,14";

        let input = parse_input(example);

        assert_eq!(solve_part1(&input), 37);
    }

    #[test]
    fn solve() {
        let input = include_str!("../input/2021/day7.txt");
        let input = parse_input(input);

        assert_eq!(solve_part1(&input), 347449);
        assert_eq!(solve_part2(&input), 98039527);
    }
}
