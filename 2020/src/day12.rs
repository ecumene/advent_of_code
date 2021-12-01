use aoc_runner_derive::{aoc, aoc_generator};
use serde::Deserialize;

#[derive(Clone, Copy)]
enum Heading {
    N,
    S,
    E,
    W,
}

#[derive(Deserialize)]
enum Direction {
    L,
    R,
}

enum Instruction {
    Move(Heading, i32),
    Rotate(Direction, i32),
    Forwards(i32),
}

impl Heading {
    fn right(&self) -> Self {
        match &self {
            Heading::N => Heading::E,
            Heading::E => Heading::S,
            Heading::S => Heading::W,
            Heading::W => Heading::N,
        }
    }

    fn left(&self) -> Self {
        match &self {
            Heading::E => Heading::N,
            Heading::S => Heading::E,
            Heading::W => Heading::S,
            Heading::N => Heading::W,
        }
    }

    fn rotate_right(&mut self, angle: &i32, waypoint: &mut Option<Waypoint>) {
        let times = angle.rem_euclid(360) / 90;
        if let Some(ref mut waypoint) = waypoint {
            for _ in 0..times {
                std::mem::swap(&mut waypoint.north, &mut waypoint.east);
                waypoint.north *= -1;
            }
        } else {
            for _ in 0..times {
                *self = self.right();
            }
        }
    }

    fn rotate_left(&mut self, angle: &i32, waypoint:  &mut Option<Waypoint>) {
        let times = angle.rem_euclid(360) / 90;
        if let Some(ref mut waypoint) = waypoint {
            for _ in 0..times {
                std::mem::swap(&mut waypoint.east, &mut waypoint.north);
                waypoint.east *= -1;
            }
        } else {
            for _ in 0..times {
                *self = self.left();
            }
        }
    }
}

struct Waypoint {
    north: i32,
    east: i32,
}

impl Waypoint { 
    fn new(north: i32, east: i32) -> Waypoint {
        Waypoint {
            north,
            east 
        }
    }
}

struct Ship {
    north: i32,
    east: i32,
    heading: Heading,
    waypoint: Option<Waypoint>,
}

impl Ship {
    fn new(waypoint: Option<Waypoint>) -> Self {
        Ship {
            north: 0,
            east: 0,
            heading: Heading::E,
            waypoint,
        }
    }

    fn head(&mut self, heading: &Heading, value: &i32) {
        if let Some(ref mut waypoint) = self.waypoint {
            match heading {
                Heading::N => waypoint.north += value,
                Heading::E => waypoint.east += value,
                Heading::S => waypoint.north -= value,
                Heading::W => waypoint.east -= value,
            }
        } else {
            match heading {
                Heading::N => self.north += value,
                Heading::E => self.east += value,
                Heading::S => self.north -= value,
                Heading::W => self.east -= value,
            }
        }
    }

    fn forwards(&mut self, value: &i32) {
        if let Some(ref waypoint) = self.waypoint {
            self.north += waypoint.north * value;
            self.east += waypoint.east * value;
        } else {
            let heading = self.heading;
            self.head(&heading, value);
        }
    }

    fn rotate(&mut self, rotation: &Direction, value: &i32) {
        match rotation {
            Direction::L => self.heading.rotate_left(value, &mut self.waypoint),
            Direction::R => self.heading.rotate_right(value, &mut self.waypoint),
        }
    }

    fn run(mut self, program: &[Instruction]) -> Self {
        for instruction in program {
            match instruction {
                Instruction::Move(heading, value) => self.head(heading, value),
                Instruction::Rotate(direction, value) => self.rotate(direction, value),
                Instruction::Forwards(value) => self.forwards(value),
            }
        }
        self
    }

    fn distance(&self) -> i32 {
        self.north.abs() + self.east.abs()
    }
}

#[aoc_generator(day12)]
fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            let direction = chars.next().unwrap();
            let value = chars
                .collect::<String>()
                .parse::<i32>()
                .expect("Invalid value");
            match direction {
                'N' => Instruction::Move(Heading::N, value),
                'E' => Instruction::Move(Heading::E, value),
                'S' => Instruction::Move(Heading::S, value),
                'W' => Instruction::Move(Heading::W, value),
                'L' => Instruction::Rotate(Direction::L, value),
                'R' => Instruction::Rotate(Direction::R, value),
                'F' => Instruction::Forwards(value),
                _ => panic!("Unknown character {}", direction),
            }
        })
        .collect()
}

#[aoc(day12, part1)]
fn solve_part1(input: &[Instruction]) -> i32 {
    Ship::new(None).run(input).distance()
}

#[aoc(day12, part2)]
fn solve_part2(input: &[Instruction]) -> i32 {
    Ship::new(Some(Waypoint::new(1, 10))).run(input).distance()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity() {
        let input = "\
F10
N3
F7
R90
F11";
        let input = &parse_input(input);
        assert_eq!(solve_part1(input), 25);

        let input = "\
F10
L180
F10
F10
R180
F10
";
        let input = &parse_input(input);
        assert_eq!(solve_part1(input), 0);

        let input = "\
F10
L180
F10
";
        let input = &parse_input(input);
        assert_eq!(solve_part1(input), 0);
    }

    #[test]
    fn solve() {
        let input = include_str!("../input/2020/day12.txt");
        let input = parse_input(input);

        assert_eq!(solve_part1(&input), 562);
        assert_eq!(solve_part2(&input), 101860);
    }
}
