use std::fs::read_to_string;
use regex::Regex;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn main() {
    let lines = read_lines("input");
    let re = Regex::new("[0-9]").unwrap();   
    let s1 = lines.iter().map(|line| {
        let s = re.find(line).unwrap().start();
        line.chars().nth(s).unwrap() as i32 - 48
    }).reduce(|a, b| a + b).unwrap(); 
    
    
    let s2 = lines.iter().map(|line| {
        let s = re.find_iter(line).last().unwrap().start();
        line.chars().nth(s).unwrap() as i32 -48
    }).reduce(|a, b| a + b).unwrap(); 
    
    println!("{}", s1 * 10 + s2);

}
