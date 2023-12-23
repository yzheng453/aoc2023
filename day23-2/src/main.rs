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

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn try_move(
    x: usize,
    y: usize,
    d: &(i32, i32),
    map: &Vec<Vec<u8>>,
    visited: &mut Vec<Vec<bool>>,
) -> i32 {
    if let (Ok(nx), Ok(ny)) = (
        usize::try_from(x as i32 + d.0),
        usize::try_from(y as i32 + d.1),
    ) {
        if let Some(c) = map.get(nx).and_then(|r| r.get(ny)) {
            if *c == b'#' {
                return 0;
            }
            if !visited[nx][ny] {
                sch(nx, ny, visited, map)
            } else {
                0
            }
        } else {
            0
        }
    } else {
        0
    }
}

fn sch(x: usize, y: usize, visited: &mut Vec<Vec<bool>>, map: &Vec<Vec<u8>>) -> i32 {
    visited[x][y] = true;
    let max_steps = match map[x][y] {
        b'#' => -1,
        b'.' | b'^' | b'v' | b'<' | b'>' => DIRECTIONS
            .iter()
            .map(|d| try_move(x, y, d, map, visited))
            .max()
            .unwrap(),
        _ => panic!(),
    };
    visited[x][y] = false;
    if max_steps > 0 {
        max_steps + 1
    } else {
        if x + 1 == map.len() {
            1
        } else {
            -1
        }
    }
}

fn longest_hike(map: &Vec<Vec<u8>>) -> i32 {
    let mut visited: Vec<Vec<bool>> = (0..map.len())
        .map(|_| (0..map[0].len()).map(|_| false).collect())
        .collect();
    let mut max_steps = 0;
    for y in 0..map[0].len() {
        if map[0][y] != b'#' {
            let steps = sch(0, y, &mut visited, &map);
            max_steps = max_steps.max(steps);
        }
    }
    max_steps
}

fn main() {
    let lines = read_lines("input");
    let map: Vec<Vec<u8>> = lines.iter().map(|line| line.as_bytes().to_vec()).collect();
    let ans = longest_hike(&map);
    println!("{}", ans - 1);
}
