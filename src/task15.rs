//https://www.hackerrank.com/challenges/the-birthday-bar/problem
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
    let s = read_stdin();
    let d_m = read_stdin();
    let (d, m) = (d_m[0], d_m[1]);

    let (mut num, mut sum) = (0, 0);
    if n < m {
        println!("{}", num);
        return;
    }
    for i in 0..(m as usize) { sum += s[i]; }
    if sum == d { num += 1; }
    for i in (m as usize)..(n as usize) {
        sum += s[i];
        sum -= s[i-(m as usize)];
        if sum == d { num += 1; }
    }

    println!("{}", num);
}