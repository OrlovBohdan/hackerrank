//https://www.hackerrank.com/challenges/drawing-book/problem

use std::io;

fn read_stdin() -> Vec<i32> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");

    if input.chars().nth(input.len()-1).unwrap() != '\n' {
        input.split(" ").map(|x| x.parse::<i32>().unwrap()).collect()
    } else {
        input[0..input.len()-1].split(" ").map(|x| x.parse::<i32>().unwrap()).collect()
    }
}

fn main() {
    let n = read_stdin()[0];
    let p = read_stdin()[0];

    let res = if p > n/2 { if p == n-1 { 1 } else { (n-p)/2 } } else { p/2 };
    println!("{}", res);
}