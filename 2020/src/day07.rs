use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

const KEYWORD: &'static str = "shiny gold";

type Color = String;
pub struct Rules(HashMap<Color, Vec<(Color, usize)>>);

impl Rules {
    fn contains(&self, needle: &str, target: &str) -> bool {
        self.0[needle]
            .iter()
            .any(|(color, _)| color == target || self.contains(color, target))
    }

    fn count_contained(&self, color: &str) -> usize {
        self.0[color]
            .iter()
            .map(|(c, count)| count * self.count_contained(c))
            .sum::<usize>()
            + 1
    }

    fn parse_color(color: &str) -> (Color, usize) {
        let (amount, adjective, color_name): (usize, String, String) =
            serde_scan::from_str(color).unwrap();
        (
            format!("{} {}", adjective.clone(), color_name.as_str()),
            amount,
        )
    }

    fn parse_rule(line: &str) -> (Color, Vec<(Color, usize)>) {
        let mut rule_split = line.split(" contain ");
        let parent_color = rule_split
            .next()
            .expect("Invalid color")
            .replace(" bags", "");
        let parent_values = rule_split.next().expect("Invalid value");
        if parent_values == "no other bags." {
            (parent_color, vec![])
        } else {
            (
                parent_color,
                parent_values
                    .replace('.', "")
                    .split(",")
                    .map(|color_and_count| Rules::parse_color(color_and_count))
                    .collect(),
            )
        }
    }
}

#[aoc_generator(day7)]
pub fn generator(input: &str) -> Rules {
    let mut rules = HashMap::new();

    for line in input.lines() {
        let (parent_color, parent_rules) = Rules::parse_rule(line);
        let entry = rules.entry(parent_color).or_insert_with(Vec::new);
        for rule in parent_rules {
            entry.push(rule);
        }
    }

    Rules(rules)
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &Rules) -> usize {
    input
        .0
        .iter()
        .filter(|&(color, _)| input.contains(color, KEYWORD))
        .count()
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &Rules) -> usize {
    input.count_contained(KEYWORD) - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bag_rules() {
        assert_eq!(Rules::parse_rule("vibrant maroon bags contain 3 dark fuchsia bags, 3 plaid turquoise bags, 1 pale silver bag, 4 shiny cyan bags."),
        ("vibrant maroon".to_owned(), vec![("dark fuchsia".to_owned(), 3), ("plaid turquoise".to_owned(), 3), ("pale silver".to_owned(), 1), ("shiny cyan".to_owned(), 4)]))
    }

    #[test]
    fn solve_example() {
        let input = "\
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
";
        let input = generator(input);
        assert_eq!(solve_part1(&input), 4);
        assert_eq!(solve_part2(&input), 32);
    }

    #[test]
    fn solve() {
        let input = include_str!("../input/2020/day7.txt");
        let input = generator(input);
        assert_eq!(solve_part1(&input), 332);
        assert_eq!(solve_part2(&input), 10875);
    }
}
