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
        b'3' => (-1, 0),
        b'1' => (1, 0),
        b'2' => (0, -1),
        b'0' => (0, 1),
        _ => panic!()
    }
}

fn main() {
    let lines = read_lines("input");
    let plan: Vec<(u8, i32)> = lines.iter().map(|line| {
        let mut it = line.split(' ').skip(2);
        let s = &(it.next().unwrap().as_bytes())[2..];
        let d = s[5];
        let l = i32::from_str_radix(std::str::from_utf8(&s[0..5]).unwrap(), 16).unwrap();
        (d, l)
    }).collect();
    let mut current = (0, 0);
    let mut xs = Vec::new();
    xs.push(0); xs.push(i32::MIN); xs.push(i32::MAX);
    let mut ys = Vec::new();
    ys.push(0); ys.push(i32::MIN); ys.push(i32::MAX);
    for dig in plan.iter() {
        let d = parse_d(dig.0);
        current.0 += d.0 * dig.1;
        current.1 += d.1 * dig.1; 
        xs.push(current.0);
        xs.push(current.0 + 1);
        ys.push(current.1);
        ys.push(current.1 + 1);
    }
    xs.sort(); xs.dedup();
    ys.sort(); ys.dedup();
    let map_x:HashMap<i32, usize> = xs.iter().enumerate().map(|(i, &v)| (v, i)).collect();
    let map_y:HashMap<i32, usize> = ys.iter().enumerate().map(|(i, &v)| (v, i)).collect();

    let mut map: Vec<Vec<bool>> = xs.iter().map(|_| ys.iter().map(|_| false).collect()).collect();
    let mut out: Vec<Vec<bool>> = xs.iter().map(|_| ys.iter().map(|_| false).collect()).collect();

    let mut current = (0, 0);
    for (d, l) in plan.iter() {
        current = match d {
            b'3' => {
                let x = current.0 - l;
                for i in *map_x.get(&x).unwrap() ..= *map_x.get(&current.0).unwrap() {
                    map[i][*map_y.get(&current.1).unwrap()] = true;                    
                }
                (x, current.1)
            },
            b'1' => {
                let x = current.0 + l;
                for i in *map_x.get(&current.0).unwrap() ..= *map_x.get(&x).unwrap() {
                    map[i][*map_y.get(&current.1).unwrap()] = true;                    
                }
                (x, current.1)
            },
            b'2' => {
                let y = current.1 - l;
                for i in *map_y.get(&y).unwrap() ..= *map_y.get(&current.1).unwrap() {
                    map[*map_x.get(&current.0).unwrap()][i] = true;                    
                }
                (current.0, y)
            },
            b'0' => {
                let y = current.1 + l;
                for i in *map_y.get(&current.1).unwrap() ..= *map_y.get(&y).unwrap() {
                    map[*map_x.get(&current.0).unwrap()][i] = true;                    
                }
                (current.0, y)
            },
            _ => panic!(),
        };
    }

    let mut deq = VecDeque::new();
    deq.push_front((*map_x.get(&0).unwrap() as i32, *map_y.get(&0).unwrap() as i32));
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
    let mut ans: i64 = 0;
    for (i, row) in out.iter().enumerate() {
        let mut line_sum = 0;
        for (j, f) in row.iter().enumerate() {
            //print!("{}", if *f {1} else {0});
            if !f {
                let dy = ys[j+1] - ys[j];
                line_sum += dy as i64;
            }
        }
        if i < xs.len() - 1 {
            let dx = xs[i+1] as i64 - xs[i] as i64;
            ans += line_sum * dx as i64;
            //println!(" {} {}", dx, line_sum);
        }
    }
    //println!("");

    println!("{}", ans);
}
