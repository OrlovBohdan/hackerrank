//https://www.hackerrank.com/challenges/birthday-cake-candles/problem

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input for number of candles");

    let _n: u32 = input
        .trim()
        .parse()
        .expect("Invalid input for number of candles");

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input for heights of candles");

    let heights: Vec<u32> = input
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid input for height of one or more candles"))
        .collect();

    let max: &u32 = heights.iter().max().unwrap();
    let mut occurrences: u32 = 0;
    for h in &heights {
        if h == max {
            occurrences += 1;
        }
    }

    println!("{}",occurrences);
}