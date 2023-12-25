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

#[derive(PartialEq, Eq, Hash)]
struct Edge<'a> {
    l: &'a str,
    r: &'a str,
}

fn main() {
    let lines = read_lines("input");
    let edges: Vec<Edge> = lines.iter().flat_map(|line| {
        let mut it = line.split(':');
        let l = it.next().unwrap();
        let rights: Vec<&str> = it.next().unwrap().trim().split(' ').collect();
        rights.iter().map(|&r| Edge{l, r}).collect::<Vec<Edge>>()
    }).collect();
    let mut m: HashMap<&str, HashMap<&str, &Edge>> = HashMap::new();
    edges.iter().for_each(|e| {
        let l = e.l;
        let r = e.r;

        if !m.contains_key(l) {
            m.insert(l, HashMap::new());
        }
        if !m.contains_key(r) {
            m.insert(r, HashMap::new());
        }
        m.get_mut(l).unwrap().insert(r, e);
        m.get_mut(r).unwrap().insert(l, e);
    });

    let mut it = m.keys();
    let s = *it.next().unwrap();
    for &t in it {
        let mut flow: HashMap<&Edge, i32> = edges.iter().map(|e| (e, 0)).collect();
        let mut visited;
        loop {
            let mut augmented = false;
            visited = HashMap::new();
            visited.insert(s, &edges[0]);
            let mut deq = VecDeque::from([s]);
            while let Some(p) = deq.pop_front() {
                if p == t {
                    augmented = true;
                    let mut cursor = p;
                    while cursor != s {
                        let &e = visited.get(cursor).unwrap();
                        if e.r == cursor {
                            // forward
                            *flow.get_mut(e).unwrap() += 1;
                            cursor = e.l;
                        } else {
                            // backward
                            *flow.get_mut(e).unwrap() -= 1;
                            cursor = e.r;
                        }
                    }
                    break;
                }
                let es = m.get(p).unwrap();
                for &e in es.values() {
                    if e.l == p {
                        // forward
                        if (*flow.get(e).unwrap() < 1) && !visited.contains_key(e.r) {
                            visited.insert(e.r, e);
                            deq.push_back(e.r);
                        } 
                    } else {
                        // backward
                        if (*flow.get(e).unwrap() > -1) && !visited.contains_key(e.l) {
                            visited.insert(e.l, e);
                            deq.push_back(e.l);
                        } 
                    }
                }
            }
            if !augmented {
                break;
            }
        }
        let total_flow: i32 = m.get(s).unwrap().values().map(|&e| {
            let f = *flow.get(e).unwrap();
            if e.l == s {f}
            else {-f}
        }).sum();
        if total_flow == 3 {
            let size_a = visited.len();
            let size_b = m.len() - size_a;
            println!("{} {} {}", size_a, size_b, size_a * size_b);
            break;
        }
    }
}
