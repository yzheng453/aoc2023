use std::{fs::read_to_string, collections::HashMap, cmp::Ordering, collections::VecDeque, default, slice::Iter};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
        
}

#[derive(Clone, Copy)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

use Direction::*;

fn dx(d: Direction) -> i32 {
    match d {
        UP => -1,
        DOWN => 1,
        LEFT => 0,
        RIGHT => 0,
    }
}

fn dy(d: Direction) -> i32  {
    match d {
        LEFT => -1,
        RIGHT => 1,
        UP | DOWN => 0,
    }
}

fn id(d: Direction) -> usize {
    match d {
        UP => 0,
        DOWN => 1,
        LEFT => 2,
        RIGHT => 3,
    }
}

struct BeamPoint{
    p: (usize, usize),
    d: Direction,
}

fn mv(x: usize, y: usize, d: Direction, a: &mut VecDeque<BeamPoint>, energized: &mut Vec<Vec<[bool; 4]>>) {
    let x = x as i32 + dx(d);
    let y = y as i32 + dy(d);
    if let Ok(xu) = usize::try_from(x) {
        if let Ok(yu) = usize::try_from(y) {
            if let Some(flags) = energized.get_mut(xu).and_then(|m| m.get_mut(yu)) {
                if flags[id(d)] == false {
                    flags[id(d)] = true;
                    a.push_back(BeamPoint{p: (xu, yu), d: d});            
                }
            }
        }
    }
}

fn calc_for_starting_point(s: BeamPoint, map: &Vec<Vec<u8>>) -> i32 {
    let mut energized: Vec<Vec<[bool; 4]>> = (0..map.len()).map(|_| (0..map[0].len()).map(|_| [false, false, false, false]).collect()).collect();
    energized[s.p.0][s.p.1][id(s.d)] = true;
    let mut a: VecDeque<BeamPoint> = VecDeque::new();
    a.push_back(s);
    while let Some(c) = a.pop_front() {
        let BeamPoint{p: (x, y), d} = c;
        match (map[x as usize][y as usize], d) {
            (b'.', _) | (b'-', LEFT) | (b'-', RIGHT) | (b'|', UP) | (b'|', DOWN)=> {
                mv(x, y, d, &mut a, &mut energized);
            },
            (b'/', LEFT) => {
                mv(x, y, DOWN, &mut a, &mut energized);
            },
            (b'/', UP) => {
                mv(x, y, RIGHT, &mut a, &mut energized);
            },
            (b'/', RIGHT) => {
                mv(x, y, UP, &mut a, &mut energized);
            },
            (b'/', DOWN) => {
                mv(x, y, LEFT, &mut a, &mut energized);
            },
            (b'\\', LEFT) => {
                mv(x, y, UP, &mut a, &mut energized);
            },
            (b'\\', UP) => {
                mv(x, y, LEFT, &mut a, &mut energized);
            },
            (b'\\', RIGHT) => {
                mv(x, y, DOWN, &mut a, &mut energized);
            },
            (b'\\', DOWN) => {
                mv(x, y, RIGHT, &mut a, &mut energized);
            },
            (b'-', DOWN) | (b'-', UP) => {
                mv(x, y, LEFT, &mut a, &mut energized);
                mv(x, y, RIGHT, &mut a, &mut energized);
            },
            (b'|', LEFT) | (b'|', RIGHT) => {
                mv(x, y, UP, &mut a, &mut energized);
                mv(x, y, DOWN, &mut a, &mut energized);
            },
            _ => panic!(),
        }
    }
    let ans = energized.iter().map(|row| {
        row.iter()
            .map(|flags| 
                if flags[0] || flags[1] || flags[2] || flags[3] {
                    1
                } else {
                    0
                }
            ).fold(0, i32::saturating_add)
        }).fold(0, i32::saturating_add);
    ans
}

fn main() {
    let lines = read_lines("input");
    let map: Vec<Vec<u8>> = lines.iter().map(|line| {
        line.as_bytes().to_vec()
    }).collect();
    let mut max = 0;
    let n = map.len();
    let m = map[0].len();
    for i in 0..n {
        let a = calc_for_starting_point(BeamPoint{p: (i, 0), d: RIGHT}, &map);
        if a > max {
            max = a;
        }
        let a = calc_for_starting_point(BeamPoint{p: (i, m-1), d: LEFT}, &map);
        if a > max {
            max = a;
        }
    }
    for i in 0..m {
        let a = calc_for_starting_point(BeamPoint{p: (0, i), d: DOWN}, &map);
        if a > max {
            max = a;
        }
        let a = calc_for_starting_point(BeamPoint{p: (n-1, i), d: UP}, &map);
        if a > max {
            max = a;
        }
    }
    println!("{}", max);
}
