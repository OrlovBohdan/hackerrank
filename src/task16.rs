//https://www.hackerrank.com/challenges/divisible-sum-pairs/problem
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
    let (n, k) = (n_k[0] as usize, n_k[1]);
    let a = read_stdin();

    let mut count = 0;
    for i in 0..n {
        for j in i+1..n {
            if (a[i] + a[j]) % k == 0 { count += 1; }
        }
    }

    println!("{}", count);
}