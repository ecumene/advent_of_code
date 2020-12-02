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
    let number_a = inputs
        .iter()
        .find(|&&x| inputs.iter().any(|&y| y == 2020 - x))
        .expect("No number found! (u naughty naughty.. u teasing meee)");
    println!("{}", number_a * (2020 - number_a));
}
