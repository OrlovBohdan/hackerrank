//https://www.hackerrank.com/challenges/kangaroo/problem

use std::io;
fn get_numbers() -> Vec<u32> {
    let mut line = String::new();
    io::stdin().read_line(&mut line).ok().expect("Failed to read line");
    line.split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect()
}

fn main() {
    let numbers = get_numbers();
    let x1 = numbers[0];
    let v1 = numbers[1];
    let x2 = numbers[2];
    let v2 = numbers[3];

    if x1 == x2 && v1 == v2 {
        println!("YES");
    }
    else if x1 == x2 && v1 > v2 {
        println!("NO");
    }
    else if x1 <= x2 && v1 <= v2 {
        println!("NO");
    }
    else {
        if (x2 - x1) % (v1 - v2) == 0  {
            println!("YES");
        }
        else {
            println!("NO");
        }
    }
}