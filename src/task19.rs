//https://www.hackerrank.com/challenges/sock-merchant/problem

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
    let s = read_stdin();

    let mut socks: Vec<i32> = [0; 100].to_vec();
    // for i in 0..100 { socks.push(Sock{ color: i, count: 0 }); }

    for v in s { socks[(v-1) as usize] += 1; }

    let mut pairs = 0;
    for i in 0..100 { pairs += socks[i]/2; }

    println!("{}", pairs);
}