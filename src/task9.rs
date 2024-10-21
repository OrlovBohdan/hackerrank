//https://www.hackerrank.com/challenges/time-conversion/problem?isFullScreen=false
use std::io;

pub fn main() {
    let raw = get_stdin_line();
    let time = raw.trim();
    let (hours, rest) = time.split_at(2);
    let hours: i32 = hours.parse().unwrap();
    let (middle, raw_am_pm) = rest.split_at(rest.len() - 2);
    let am_pm = String::from(raw_am_pm).to_uppercase();

    if am_pm == "AM" {
        if hours == 12 {
            println!("00{}", middle);
        } else {
            println!("{:>0width$}{}", hours, middle, width=2);
        }
    } else {
        if hours == 12 {
            println!("12{}", middle);
        } else {
            let mut military_hours = hours - 12;
            if military_hours < 0 {
                military_hours += 24;
            }
            println!("{:>0width$}{}", military_hours, middle, width=2);
        }
    }
}

fn get_stdin_line() -> String {
    let mut line = String::default();
    let _ = io::stdin().read_line(&mut line);
    line
}