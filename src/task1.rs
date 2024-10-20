
//https://www.hackerrank.com/challenges/solve-me-first/problem?isFullScreen=false

use std::io;
#[test]

fn main() {
    // змінні
    let mut _num_str_1 = String::new();
    let mut _num_str_2 = String::new();

    // зчитування змінних
    io::stdin().read_line(&mut _num_str_1).ok().expect("read error");
    io::stdin().read_line(&mut _num_str_2).ok().expect("read error");

    // перетворення на цілі числа
    let _num_1: i32 = _num_str_1.trim().parse().ok().expect("parse error");
    let _num_2: i32 = _num_str_2.trim().parse().ok().expect("parse error");

    // виведення суми
    println!("{}", _num_1 + _num_2);
}
