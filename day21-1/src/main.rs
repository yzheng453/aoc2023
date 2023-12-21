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

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

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
    
    let mut plots: Vec<HashSet<(usize, usize)>> = Vec::new();
    plots.push(HashSet::from([(sx, sy)]));
    let n_iter = 64;
    for i in 0..n_iter {
        let plot = plots.last().unwrap();
        let mut n_plot = HashSet::new();
        for (x, y) in plot.iter() {
            for (dx, dy) in DIRECTIONS {
                let nx = *x as i32 + dx;
                let ny = *y as i32 + dy;
                if let Ok(nx) = usize::try_from(nx) {
                    if let Ok(ny) = usize::try_from(ny) {
                        if let Some(b) = map.get(nx).and_then(|row| row.get(ny)) {
                            if *b != b'#' {
                                n_plot.insert((nx, ny));
                            }
                        }
                    }
                }
            }
        }
        plots.push(n_plot);
    }
    println!("{}", plots.last().unwrap().len());
}
