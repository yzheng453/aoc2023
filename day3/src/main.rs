use std::{fs::read_to_string};
use regex::Regex;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
        
}

fn calc_sum_for_line(row: usize, line: &String, lines: &Vec<String>) -> i64 {
    let re = Regex::new(r"\*").unwrap();
    let prev_row = if row > 0 {lines.get(row - 1)}
    else {None};

    let next_row = lines.get(row + 1);

    let sum = re.find_iter(line).into_iter().map(|m| calc_gear_ratio(line, prev_row, next_row, m.start())).fold(0, i64::saturating_add);
    println!("Sum: {}", sum);
    sum
}

fn calc_gear_ratio(line: &String, prev_row: Option<&String>, next_row: Option<&String>, pos: usize) -> i64 {
    let v = find_numbers_for_gear(line, prev_row, next_row, pos);
    if v.len() == 2 {
        v[0] * v[1]
    } else {
        0
    }
}

fn find_numbers_for_gear(line: &String, prev_row: Option<&String>, next_row: Option<&String>, pos: usize) -> Vec<i64> {
    let mut result = Vec::new();
    prev_row.map(|l| {
        let mut v = find_numbers_in_line(l, pos);
        result.append(&mut v)
    });
    next_row.map(|l| {
        let mut v = find_numbers_in_line(l, pos);
        result.append(&mut v)
    });
    let mut v = find_numbers_in_line(line, pos);
    result.append(&mut v);
    result
}

fn find_numbers_in_line(line: &String, pos: usize) -> Vec<i64> {
    let mut positions = Vec::new();
    let cs: Vec<char> = line.chars().collect::<Vec<char>>();
    if pos > 0 {
        if let Some(pair) = search_for_number(&cs, pos - 1) {
            positions.push(pair)
        }
    }
    if let Some(pair) = search_for_number(&cs, pos) {
        positions.push(pair)
    }
    
    if let Some(pair) = search_for_number(&cs, pos + 1) {
        positions.push(pair)
    }
    positions.dedup();
    positions.iter().map(|(x, y)| cs.get(*x..*y).unwrap().iter().collect::<String>().parse().unwrap()).collect()
}

fn search_for_number(cs: &Vec<char>, pos: usize) -> Option<(usize, usize)> {
    if pos >= cs.len() {
        return None
    }
    let left = {
        let mut i = pos;
        while (i > 0) && is_num(cs[i]) {
            i -= 1;            
        }
        if !is_num(cs[i]) {
            i += 1;
        }
        i
    };
    let mut right = pos;
    while cs.get(right).filter(|c| is_num(**c)).is_some() {
        right += 1;
    }
    if left < right {
        Some((left, right))
    } else {
        None
    }
}

fn is_num(c: char) -> bool {
    (c >='0') && (c <='9')   
}

fn main() {
    let lines = read_lines("input");
    let sum = lines.iter().enumerate().map(|(row, line)| calc_sum_for_line(row, line, &lines)).fold(0, i64::saturating_add);
    println!("{}", sum);
}