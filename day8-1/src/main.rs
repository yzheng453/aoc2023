use std::{fs::read_to_string, collections::HashMap, cmp::Ordering};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
        
}

struct Node {
    v: String,
    l: String,
    r: String,
}

fn parse_line(line: &String, nodes: &mut HashMap<String, Node>) {
    let mut it_1 = line.split('=');
    let v = it_1.next().unwrap().trim().to_string();
    let rhs: &str = it_1.next().unwrap().trim();
    let l: String= rhs[1..4].to_string();
    let r: String = rhs[6..9].to_string();
    nodes.insert(v.clone(), Node{v: v, l: l, r: r});
}

fn gcd(mut x: i64, mut y: i64) -> i64 {
    let mut z = x % y;
    while z != 0 {
        x = y;
        y = z;
        z = x % y;
    }
    y
}

fn multi(x: i64, y: i64) -> i64 {
    x * y / gcd(x, y)
}

fn main() {
    let lines = read_lines("input");
    let instructions = &lines[0];
    
    let mut nodes = HashMap::new();
    lines[2..].iter().for_each(|line| 
        parse_line(line, &mut nodes)
    );
    
    let ans = nodes.keys().filter(|s| s.as_bytes()[2] == b'A').map(|s| {
        let mut i = 0;
        let n = instructions.len();
        let bytes = instructions.as_bytes();
        let mut c = s;
        while c.as_bytes()[2] != b'Z' {
            match bytes[i % n] {
                b'L' => c = &nodes.get(c).unwrap().l,
                b'R' => c = &nodes.get(c).unwrap().r,
                _ => panic!(),
            }
            i += 1
        }
        i as i64
    }).fold(1 as i64, |x, y| multi(x, y));

    println!("{}", ans);
}
