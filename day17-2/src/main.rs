use std::{fs::read_to_string, collections::HashMap, cmp::Ordering, collections::VecDeque, default, slice::Iter};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
        
}

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

fn get_min(d: &[[i32; 11]; 5], m: usize) -> i32 {
    let mut min = i32::MAX;
    for i in 0..4 {
        for j in m..11 {
            if d[i][j] < min {
                min = d[i][j];
            }
        }
    }
    min
}

fn main() {
    let lines = read_lines("input");
    let map: Vec<Vec<u8>> = lines.iter().map(|line| {
        line.as_bytes().to_vec()
    }).collect();

    let mut dist = Vec::new();
    for i in 0..map.len() {
        let d_r: Vec<[[i32; 11]; 5]> = map[i].iter().map(|_| [[i32::MAX; 11]; 5]).collect();
        dist.push(d_r);
    }
    let mut q: VecDeque<(usize, usize, usize, usize)> = VecDeque::new();
    q.push_back((0, 0, 4, 0));
    dist[0][0][4][0] = 0;
    while let Some(c) = q.pop_front() {
        for i in 0..4 {
            if (c.3 < 4) && (i != c.2) && (c.2 != 4) {
                continue;
            }
            if i + c.2 == 3 {
                continue;
            }
            let x = c.0 as i32 + DIRECTIONS[i as usize].0;
            let y = c.1 as i32 + DIRECTIONS[i as usize].1;
            let n_steps = if i == c.2 { c.3 + 1 } else { 1 };
            let current_d = dist[c.0][c.1][c.2][c.3];
            if n_steps > 10 {
                continue;
            }
            if let Ok(x) = usize::try_from(x) {
                if let Ok(y) = usize::try_from(y) {
                    if let Some(t) = dist.get_mut(x).and_then(|d| d.get_mut(y)) {
                        let v = map[x][y] - b'0';
                        let n_d =  current_d + v as i32;
                        if n_d < t[i][n_steps] {
                            t[i][n_steps] = n_d;
                            q.push_back((x, y, i, n_steps));
                        }
                    }
                }
            }
        }        
    }
    
    /*
    for row in dist.iter() {
        for d in row.iter() {
            print!("{} ", get_min(d, 0));
        }
        println!("");
    }
    */

    println!("{}", get_min(&dist[map.len() - 1][map[0].len() - 1], 4));
}
