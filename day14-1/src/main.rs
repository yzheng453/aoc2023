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
    let a: Vec<&[u8]> = lines.iter().map(|l| l.as_bytes()).collect();
    let mut total_load = 0;
    let height = a.len();
    for i in 0..a[0].len() {
        let mut load = 0;
        let mut last = 0;
        for j in 0..height {
            match a[j][i] {
                b'O' => {
                    load += height - last;
                    last += 1;
                },
                b'.' => (),
                b'#' => {
                    last = j + 1;
                },
                _ => panic!(),
            }
        }
        total_load += load;
    }
    println!("{}", total_load);
}
