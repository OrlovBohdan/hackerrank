//https://www.hackerrank.com/challenges/staircase/problem?isFullScreen=false

use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse::<usize>().unwrap();
    for i in 0..n {
        add_whitespace(n - i - 1);
        add_sharp(i + 1);
        println!();
    }
}

fn add_whitespace(num: usize) {
    for _ in 0..num {
        print!(" ");
    }
}

fn add_sharp(num: usize) {
    for _ in 0..num {
        print!("#");
    }
}