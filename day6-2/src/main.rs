use std::{fs::read_to_string};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
        
}


fn main() {
    let t: i64 = 40709879;
    let d: i64 = 215_1051_2147_1005;
    let f = f64::sqrt((t*t - 4*d) as f64);
    let l: i64 = ((t as f64 - f) / 2_f64).max(0_f64).floor() as i64 + 1;
    let r: i64 = ((t as f64 + f) / 2_f64).min(t as f64).ceil() as i64 - 1;
    let ans = r - l + 1;

    println!("{}", ans);
}
