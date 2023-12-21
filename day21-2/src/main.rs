use std::{
    cmp::Ordering,
    collections::{btree_map::Range, HashMap, HashSet},
    collections::{BTreeMap, VecDeque},
    convert::identity,
    default,
    fs::read_to_string,
    slice::Iter,
};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

const DIRECTIONS: [(i64, i64); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn main() {
    let lines = read_lines("input");
    let map: Vec<&[u8]> = lines.iter().map(|line| line.as_bytes()).collect();
    let (sx, sy) = map.iter().enumerate().flat_map(|(x, &r)| {
        if let Some((y, _)) = r.iter().enumerate().find(|(y, &b)| b == b'S') {
            Some((x, y))
        } else {
            None
        }
    }).next().unwrap();
    let n = map.len() as i64;
    let m = map[0].len() as i64;
    
    let mut plots: Vec<HashSet<(i64, i64)>> = Vec::new();
    plots.push(HashSet::from([(sx as i64, sy as i64)]));
    let n_iter = 5000;
    for i in 0..n_iter {
        let plot = plots.last().unwrap();
        let mut n_plot = HashSet::new();
        for (x, y) in plot.iter() {
            for (dx, dy) in DIRECTIONS {
                let nx = *x + dx;
                let ny = *y + dy;
                let mx = nx.rem_euclid(n) as usize;
                let my = ny.rem_euclid(m) as usize;
                let b = map[mx][my];
                if b != b'#' {
                    n_plot.insert((nx, ny));
                }
            }
        }
        println!("{} {}", i, n_plot.len());
        plots.push(n_plot);
    }
}
