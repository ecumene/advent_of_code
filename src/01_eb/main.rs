use std::io::{self, BufRead};

fn main() {
    let inputs: Vec<u32> = io::stdin()
        .lock()
        .lines()
        .map(|line| {
            line.expect("Could not read line from standard in.")
                .parse::<u32>()
                .expect("That's not a number!")
        })
        .collect();

    inputs.iter().copied().combinations(3).for_each(|c| {
        if c.iter().sum::<i32>() == 2020 {
            result = c.iter().product()
        }
    });
    println!("{}", number_a * );
}
