use std::{fs::read_to_string, collections::HashMap, cmp::Ordering, collections::VecDeque, default, slice::Iter};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
        
}

fn calc_line(line: &String) -> i64 {
    let mut i_space = line.find(' ').unwrap();
    let springs_single: &[u8] = &(line.as_bytes())[0..i_space];
    let mut springs = springs_single.to_vec();
    (0..4).for_each(|_| {
        springs.push(b'?');
        springs.append(&mut springs_single.to_vec());
    });
    springs.push(b'.');
    let mut it = line.split(' ');
    it.next();
    let nums_single: Vec<usize> = it.next().unwrap().split(',').map(|s| s.parse().unwrap()).collect();
    let nums: Vec<usize> = (0..5).flat_map(|_| nums_single.to_vec()).collect();
    let mut f: Vec<Vec<i64>> = (0..=springs.len()).map(|_| (0..=nums.len()).map(|_| 0).collect()).collect();
    f[0][0] = 1;
    for i in 0..springs.len() {
        for j in 0..=nums.len() {
            if f[i][j] == 0 {
                continue;
            }
            let current = springs[i];
            if current != b'#' {
                f[i+1][j] += f[i][j];
            }
            if (j < nums.len()) && (current == b'#' || current == b'?') {
                let can_match_broken = (i..i+nums[j]).map(|k| springs.get(k).unwrap_or(&b' ')).all(|&c| c == b'#' || c == b'?');
                let &next_s = springs.get(i + nums[j]).unwrap_or(&b'#');
                let can_match_next = next_s != b'#';
                if can_match_broken && can_match_next {
                    f[i + nums[j] + 1][j+1] += f[i][j];
                }
            }
        }
    }
    
    f[springs.len()][nums.len()]
}

fn main() {
    let lines = read_lines("input");
    let ans = lines.iter().map(calc_line).fold(0, i64::saturating_add);
    println!("{}", ans);
}
