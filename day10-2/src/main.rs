use std::{fs::read_to_string, collections::HashMap, cmp::Ordering, collections::VecDeque, default};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
        
}

struct Shape([[u8; 3]; 3]);
const SHAPE_S: Shape = Shape([[0, 1, 0], [1, 1, 1], [0, 1, 0]]);
const SHAPE_H: Shape = Shape([[0, 1, 0], [0, 1, 0], [0, 1, 0]]);
const SHAPE_V: Shape = Shape([[0, 0, 0], [1, 1, 1], [0, 0, 0]]);
const SHAPE_L: Shape = Shape([[0, 1, 0], [0, 1, 1], [0, 0, 0]]);
const SHAPE_J: Shape = Shape([[0, 1, 0], [1, 1, 0], [0, 0, 0]]);
const SHAPE_7: Shape = Shape([[0, 0, 0], [1, 1, 0], [0, 1, 0]]);
const SHAPE_F: Shape = Shape([[0, 0, 0], [0, 1, 1], [0, 1, 0]]);
const SHAPE_D: Shape = Shape([[0, 0, 0], [0, 1, 0], [0, 0, 0]]);

const MOVES:[[i32; 2]; 4] = [[1, 0], [-1, 0], [0, 1], [0, -1]];

fn main() {
    let lines = read_lines("input");
    
    let shape_map = HashMap::from([
        (b'S', SHAPE_S),
        (b'|', SHAPE_H),
        (b'-', SHAPE_V),        
        (b'L', SHAPE_L),
        (b'J', SHAPE_J),
        (b'7', SHAPE_7),
        (b'F', SHAPE_F),
        (b'.', SHAPE_D),
    ]);

    let map: Vec<Vec<u8>> = lines.iter().flat_map(|line| {
        let mut v: Vec<Vec<u8>> = (0..3).map(|_| Vec::new()).collect();
        line.as_bytes().iter().map(|b| shape_map.get(b).unwrap()).for_each(|s| {
            v[0].append(&mut s.0[0].to_vec());
            v[1].append(&mut s.0[1].to_vec());
            v[2].append(&mut s.0[2].to_vec());
        });
        v
    }).collect();
    let mut flag: Vec<Vec<i64>> = map.iter().map(|l| l.iter().map(|_| 0).collect()).collect();
    let mut deq = VecDeque::new();
    let n = map.len();
    let m = map[0].len();
    for i in 0..n {
        flag[i][0] = 1;
        deq.push_back((i, 0));
        flag[i][m-1] = 1;
        deq.push_back((i, m-1));
    }
    for j in 0..m {
        flag[0][j] = 1;
        deq.push_back((0, j));
        flag[n-1][j] = 1;
        deq.push_back((n-1, j));
    }
    while let Some((x, y)) = deq.pop_front() {
        for m in MOVES {
            match (usize::try_from(x as i32 + m[0]), usize::try_from(y as i32 + m[1])) {
                (Ok(nx), Ok(ny)) => {
                    if let Some(current_f) = flag.get_mut(nx).and_then(|dist_x| dist_x.get_mut(ny)) {
                        if (*current_f == 0) && (map[nx][ny] == 0 ) {
                            *current_f = 1;
                            deq.push_back((nx, ny));
                        }
                    }
                },
                _ => (),
            }
        }
    }
    let mut cnt = 0;

    for i in 0..n {
        if i % 3 != 1 {
            continue;
        }
        for j in 0..m {
            if j % 3 != 1 {
                continue;
            }
            if flag[i-1][j-1] == 0 && flag[i-1][j+1] == 0 && flag[i+1][j-1] == 0 && flag[i+1][j+1] == 0 {
                cnt += 1;
            }
        }
    }

    println!("{}", cnt);
}
