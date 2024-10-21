//https://www.hackerrank.com/challenges/breaking-best-and-worst-records/problem
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
    let _n = read_stdin()[0];
    let scores = read_stdin();

    let (mut min, mut max) = (scores[0], scores[0]);
    let (mut l, mut h) = (0, 0);

    for s in scores {
        if s > max {
            h += 1;
            max = s;
        } else if s < min {
            l += 1;
            min = s;
        }
    }

    println!("{0} {1}", h, l);
}