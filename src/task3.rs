//https://www.hackerrank.com/challenges/compare-the-triplets/problem?isFullScreen=false
use std::io;
use std::str::FromStr;
use std::ops::Index;

pub fn main() {
    let alice_triplet: Vec<i32> = get_stdin_values();
    let bob_triplet: Vec<i32> = get_stdin_values();

    let mut score_alice = 0;
    let mut score_bob = 0;

    for i in 0..alice_triplet.len() {
        let alice_comparison_score = alice_triplet.index(i);
        let bob_comparison_score = bob_triplet.index(i);

        if alice_comparison_score > bob_comparison_score {
            score_alice += 1;
        } else if alice_comparison_score < bob_comparison_score {
            score_bob += 1;
        }
    }

    println!("{} {}", score_alice, score_bob);
}

fn get_stdin_line() -> String {
    let mut line = String::default();
    let _ = io::stdin().read_line(&mut line);
    line
}

fn get_stdin_values<V: FromStr>() -> Vec<V> {
    let line = get_stdin_line();
    let mut values = Vec::default();

    for value in line.split_whitespace() {
        values.push(value.parse::<V>().ok().expect("value is of incorrect type"));
    }

    values
}