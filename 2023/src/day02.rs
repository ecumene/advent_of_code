use aoc_runner_derive::{aoc, aoc_generator};

struct Round {
    red: Option<usize>,
    green: Option<usize>,
    blue: Option<usize>,
}

struct Game {
    id: usize,
    rounds: Vec<Round>,
}

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let id = parts
                .next()
                .unwrap()
                .split(" ")
                .nth(1)
                .unwrap()
                .parse()
                .unwrap();
            let rounds = parts
                .next()
                .unwrap()
                .split("; ")
                .map(|round| {
                    let parts = round.split(", ");
                    let mut red = None;
                    let mut green = None;
                    let mut blue = None;
                    for part in parts {
                        let mut part = part.split(" ");
                        let value = part.next().unwrap().parse().unwrap();
                        let color = part.next().unwrap();
                        match color {
                            "red" => red = Some(value),
                            "green" => green = Some(value),
                            "blue" => blue = Some(value),
                            _ => panic!("Unknown color"),
                        }
                    }
                    Round { red, green, blue }
                })
                .collect::<Vec<Round>>();
            Game { id, rounds }
        })
        .collect::<Vec<Game>>()
}

#[aoc(day2, part1)]
fn solve_part1(input: &Vec<Game>) -> usize {
    let mut total = 0;
    for game in input {
        if game
            .rounds
            .iter()
            .find(|round| {
                round.red.map_or(false, |r| r > 12)
                    || round.green.map_or(false, |g| g > 13)
                    || round.blue.map_or(false, |b| b > 14)
            })
            .is_none()
        {
            total += game.id;
        }
    }
    total
}

#[aoc(day2, part2)]
fn solve_part2(input: &Vec<Game>) -> usize {
    input
        .iter()
        .map(|game| {
            let max_color = |color_fn: fn(&Round) -> Option<usize>| {
                game.rounds.iter().filter_map(color_fn).max().unwrap()
            };

            let red = max_color(|f| f.red);
            let green = max_color(|f| f.green);
            let blue = max_color(|f| f.blue);

            println!("{} {} {}", red, green, blue);
            red * green * blue
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let input = parse_input(&input);
        assert_eq!(solve_part1(&input), 8);
        assert_eq!(solve_part2(&input), 2286);
    }

    #[test]
    fn solve() {
        let input = include_str!("../input/2023/day2.txt");
        let input = parse_input(&input);

        assert_eq!(solve_part1(&input), 2416);
        assert_eq!(solve_part2(&input), 63307);
    }
}
