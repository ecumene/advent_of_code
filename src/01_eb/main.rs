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
    let mut closestSum = u32::MAX;
    inputs.sort();
    let triplet = inputs.iter().peekable().find_map(|&x| {
        let split = inputs.split(|&left| left == x).next();
        if let Some(split_values) = split {
            
        } 
        None
    })
    println!("{}", number_a * );
}
