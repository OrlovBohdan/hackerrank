//https://www.hackerrank.com/challenges/migratory-birds/problem
fn get_max(arr: Vec<i32>) -> i32 {
    let mut x: Option<i32> = None;
    let mut c = 0;
    let mut max_val = (0i32 ,0);
    let check_max = |cur: (i32, usize), val: i32, count: usize| -> (i32, usize) {
        if cur.1 < count { (val, count) }
        else  { cur }
    };
    for a in arr.into_iter() {
        match x {
            None => { x = Some(a); c = 1; },
            Some(v) if v != a => {
                max_val = check_max(max_val, v, c);
                x = Some(a);
                c = 1;
            },
            _ => { c += 1; }
        }
    }
    max_val = check_max(max_val, x.unwrap(), c);
    return max_val.0;
}
fn main() {
    let read_line = || {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).ok();
        line.trim().to_string()
    };
    read_line();
    let mut v = read_line().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
    v.sort();
    println!("{}", get_max(v));
}