//https://www.hackerrank.com/challenges/mini-max-sum/problem?isFullScreen=false
use std::io;

fn main() {

    let n = 5;

    let mut numbers_str = String::new();
    io::stdin().read_line(&mut numbers_str).ok().expect("read error");
    let alice_numbers = { numbers_str.split_whitespace()
        .take(n)
        .map(|s| s.parse::<u64>().unwrap())
    };

    let tuple = alice_numbers.fold(
        ((10u64).pow(9), 0, 0), //min, max, sum
        |acc, i| (if i < acc.0 {i} else {acc.0},
                  if i <= acc.1 {acc.1} else {i},
                  acc.2 + i
        )
    );

    println!("{} {}", tuple.2 - tuple.1, tuple.2 - tuple.0);
}