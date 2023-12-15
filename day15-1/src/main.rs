use std::{fs::read_to_string, collections::HashMap, cmp::Ordering, collections::VecDeque, default, slice::Iter};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
        
}


fn main() {
    let lines = read_lines("input");
    let seq: Vec<&str> = lines[0].split(',').collect();
    let ans = seq.iter().map(|&step| {
        let mut current = 0_i32;
        for b in step.as_bytes() {
            current += *b as i32;
            current *= 17;
            current = current % 256;
        }
        current
    }).fold(0, i32::saturating_add);
    println!("{}", ans);
}
