use std::{fs::read_to_string};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
        
}

struct Seed {
    seed: i64,
    cur: Vec<i64>,
}

fn main() {
    let lines: Vec<String> = read_lines("input");
    let mut seeds: Vec<Seed> = lines.get(0).unwrap().split(' ').skip(1).map(|w| {
        let s = w.trim().parse::<i64>().unwrap();
        Seed{seed: s, cur: vec![s; 1]}
    }).collect();
    let mut i = lines.iter().skip(2);
    let mut gen = 0;
    while let Some(_) = i.next() {
        seeds.sort_by(|a, b| a.cur.get(gen).cmp(&b.cur.get(gen)));
        while let Some(line) = i.next() {
            let t: Vec<i64> = line.split(' ').flat_map(|n| n.parse::<i64>()).collect();
            if t.len() > 0 {
                let l = t[1];
                let r = t[1] + t[2];
                let m_l = t[0];
                let il = seeds.partition_point(|s| *s.cur.get(gen).unwrap() < l);
                let ir = seeds.partition_point(|s| *s.cur.get(gen).unwrap() < r);
                for it in il..ir {
                    if seeds[it].cur.len() != gen + 1 {
                        panic!("what")
                    }
                    let new_cur = seeds[it].cur[gen] - l + m_l;
                    seeds[it].cur.push(new_cur)                  
                }
            } else {
                break;
            }
        }
        for s in seeds.iter_mut() {
            if s.cur.len() == gen + 1 {
                s.cur.push(s.cur.last().unwrap().clone())
            }
        }
        gen += 1;
    }
    seeds.sort_by(|a, b| a.cur.get(gen).cmp(&b.cur.get(gen)));
    println!("{}", seeds[0].cur.get(gen).unwrap());
}

