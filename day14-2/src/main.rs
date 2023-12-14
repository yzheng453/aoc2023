use std::{fs::read_to_string, collections::HashMap, cmp::Ordering, collections::VecDeque, default, slice::Iter};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
        
}

fn rotate(a: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let n = a.len();
    let m = a[0].len();
    (0..m).map(|x| 
        (0..n).map(|y| a[n-1-y][x].clone()).collect()
    ).collect()
}

fn calculate_load(a: &Vec<Vec<u8>>) -> i64 {
    let height = a[0].len();
    a.iter().flat_map(|line| 
        line.iter().enumerate().filter(|(_, &c)| c == b'O').map(|(i, _)| height as i64-i as i64)).fold(0, i64::saturating_add)
}

fn print_a(a: &Vec<Vec<u8>>) {
    a.iter().for_each(|line| {
        println!("{}", String::from_utf8(line.to_vec()).unwrap());
    });
    println!("");
}

fn main() {
    let lines = read_lines("input");
    let mut a: Vec<Vec<u8>> = lines.iter().map(|l| l.as_bytes().to_vec()).collect();
    for i in 0..3 {
        a = rotate(a);
    }
    for i in 1.. 1000{
        for j in 0..4 {
            a.iter_mut().for_each(|line| {
                let mut last = 0;
                for x in 0..line.len() {
                    match line[x] {
                        b'O' => {
                            if last != x {
                                line[last] = b'O';
                                line[x] = b'.';
                            }
                            last += 1;
                        },
                        b'.' => (),
                        b'#' => {
                            last = x + 1;
                        },
                        _ => panic!(),
                    }
                }
            });
            a = rotate(a);
        }
        //if i % 10000 == 0 {
            println!("{} {}", i, calculate_load(&a));
        //}
    }

    println!("{}", calculate_load(&a));
}
