use aoc_runner_derive::{aoc, aoc_generator};

const PATTERNS: [[usize; 5]; 10] = [
    [0, 1, 2, 3, 4],
    [5, 6, 7, 8, 9],
    [10, 11, 12, 13, 14],
    [15, 16, 17, 18, 19],
    [20, 21, 22, 23, 24],
    [0, 5, 10, 15, 20],
    [1, 6, 11, 16, 21],
    [2, 7, 12, 17, 22],
    [3, 8, 13, 18, 23],
    [4, 9, 14, 19, 24],
];

pub type Board = [usize; 25];

fn has_won(calls: &[&usize], board: &Board) -> bool {
    PATTERNS
        .iter()
        .any(|&pattern| pattern.iter().all(|&idx| calls.contains(&&board[idx])))
}

fn score(calls: &[&usize], board: Board) -> usize {
    board
        .iter()
        .filter(|&&item| !calls.contains(&&item))
        .sum()
}

#[derive(Debug, PartialEq, Clone)]
pub struct Bingo {
    calls: Vec<usize>,
    boards: Vec<Board>,
}

#[aoc_generator(day4)]
pub fn parse_input(input: &str) -> Bingo {
    let mut sections = input.split("\n\n");
    let first_line = sections.next().unwrap().replace(",", " "); // Make it easy for serde_scan
    let calls: Vec<usize> = serde_scan::from_str(&first_line).expect("Cannot parse first line.");
    let boards: Vec<Board> = sections
        .map(serde_scan::from_str)
        .collect::<Result<Vec<Board>, serde_scan::ScanError>>()
        .expect("Cannot parse sections.");
    Bingo { calls, boards }
}

enum Part {
    One,
    Two
}

fn solve(inputs: &Bingo, part: Part) -> usize {
    let mut input = inputs.clone();
    let mut winner = None;
    let mut last_called = None;

    for i in 0..input.calls.len() {
        let called: Vec<&usize> = input.calls.iter().take(i).collect();
        let mut won: Vec<Board> = input
            .boards
            .drain_filter(|board| has_won(&called, board))
            .collect();
        let predecate = match part {
            Part::One => !won.is_empty(),
            Part::Two => input.boards.is_empty()
        };
        if predecate {
            winner = won.pop();
            last_called = Some(called);
            break;
        }
    }

    let last_called = last_called.unwrap();
    let score = score(&last_called, winner.unwrap());
    score * *last_called.last().unwrap()
}

#[aoc(day4, part1)]
pub fn solve_part1(inputs: &Bingo) -> usize {
    solve(inputs, Part::One)
}

#[aoc(day4, part2)]
pub fn solve_part2(inputs: &Bingo) -> usize {
    solve(inputs, Part::Two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let example = r"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

        let input = parse_input(example);

        assert_eq!(solve_part1(&input), 4512);
        assert_eq!(solve_part2(&input), 1924);
    }

    #[test]
    fn solve() {
        let input = include_str!("../input/2021/day4.txt");
        let input = parse_input(input);

        assert_eq!(solve_part1(&input), 51034);
        assert_eq!(solve_part2(&input), 5434);
    }
}
