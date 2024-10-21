//https://www.hackerrank.com/challenges/plus-minus/problem?isFullScreen=false

use std::io;

fn main() {
    let mut s1 = String::new();
    let mut s2 = String::new();

    io::stdin().read_line(&mut s1).ok().expect("Read error");
    io::stdin().read_line(&mut s2).ok().expect("Read error");

    let num_vec = s2.split(" ");
    let n = s1.trim().parse::<i32>().ok().expect("Parse error");
    let mut pos = 0;
    let mut negs = 0;
    let mut zeroes = 0;
    for num in num_vec {
        //println!("{}", num);
        match num.parse::<i32>().ok().expect("Parse error") {
            x if x > 0 => pos += 1,
            x if x < 0 => negs += 1,
            x if x == 0 => zeroes += 1,
            _ => (),
        }
    }
    println!("{:.6}", pos as f32/n as f32);
    println!("{:.6}", negs as f32/n as f32);
    println!("{:.6}", zeroes as f32/n as f32);
}