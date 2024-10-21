//https://www.hackerrank.com/challenges/a-very-big-sum/problem?isFullScreen=false
// Enter your code here
use std::io;


fn main() {
    let mut limit = String::new();
    let mut array = String::new();

    io::stdin().read_line(&mut limit).ok();
    io::stdin().read_line(&mut array).ok();

    let limit: i64 = limit.trim().parse().unwrap();

    let mut array = array.split_whitespace();

    let mut sum: i64 = 0;

    for _ in 0..limit {
        sum += array.next().unwrap().parse().unwrap();
    }

    print!("{}", sum);


}