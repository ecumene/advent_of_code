use aoc_runner_derive::{aoc, aoc_generator};

#[derive(PartialEq, Copy, Clone)]
struct Seat(Option<bool>);

impl Seat {
    fn is_occupied(&self) -> usize {
        if self.0.unwrap_or(false) {
            1
        } else {
            0
        }
    }
}

#[derive(Copy, Clone)]
enum Direction {
    Increase,
    Decrease,
    Steady,
}

impl Direction {
    fn next(&self, x: usize) -> usize {
        match self {
            Direction::Increase => x + 1,
            Direction::Decrease => x.wrapping_sub(1),
            Direction::Steady => x,
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct Floor {
    floor: Vec<Vec<Seat>>,
}

impl Floor {
    fn queen_iterator(
        &self,
        row: usize,
        col: usize,
        delta_r: Direction,
        delta_c: Direction,
    ) -> QueenIterator {
        QueenIterator {
            row,
            col,
            delta_r,
            delta_c,
            floor: &self.floor,
        }
    }

    fn count_neighbors(&self, row: usize, col: usize) -> usize {
        let row_min = row.saturating_sub(1);
        let row_max = (row + 1).min(self.floor.len() - 1);

        let col_min = col.saturating_sub(1);
        let col_max = (col + 1).min(self.floor[0].len() - 1);

        self.floor[row_min..=row_max]
            .iter()
            .map(|floor_row| {
                floor_row[col_min..=col_max]
                    .iter()
                    .map(|x| x.is_occupied())
                    .sum::<usize>()
            })
            .sum::<usize>()
            .saturating_sub(self.floor[row][col].is_occupied())
    }

    fn count_queen(&self, row: usize, col: usize) -> usize {
        [
            (Direction::Steady, Direction::Decrease),
            (Direction::Steady, Direction::Increase),
            (Direction::Decrease, Direction::Steady),
            (Direction::Increase, Direction::Steady),
            (Direction::Decrease, Direction::Decrease),
            (Direction::Decrease, Direction::Increase),
            (Direction::Increase, Direction::Increase),
            (Direction::Increase, Direction::Decrease),
        ]
        .iter()
        .map(|&(delta_r, delta_c)| {
            self.queen_iterator(row, col, delta_r, delta_c)
                .find_map(|x: &Seat| Some(x.is_occupied()).filter(|_| x.0.is_some()))
                .unwrap_or(0)
        })
        .sum()
    }

    fn rule<F>(&self, row: usize, col: usize, threshold: usize, count_fn: &F) -> Seat
    where
        F: Fn(&Self, usize, usize) -> usize,
    {
        let seat = self.floor[row][col];
        if !seat.0.is_some() {
            return seat;
        }

        let occ_count = count_fn(self, row, col);
        let occ = seat.0.unwrap();

        if !occ && occ_count == 0 {
            Seat(Some(true))
        } else if occ && occ_count >= threshold {
            Seat(Some(false))
        } else {
            seat
        }
    }

    fn tick<F>(&self, threshold: usize, count_fn: &F, floor: &mut [Vec<Seat>])
    where
        F: Fn(&Self, usize, usize) -> usize,
    {
        for (r, row) in floor.iter_mut().enumerate() {
            for (c, cell) in row.iter_mut().enumerate() {
                *cell = self.rule(r, c, threshold, count_fn);
            }
        }
    }
}

struct QueenIterator<'a> {
    row: usize,
    col: usize,
    delta_r: Direction,
    delta_c: Direction,
    floor: &'a [Vec<Seat>],
}

impl<'a> Iterator for QueenIterator<'a> {
    type Item = &'a Seat;

    fn next(&mut self) -> Option<Self::Item> {
        self.row = self.delta_r.next(self.row);
        self.col = self.delta_c.next(self.col);

        self.floor.get(self.row).and_then(|row| row.get(self.col))
    }
}

#[aoc_generator(day11)]
pub fn generator(input: &str) -> Floor {
    Floor {
        floor: input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|x| match x {
                        'L' => Seat(Some(false)),
                        '#' => Seat(Some(true)),
                        '.' => Seat(None),
                        _ => panic!("Uknown character {}", x),
                    })
                    .collect::<Vec<Seat>>()
            })
            .collect(),
    }
}

fn solve<F>(mut current: Floor, threshold: usize, count_fn: F) -> usize
where
    F: Fn(&Floor, usize, usize) -> usize,
{
    let mut next = current.clone();

    loop {
        current.tick(threshold, &count_fn, &mut next.floor);
        if next == current {
            break;
        }

        std::mem::swap(&mut current, &mut next);
    }

    current
        .floor
        .iter()
        .flat_map(|row| row.iter())
        .map(|s| s.is_occupied())
        .sum()
}

#[aoc(day11, part1)]
pub fn part1(inputs: &Floor) -> usize {
    solve(inputs.clone(), 4, Floor::count_neighbors)
}

#[aoc(day11, part2)]
pub fn part2(inputs: &Floor) -> usize {
    solve(inputs.clone(), 5, Floor::count_queen)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 37);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 26);
    }

    #[test]
    pub fn test() {
        let input = include_str!("../input/2020/day11.txt").trim_end_matches('\n');
        assert_eq!(part1(&generator(input)), 2261);
        assert_eq!(part2(&generator(input)), 2039);
    }
}
