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

enum TypeOfRock {
    X,
    Y,
    Z,
}

use TypeOfRock::*;

struct Rock {
    t: TypeOfRock,
    p: (usize, usize, usize),
    l: usize,
}

fn parse_rock(line: &String) -> Rock {
    let mut it = line.split('~');
    let p0: Vec<usize> = it
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let p1: Vec<usize> = it
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    if p0[0] != p1[0] {
        Rock {
            t: X,
            p: (p0[0], p0[1], p0[2]),
            l: p1[0] - p0[0] + 1,
        }
    } else if p0[1] != p1[1] {
        Rock {
            t: Y,
            p: (p0[0], p0[1], p0[2]),
            l: p1[1] - p0[1] + 1,
        }
    } else {
        Rock {
            t: Z,
            p: (p0[0], p0[1], p0[2]),
            l: p1[2] - p0[2] + 1,
        }
    }
}

enum Slot {
    Empty,
    Earth,
    Occupied { id: usize },
}

use Slot::*;

fn main() {
    let lines = read_lines("input");
    let mut rocks: Vec<Rock> = lines.iter().map(parse_rock).collect();
    let mut map: Vec<Vec<Vec<Slot>>> = Vec::new();
    for z in 0..350 {
        let mut level = Vec::new();
        for x in 0..10 {
            let mut row = Vec::new();
            for y in 0..10 {
                let s = if z == 0 { Earth } else { Empty };
                row.push(s);
            }
            level.push(row);
        }
        map.push(level);
    }
    for (id, rock) in rocks.iter().enumerate() {
        match rock.t {
            X => {
                for d in 0..rock.l {
                    map[rock.p.2][rock.p.0 + d][rock.p.1] = Occupied { id: id }
                }
            }
            Y => {
                for d in 0..rock.l {
                    map[rock.p.2][rock.p.0][rock.p.1 + d] = Occupied { id: id }
                }
            }
            Z => {
                for d in 0..rock.l {
                    map[rock.p.2 + d][rock.p.0][rock.p.1] = Occupied { id: id }
                }
            }
        }
    }

    loop {
        let mut rock_fell = false;
        for (id, rock) in rocks.iter_mut().enumerate() {
            loop {
                let has_no_support = match rock.t {
                    X => (0..rock.l).all(|d| matches!(map[rock.p.2 - 1][rock.p.0 + d][rock.p.1], Empty)),
                    Y => (0..rock.l).all(|d| matches!(map[rock.p.2 - 1][rock.p.0][rock.p.1 + d], Empty)),
                    Z => matches!(map[rock.p.2 - 1][rock.p.0][rock.p.1], Empty),
                };
                if !has_no_support {
                    break;
                }
                match rock.t {
                    X => {
                        (0..rock.l).for_each(|d| map[rock.p.2][rock.p.0 + d][rock.p.1] = Empty);
                        (0..rock.l).for_each(|d| map[rock.p.2 - 1][rock.p.0 + d][rock.p.1] = Occupied { id: id });
                    }, 
                    Y => {
                        (0..rock.l).for_each(|d| map[rock.p.2][rock.p.0][rock.p.1 + d] = Empty);
                        (0..rock.l).for_each(|d| map[rock.p.2 - 1][rock.p.0][rock.p.1 + d] = Occupied { id: id });
                    },
                    Z => {
                        map[rock.p.2 + rock.l - 1][rock.p.0][rock.p.1] = Empty;
                        map[rock.p.2 - 1][rock.p.0][rock.p.1] = Occupied { id: id };
                    }
                }
                rock.p.2 -= 1;
                rock_fell = true;
            }
        }
        if !rock_fell {
            break;
        }
    }
    
    let supporting: Vec<HashSet<usize>> = rocks.iter().map(|rock| {
        match rock.t {
            X => (0..rock.l).flat_map(|d| if let Occupied{id} = map[rock.p.2 - 1][rock.p.0 + d][rock.p.1] {Some(id)} else {None}).collect(),
            Y => (0..rock.l).flat_map(|d| if let Occupied{id} = map[rock.p.2 - 1][rock.p.0][rock.p.1 + d] {Some(id)} else {None} ).collect(),
            Z => if let Occupied{id} = map[rock.p.2 - 1][rock.p.0][rock.p.1] {HashSet::from([id])} else {HashSet::new()},
        }
    }).collect();
    let ans = rocks.iter().filter(|&rock| {
        match rock.t {
            X => (0..rock.l).all(|d| if let Occupied{id} = map[rock.p.2 + 1][rock.p.0 + d][rock.p.1] {supporting[id].len() > 1} else {true}),
            Y => (0..rock.l).all(|d| if let Occupied{id} = map[rock.p.2 + 1][rock.p.0][rock.p.1 + d] {supporting[id].len() > 1} else {true}),
            Z => if let Occupied{id} = map[rock.p.2 + rock.l][rock.p.0][rock.p.1] {supporting[id].len() > 1} else {true},
        }
    }).count();
    println!("{}", ans);
}
