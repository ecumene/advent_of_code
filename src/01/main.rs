use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut inputs: Vec<u32> = vec![];
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in.");
        inputs.push(line.parse::<u32>().expect("That's not a number!"));
    }
    let number_a = inputs
        .iter()
        .find(|&&x| inputs.iter().any(|&y| y == 2020 - x))
        .expect("No number found! (u naughty naughty.. u teasing meee)");
    let number_b = 2020 - number_a;
    println!("{:?}", number_a * number_b);
}
