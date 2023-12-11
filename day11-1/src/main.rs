use std::{fs::read_to_string, collections::HashMap, cmp::Ordering, collections::VecDeque, default};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
        
}

fn calc_dist(cnt: &Vec<i64>) -> i64 {
    let mut dist = 0;
    let mut n_galaxies = 0;
    let mut total_dist = 0;
    for c in cnt {
        match c {
            0 => {
                dist += 2 * n_galaxies;
            },
            _ => {
                dist += n_galaxies;
                total_dist += dist * c;
                n_galaxies += c;
            }
        }
    }
    total_dist
}

fn main() {
    let lines = read_lines("input");
    let galaxies: Vec<(usize, usize)> = lines.iter().enumerate().flat_map(|(x, line)| {
        line.as_bytes().iter().enumerate().flat_map(|(y, &c)| 
            if c == b'#' {
                Some((x, y))
            } else {
                None
            }
        ).collect::<Vec<(usize,usize)>>()
    }).collect();
    
    let mut r_cnt = vec![0; 150];
    let mut c_cnt = vec![0; 150];
    for g in galaxies {
        r_cnt[g.0] += 1;
        c_cnt[g.1] += 1;
    }

    let ans = calc_dist(&c_cnt) + calc_dist(&r_cnt);
    
    println!("{}", ans);
}
