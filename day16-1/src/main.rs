use std::{fs::read_to_string, collections::HashMap, cmp::Ordering, collections::VecDeque, default, slice::Iter};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
        
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Direction(i32, i32, usize);

const UP: Direction = Direction(-1, 0, 0);
const DOWN: Direction = Direction(1, 0, 1);
const LEFT: Direction = Direction(0, -1, 2);
const RIGHT: Direction = Direction(0, 1, 3);

struct BeamPoint{
    p: (usize, usize),
    d: Direction,
}

fn mv(x: usize, y: usize, d: Direction, a: &mut VecDeque<BeamPoint>, energized: &mut Vec<Vec<[bool; 4]>>) {
    let x = x as i32 + d.0;
    let y = y as i32 + d.1;
    if let Ok(xu) = usize::try_from(x) {
        if let Ok(yu) = usize::try_from(y) {
            if let Some(flags) = energized.get_mut(xu).and_then(|m| m.get_mut(yu)) {
                if flags[d.2] == false {
                    flags[d.2] = true;
                    a.push_back(BeamPoint{p: (xu, yu), d: d});            
                }
            }
        }
    }
}


fn main() {
    let lines = read_lines("input");
    let map: Vec<Vec<u8>> = lines.iter().map(|line| {
        line.as_bytes().to_vec()
    }).collect();
    let mut a: VecDeque<BeamPoint> = VecDeque::new();
    let mut energized: Vec<Vec<[bool; 4]>> = (0..map.len()).map(|_| (0..map[0].len()).map(|_| [false, false, false, false]).collect()).collect();
    energized[0][0][RIGHT.2] = true;
    a.push_back(BeamPoint{
        p: (0, 0),
        d: RIGHT,
    });
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
        println!("");
        row.iter()
            .map(|flags| 
                if flags[0] || flags[1] || flags[2] || flags[3] {
                    print!("#");
                    1
                } else {
                    print!(".");
                    0
                }
            ).fold(0, i32::saturating_add)
        }).fold(0, i32::saturating_add);
    println!("{}", ans);
}
