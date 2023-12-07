use std::{fs::read_to_string};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
        
}

fn f1(s: &str) -> std::result::IntoIter<i64> {
    let a = s.trim();
    let b = a.parse::<i64>();
    b.into_iter()
}

fn main() {
    let lines: Vec<String> = read_lines("input");
    let mut copies = [1_i64; 218];
    
    for (i, line) in lines.iter().enumerate() {
        let p0 = line.find(':').unwrap() + 1;
        let p1 = line.find('|').unwrap();
        let mut winning_numbers: Vec<i64> = (&line[p0..p1]).trim().split(' ').flat_map(f1).collect();
        winning_numbers.sort();
        let mut own_numbers: Vec<i64> = (&line[(p1 + 1)..]).trim().split(' ').flat_map(f1).collect();
        own_numbers.sort();
        let mut it_winning = winning_numbers.iter().peekable();
        let mut cnt = 0 as usize;
        for o in own_numbers {
            while it_winning.peek().filter(|w| ***w < o).is_some() {
                it_winning.next();
            }
            
            if let Some(&&w) = it_winning.peek() {
                if w == o {
                    cnt += 1;
                }
            }
        }
        let delta = copies[i];
        for j in (i+1) .. (i+1+cnt) {
            copies[j] += delta;
        }
        println!("{}: {}", i, cnt);
    }
    println!("{}", copies[0..(lines.len())].iter().fold(0_i64, |a, b| a + b));
}
