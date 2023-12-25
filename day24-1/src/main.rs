use std::{
    borrow::Borrow,
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

#[derive(Clone, Copy, PartialEq, Eq)]
struct Hailstone {
    p: [i64; 3],
    v: [i64; 3]
}

const L_BOUND: f64 = 200000000000000.0;
const R_BOUND: f64 = 400000000000000.0;
//const L_BOUND: f64 = 7.0;
//const R_BOUND: f64 = 27.0;

fn main() {
    let lines = read_lines("input");
    let hailstones: Vec<Hailstone> = lines.iter().map(|line| {
        let mut i = line.split('@');
        let ps = i.next().unwrap();
        let mut p = [0, 0, 0];
        ps.split(',').enumerate().for_each(|(i, s)| p[i] = s.trim().parse().unwrap());
        let vs = i.next().unwrap();
        let mut v = [0, 0, 0];
        vs.split(',').enumerate().for_each(|(i, s)| v[i] = s.trim().parse().unwrap());
        Hailstone{p, v}
    }).collect();
    
    let mut sum = 0;
    for a in hailstones.iter() {
        for b in hailstones.iter() {
            if a == b {
                continue;
            }
            let d = a.v[0] * b.v[1] - b.v[0] * a.v[1];
            if d == 0 {
                let dp = (a.p[0] - b.p[0], a.p[1] - b.p[1]);
                if dp.0 * a.v[1] - dp.1 * a.v[0] == 0 {
                    sum += 1;
                }
            }
            let tb = (a.v[1] * (b.p[0] - a.p[0]) - a.v[0] * (b.p[1] - a.p[1])) as f64 / d as f64;
            let ta = (b.v[0] * (a.p[1] - b.p[1]) - b.v[1] * (a.p[0] - b.p[0])) as f64 / d as f64;
            if ta < 0.0 {
                continue;
            }
            if tb < 0.0 {
                continue;
            }
            let x = b.p[0] as f64 + b.v[0] as f64 * tb;
            let y = b.p[1] as f64 + b.v[1] as f64 * tb;
            if x >= L_BOUND && x <= R_BOUND {
                if y >= L_BOUND && y <= R_BOUND {
                    sum += 1;
                }
            }
        }
    }
    println!("{}", sum/2);
}
