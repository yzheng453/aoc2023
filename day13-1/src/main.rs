use std::{fs::read_to_string, collections::HashMap, cmp::Ordering, collections::VecDeque, default, slice::Iter};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
        
}

fn find_line_of_reflection(pattern: &[String]) -> Option<i64> {
    for i in 0..pattern.len()-1 {
        let mut it = (i+1..pattern.len()).zip((0..=i).rev());
        let perfect = it.all(|(x ,y)| {
            &pattern[x] == &pattern[y]
        });
        if perfect {
            return Some((i + 1) as i64);
        }
    }
    None
}

fn summarize_pattern(pattern: &[String]) -> i64 {
    let row = find_line_of_reflection(pattern);
    if let Some(n) = row {
        return n * 100;
    }
    let pattern: Vec<&[u8]> = pattern.iter().map(|line| line.as_bytes()).collect();
    let rotated_pattern: Vec<String> = (0..pattern[0].len()).map(|i| 
        String::from_utf8(pattern.iter().map(|line| line[i].clone()).collect()).unwrap()).collect();
    if let Some(n) = find_line_of_reflection(&rotated_pattern) {
        return n;
    }
    panic!();
}

fn main() {
    let lines = read_lines("input");
    let ans = lines.split(|line| line == "").map(summarize_pattern).fold(0, i64::saturating_add);
    println!("{}", ans);
}
