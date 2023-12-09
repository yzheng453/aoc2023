use std::{fs::read_to_string, collections::HashMap, cmp::Ordering};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
        
}

fn has_none_zero(s: &Vec<i64>) -> bool {
    s.iter().any(|&x| x != 0)
}

fn solve_for_line(line: &String) -> i64 {
    let seq: Vec<i64> = line.split(' ').map(|s| s.trim().parse().unwrap()).collect();    
    let mut a: Vec<Vec<i64>> = Vec::new();
    a.push(seq);
    let mut last = a.last().unwrap();
    while has_none_zero(last) {
        let ns: Vec<i64> = last.iter().zip(last.iter().skip(1)).map(|(x, y)| y - x).collect();
        a.push(ns);
        last = a.last().unwrap();
    }
    a.iter().rev().fold(0, |x, y| 
        x + y.last().unwrap())
}


fn main() {
    let lines = read_lines("input");
    
    let ans = lines.iter().map(solve_for_line)
        .fold(0, i64::saturating_add);
    println!("{}", ans);
}
