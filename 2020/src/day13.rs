use aoc_runner_derive::{aoc, aoc_generator};

type Timestamp = usize;

struct PuzzleInput {
    depart_time: Timestamp,
    busses: Vec<(usize, Timestamp)>,
}

#[aoc_generator(day13)]
fn parse_input(input: &str) -> PuzzleInput {
    let mut lines = input.lines();
    let depart_time = lines
        .next()
        .unwrap()
        .parse::<Timestamp>()
        .expect("Invalid depart time");
    let busses = lines
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|(_, c)| c != &"x")
        .map(|(i, number)| (i, number.parse::<Timestamp>().expect("Invalid bus time")))
        .collect::<Vec<(usize, Timestamp)>>();
    PuzzleInput {
        depart_time,
        busses,
    }
}

#[aoc(day13, part1)]
fn solve_part1(input: &PuzzleInput) -> Timestamp {
    let output = input
        .busses
        .iter()
        .map(|(_, time)| {
            if time == &input.depart_time {
                (*time, *time)
            } else {
                (
                    input.depart_time + (time - (input.depart_time % time)),
                    *time,
                )
            }
        })
        .min_by(|(ta, _), (tb, _)| ta.cmp(tb))
        .expect("No minimum");

    (output.0 - input.depart_time) * output.1
}

#[aoc(day13, part2)]
fn solve_part2(input: &PuzzleInput) -> Timestamp {
    input
        .busses
        .iter()
        .fold((0, 1), |(mut t, step_by), (n, bus)| {
            while (t + n) % bus != 0 {
                t += step_by;
            }
            (t, step_by * bus)
        })
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity() {
        let input = "\
939
7,13,x,x,59,x,31,19
";
        let input = &parse_input(input);
        assert_eq!(solve_part1(input), 295);
        assert_eq!(solve_part2(input), 1068781);
    }

    #[test]
    fn solve() {
        let input = include_str!("../input/2020/day13.txt");
        let input = parse_input(input);

        assert_eq!(solve_part1(&input), 4782);
        assert_eq!(solve_part2(&input), 1118684865113056);
    }
}
