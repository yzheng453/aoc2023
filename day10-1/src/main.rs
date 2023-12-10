use std::{fs::read_to_string, collections::HashMap, cmp::Ordering, collections::VecDeque, default};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
        
}

struct Move([u8; 4], (i32, i32), [u8; 4]);
const TOP_SHAPES: [u8; 4] = [b'|', b'L', b'J', b'S'];
const DOWN_SHAPES: [u8; 4] = [b'|', b'7', b'F', b'S'];
const LEFT_SHAPES: [u8; 4] = [b'-', b'7', b'J', b'S'];
const RIGHT_SHAPES: [u8; 4] = [b'-', b'L', b'F', b'S'];

const TOP: Move = Move(TOP_SHAPES, (-1, 0), DOWN_SHAPES);
const DOWN: Move = Move(DOWN_SHAPES, (1, 0), TOP_SHAPES);
const LEFT: Move = Move(LEFT_SHAPES, (0, -1), RIGHT_SHAPES);
const RIGHT: Move = Move(RIGHT_SHAPES, (0, 1), LEFT_SHAPES);

const MOVES: [Move; 4] = [TOP, DOWN, LEFT, RIGHT];

fn main() {
    let lines = read_lines("input");

    let map: Vec<Vec<u8>> = lines.iter().map(|line| line.as_bytes().to_vec()).collect();
    let (x, y) = map.iter().enumerate().flat_map(|(i, row)| row.iter().enumerate().filter(|(j, &b)| b == b'S').next().map(|(j, _)|(i, j))).next().unwrap();
    let mut dist: Vec<Vec<i64>> = map.iter().map(|l| l.iter().map(|_| -1).collect()).collect();
    dist[x][y] = 0;
    let mut deq = VecDeque::new();
    deq.push_back((x, y));
    while let Some((x, y)) = deq.pop_front() {
        let c = map[x][y];
        let nd = dist[x][y] + 1;
        for m in MOVES {
            if m.0.contains(&c) {
                match (usize::try_from(x as i32 + m.1.0), usize::try_from(y as i32 + m.1.1)) {
                    (Ok(nx), Ok(ny)) => {
                        if let Some(current_d) = dist.get_mut(nx).and_then(|dist_x| dist_x.get_mut(ny)) {
                            let s = map[nx][ny];
                            if m.2.contains(&s) && (nd < *current_d || *current_d < 0){
                                *current_d = nd;
                                deq.push_back((nx, ny));
                            }
                        }
                    },
                    _ => (),
                }
            }
        }
    }
    let max = dist.iter().map(|d_row| d_row.iter().fold(0_i64, |a, &b| a.max(b))).fold(0_i64, |a, b| a.max(b));
    
    /*
    for d_row in dist {
        for d in d_row {
            print!("{} ", d);
        }
        println!("");
    }
*/

    println!("{}", max);
}
