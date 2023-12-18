use std::{fs::read_to_string, collections::HashMap, cmp::Ordering, collections::VecDeque, default, slice::Iter};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
        
}

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

fn parse_d(c: u8) -> (i32, i32) {
    match c {
        b'U' => (-1, 0),
        b'D' => (1, 0),
        b'L' => (0, -1),
        b'R' => (0, 1),
        _ => panic!()
    }
}

fn main() {
    let lines = read_lines("input");
    let plan: Vec<(u8, i32, &str)> = lines.iter().map(|line| {
        let mut it = line.split(' ');
        let d = it.next().unwrap().as_bytes()[0];
        let l = it.next().unwrap().parse().unwrap();
        let s = it.next().unwrap();
        (d, l, s)
    }).collect();
    let mut current = (0, 0);
    let mut max = (0, 0);
    let mut min = (0, 0);
    for dig in plan.iter() {
        let d = parse_d(dig.0);
        current.0 += d.0 * dig.1;
        current.1 += d.1 * dig.1; 
        max.0 = max.0.max(current.0);
        max.1 = max.1.max(current.1);
        min.0 = min.0.min(current.0);
        min.1 = min.1.min(current.1);
    }
    min.0 -= 1;
    min.1 -= 1;
    max.0 += 1;
    max.1 += 1;
    let start = (0 - min.0, 0 - min.1);
    let mut map: Vec<Vec<bool>> = (min.0 ..= max.0).map(|_| (min.1 ..= max.1).map(|_| false).collect()).collect();
    let mut out: Vec<Vec<bool>> = (min.0 ..= max.0).map(|_| (min.1 ..= max.1).map(|_| false).collect()).collect();

    let mut current = start.clone();
    for dig in plan.iter() {
        let d = parse_d(dig.0);
        match d {
            (0, x) if x > 0 => for i in 1..=(x*dig.1) {
                map[current.0 as usize][(current.1 + i) as usize] = true;
            },
            (0, x) if x < 0 => for i in (x*dig.1)..0 {
                map[current.0 as usize][(current.1 + i) as usize] = true;
            },
            (x, 0) if x > 0 => for i in 1..=(x*dig.1) {
                map[(current.0 + i) as usize][current.1 as usize] = true;
            },
            (x, 0) if x < 0 => for i in (x*dig.1)..0 {
                map[(current.0 + i) as usize][current.1 as usize] = true;
            },
            _ => panic!(),
        }
        current.0 += d.0 * dig.1;
        current.1 += d.1 * dig.1; 
    }

    let mut deq = VecDeque::new();
    deq.push_front((0, 0));
    out[0][0] = true;
    while let Some(p) = deq.pop_back() {
        for d in DIRECTIONS {
            let x = p.0 + d.0;
            let y = p.1 + d.1;
            if let Ok(x) = usize::try_from(x) {
                if let Ok(y) = usize::try_from(y) {
                    if let Some(o) = out.get_mut(x).and_then(|r| r.get_mut(y)) {
                        if (!*o) && (map[x][y] == false) {
                            *o = true;
                            deq.push_front((x as i32, y as i32));
                        }
                    }
                }
            }
        }
    }
    let ans = out.iter().map(|r| r.iter()
    .map(|c|
        if *c {
            0
        } else {
            1
        }
    ).fold(0, i32::saturating_add)).fold(0, i32::saturating_add);

    println!("{}", ans);
}
