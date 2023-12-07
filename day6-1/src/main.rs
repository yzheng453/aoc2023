use std::{fs::read_to_string};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
        
}


fn main() {
    let lines: Vec<String> = read_lines("input");
    let time: Vec<i64> = lines[0].split(':').skip(1).next().unwrap().split(' ').flat_map(|s| s.trim().parse::<i64>()).collect();
    let dist: Vec<i64> = lines[1].split(':').skip(1).next().unwrap().split(' ').flat_map(|s| s.trim().parse::<i64>()).collect();
    let ans = time.iter().zip(dist).map(|(t, d)| {
        let f = f64::sqrt((t*t - 4*d) as f64);
        let l: i64 = ((*t as f64 - f) / 2_f64).max(0_f64).floor() as i64 + 1;
        let r: i64 = ((*t as f64 + f) / 2_f64).min(*t as f64).ceil() as i64 - 1;
        r - l + 1
    }).fold(1, i64::saturating_mul);

    println!("{}", ans);
}
