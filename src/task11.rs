//https://www.hackerrank.com/challenges/apple-and-orange/problem

use std::io;
use std::str::FromStr;
use std::fmt::Debug;

fn read_vec<T>() -> Vec<T> where T: FromStr, <T as FromStr>::Err: Debug
{
    let mut str_line = String::new();
    io::stdin().read_line(&mut str_line).expect("read error");
    str_line.trim().split(' ').map(|s| s.parse::<T>().expect("parse error")).collect()
}

fn how_many(v: &Vec<i32>, tree: i32, s: i32, t: i32) -> u32
{
    v.iter().fold(0, |acc, x| acc + if (s <= tree + *x) && (t >= tree + *x) {1} else {0})
}

fn main()
{
    let st = read_vec::<i32>();
    let ab = read_vec::<i32>();
    let _mn = read_vec::<i32>();
    let apples = read_vec::<i32>();
    let oranges = read_vec::<i32>();

    println!("{}", how_many(&apples, ab[0], st[0], st[1]));
    println!("{}", how_many(&oranges, ab[1], st[0], st[1]));
}