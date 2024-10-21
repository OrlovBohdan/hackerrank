//https://www.hackerrank.com/challenges/diagonal-difference/problem?isFullScreen=false
use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf);

    let mut iter = buf.split_whitespace();
    let n : i32 = iter.next().unwrap().parse().unwrap();
    let array : Vec<i32> = iter.map(|v| v.parse::<i32>().unwrap()).collect();

    let mut primary_sum = 0i32;
    let mut second_sum = 0i32;
    for i in 0..n {
        let prim_index = (i * n + i) as usize;
        let second_index = ((n - 1) + i * (n - 1)) as usize;
        primary_sum += array[prim_index];
        second_sum += array[second_index];
    }
    println!("{}", (second_sum - primary_sum).abs());
}