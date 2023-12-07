use std::{fs::read_to_string};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
        
}

#[derive(Clone, Copy)]
struct Seed {
    l: i64,
    r: i64,
    mapped: bool,
}

fn main() {
    let lines: Vec<String> = read_lines("input");
    let mut seed_iter = lines.get(0).unwrap().split(' ').skip(1).map(|w| w.trim().parse::<i64>().unwrap());
    let mut seeds: Vec<Seed> = Vec::new();
    while let Some(l) = seed_iter.next() {
        let s = seed_iter.next().unwrap();
        seeds.push(Seed{l: l, r: l + s, mapped: false});
    }
    let mut i = lines.iter().skip(2);
    let mut gen = 0;
    while let Some(_) = i.next() {
        seeds.sort_by(|a, b| a.l.cmp(&b.l));
        let mut seeds_n = Vec::new();
        while let Some(line) = i.next() {
            let t: Vec<i64> = line.split(' ').flat_map(|n| n.parse::<i64>()).collect();
            if t.len() > 0 {
                let l = t[1];
                let r = t[1] + t[2];
                let m_l = t[0];
                let il = seeds.partition_point(|s| s.r < l);
                let ir = seeds.partition_point(|s| s.l < r);
                for it in il..ir {
                    if seeds[it].mapped {
                        panic!("what")
                    }
                    if (seeds[it].l >= l) && (seeds[it].r <=r) {
                        let nl = seeds[it].l - l + m_l;
                        let nr = seeds[it].r - l + m_l;
                        seeds_n.push(Seed{l: nl, r: nr, mapped: false});
                        seeds[it].mapped = true;
                    }
                    if (seeds[it].l >= l) && (seeds[it].r > r) {
                        let nl = seeds[it].l - l + m_l;
                        let nr = r - l + m_l;
                        seeds_n.push(Seed{l: nl, r: nr, mapped: false});
                        seeds[it].l = r;
                    }
                    if (seeds[it].l < l) && (seeds[it].r <= r) {
                        let nl = m_l;
                        let nr = seeds[it].r - l + m_l;
                        seeds_n.push(Seed{l: nl, r: nr, mapped: false});
                        seeds[it].r = l;
                    }
                }
            } else {
                break;
            }
        }
        for s in seeds.iter_mut() {
            if !s.mapped {
                seeds_n.push(s.clone());
            }
        }
        seeds = seeds_n;
        gen += 1;
    }
    seeds.sort_by(|a, b| a.l.cmp(&b.l));
    println!("{}", seeds[0].l);
}

