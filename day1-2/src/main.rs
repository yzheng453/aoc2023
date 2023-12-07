use std::fs::read_to_string;
use regex::Regex;
use regex::Match;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn match_to_i32(s: &str) -> i32 {
    match s {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        s => s.as_bytes()[0] as i32 - 48
    }
}

fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

fn main() {
    let re = Regex::new("[0-9]|one|two|three|four|five|six|seven|eight|nine").unwrap();   
    let lines = read_lines("input");
    let s1 = lines.iter().map(|line| {
        let m = re.find(line).unwrap();
        match_to_i32(m.as_str())
    }).reduce(|a, b| a + b).unwrap(); 
    
    let re2 = Regex::new("[0-9]|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin").unwrap();   
    let s2 = lines.iter().map(|line| {
        let q = reverse(line);
        let m = re2.find(&q).unwrap();
        let r = reverse(m.as_str());
        match_to_i32(r.as_str())
    }).reduce(|a, b| a + b).unwrap(); 
    
    println!("{}", s1 * 10 + s2);

}
