use aoc_runner_derive::aoc;

#[aoc(day3, part1)]
fn solve_part1(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut total = 0;
    for i in 0..rows {
        let mut is_number = false;
        let mut number_buffer = String::new();
        let mut is_valid = false;

        for j in 0..cols {
            let char_numeric = grid[i][j].is_numeric();
            let mut surrounding_chars = vec![];

            if char_numeric && number_buffer.len() == 0 {
                is_valid = false;
            }

            for di in -1..=1 {
                for dj in -1..=1 {
                    if di == 0 && dj == 0 {
                        continue;
                    };

                    let new_i = i as i32 + di;
                    let new_j = j as i32 + dj;

                    if new_i >= 0 && new_i < rows as i32 && new_j >= 0 && new_j < cols as i32 {
                        surrounding_chars.push(grid[new_i as usize][new_j as usize]);
                    }
                }
            }

            let is_row_end = j == cols - 1;

            if char_numeric {
                number_buffer.push(grid[i][j]);
                is_number = true;

                if surrounding_chars
                    .iter()
                    .filter(|c| **c != '.' && !c.is_numeric())
                    .count()
                    > 0
                {
                    is_valid = true;
                }
            }

            if is_number && (!char_numeric || is_row_end) {
                if is_valid {
                    total += number_buffer.parse::<usize>().unwrap();
                }
                number_buffer.clear();
                is_number = false;
            }
        }
    }
    total
}

#[aoc(day3, part2)]
fn solve_part2(input: &str) -> usize {
    let mut bbs = vec![];

    for (y, line) in input.lines().enumerate() {
        let cols = line.chars().count();
        let mut is_number = false;
        let mut str_buffer = String::new();
        let mut start_idx = 0;
        for (x, char) in line.char_indices() {
            if char.is_numeric() && !is_number {
                start_idx = x;
            }

            let top_left_x = start_idx.saturating_sub(1);
            let top_left_y = y.saturating_sub(1);
            let bottom_right_x = x;
            let bottom_right_y = y + 1;

            if char.is_numeric() {
                str_buffer.push(char);
                is_number = true;
            }

            let is_row_end = x == cols - 1;

            if is_number && (!char.is_numeric() || is_row_end) {
                println!("str_buffer: {}", str_buffer);
                bbs.push((
                    str_buffer.parse::<usize>().unwrap(),
                    (top_left_x, top_left_y),
                    (bottom_right_x, bottom_right_y),
                ));
                str_buffer.clear();
                is_number = false;
            }
        }
    }

    // print bbs
    println!("bbs: {:?}", bbs);

    // find stars in bbs
    let mut total = 0;
    for (i, line) in input.lines().enumerate() {
        for (star_idx, char) in line.char_indices() {
            if char == '*' {
                let matching = bbs
                    .iter()
                    .filter(
                        |(_, (top_left_x, top_left_y), (bottom_right_x, bottom_right_y))| {
                            star_idx >= *top_left_x
                                && star_idx <= *bottom_right_x
                                && i >= *top_left_y
                                && i <= *bottom_right_y
                        },
                    )
                    .map(|(number, _, _)| *number)
                    .collect::<Vec<usize>>();
                println!("matching: {:?}", matching);
                if matching.len() == 2 {
                    total += matching[0] * matching[1];
                }
            }
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(solve_part1(&input), 4361);
        assert_eq!(
            solve_part1(
                "........
.24..4..
......*."
            ),
            4
        );
        assert_eq!(
            solve_part1(
                "........
.24$-4..
......*."
            ),
            28
        );
        assert_eq!(
            solve_part1(
                "11....11
..$..$..
11....11"
            ),
            44
        );
        assert_eq!(
            solve_part1(
                "$......$
.1....1.
.1....1.
$......$"
            ),
            4
        );
        assert_eq!(
            solve_part1(
                "$......$
.11..11.
.11..11.
$......$"
            ),
            44
        );
        assert_eq!(
            solve_part1(
                "$11
...
11$
..."
            ),
            22
        );
        assert_eq!(
            solve_part1(
                "$..
.11
.11
$..
..$
11.
11.
..$"
            ),
            44
        );
        assert_eq!(solve_part1("11.$."), 0);
        assert_eq!(solve_part2(&input), 467835);
    }

    #[test]
    fn solve() {
        let input = include_str!("../input/2023/day3.txt");

        assert_eq!(solve_part1(&input), 530495);
        assert_eq!(solve_part2(&input), 55218);
    }
}
