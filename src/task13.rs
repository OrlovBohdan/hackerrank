//https://www.hackerrank.com/challenges/between-two-sets/problem
use std::io;

fn gcd(mut a: i32, mut b: i32) -> i32 {
    if b == 0 { return a; }
    a %= b;
    gcd(b, a)
}

fn lcm(mut a: i32, mut b: i32) -> i32 {
    a * b / gcd(a, b)
}

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

fn set_lcm(S: Vec<i32>) -> i32 {
    let mut LCM = S[0];

    for s in S {
        LCM = lcm(s, LCM);
    }

    LCM
}

fn set_gcd(S: Vec<i32>) -> i32 {
    let mut GCD = S[0];

    for s in S {
        GCD = gcd(s, GCD);
    }

    GCD
}

fn main() {
    let _n_m = read_stdin();
    let (A, B) = (read_stdin(), read_stdin());

    let LCM = set_lcm(A);
    let GCD = set_gcd(B);

    let mut count = 0;
    let mut m = LCM;
    let mut k = 1;
    loop {
        if m > GCD { break; }
        if GCD % m == 0 { count += 1; }
        k += 1; m = LCM * k;
    }

    println!("{}", count);
}