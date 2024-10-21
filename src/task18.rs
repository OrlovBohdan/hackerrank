//https://www.hackerrank.com/challenges/bon-appetit/problem
#![allow(dead_code)]

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
    let n_k = read_stdin();
    let (_n, k) = (n_k[0], n_k[1]);
    let c = read_stdin();
    let b = read_stdin()[0];

    let sum: i32 = c.iter().sum();

    if (sum - c[k as usize])/2 == b {
        println!("Bon Appetit");
    } else {
        println!("{}", c[k as usize]/2);
    }
}