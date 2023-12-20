use std::{
    cmp::Ordering,
    collections::{btree_map::Range, HashMap},
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

enum Module<'a> {
    FlipFlop {
        name: &'a [u8],
        state: FlipFlopState,
    },
    Conjunction {
        name: &'a [u8],
        state: ConjunctionState<'a>,
    },
    Broadcast {
        name: &'a [u8],
    },
}

use Module::*;

struct FlipFlopState {
    on_off: bool,
}

struct ConjunctionState<'a> {
    memory: BTreeMap<&'a [u8], bool>,
}

fn get_state(m: &Module) -> Vec<bool> {
    match m {
        FlipFlop { state: s, .. } => {
            vec![s.on_off; 1]
        }
        Conjunction { state: s, .. } => s.memory.values().map(|b| b.clone()).collect(),
        _Broadcast => [].to_vec(),
    }
}

fn main() {
    let lines = read_lines("input");
    let mut modules = BTreeMap::new();
    let mut outputs = BTreeMap::new();
    lines.iter().for_each(|line| {
        let mut it = line.split("->");
        let s_name = it.next().unwrap().trim().as_bytes();
        let output: Vec<&[u8]> = it
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.trim().as_bytes())
            .filter(|s| s.len() > 0)
            .collect();
        let (name, module) = match &s_name[0] {
            b'%' => {
                let name = &s_name[1..];
                (
                    name,
                    FlipFlop {
                        name: name,
                        state: FlipFlopState { on_off: false },
                    },
                )
            }
            b'&' => {
                let name = &s_name[1..];
                (
                    name,
                    Conjunction {
                        name: name,
                        state: ConjunctionState {
                            memory: BTreeMap::new(),
                        },
                    },
                )
            }
            b'b' => (s_name, Broadcast { name: s_name }),
            _ => panic!(),
        };
        modules.insert(name, module);
        outputs.insert(name, output);
    });

    for (name, output) in outputs.iter() {
        for &o in output.iter() {
            match modules.get_mut(o) {
                Some(Conjunction { state: s, .. }) => {
                    s.memory.insert(name, false);
                }
                _ => (),
            }
        }
    }

    let mut states_vec = Vec::new();
    let mut high = 0;
    let mut low = 0;
    let n_iter = 100000;
    for i in 0..n_iter {
        let state: Vec<bool> = modules.values().flat_map(get_state).collect();
        /*for b in state {
            print!("{}", if b {1} else {0});
        }
        println!();*/
        states_vec.push((high, low));
        let mut deq = VecDeque::new();
        deq.push_back(("broadcaster".as_bytes(), false, "".as_bytes()));
        while let Some((name, beam, source)) = deq.pop_front() {
            if beam {
                high += 1;
            } else {
                low += 1;
            }
            if let Some(m) = modules.get_mut(name) {
                let (n_beam, should_propogate) = match m {
                    FlipFlop { state: s, .. } => {
                        if !beam {
                            s.on_off = !s.on_off;
                            (s.on_off, true)
                        } else {
                            (false, false)
                        }
                    }
                    Conjunction { state: s, .. } => {
                        *s.memory.get_mut(source).unwrap() = beam;
                        let n_beam = !s.memory.values().all(|b| *b);
                        if !n_beam {
                            println!("{} low at {}", std::str::from_utf8(name).unwrap(), i);
                        }
                        (n_beam, true)
                    }
                    Broadcast => (beam, true),
                };

                if should_propogate {
                    for &o in outputs.get(name).unwrap() {
                        deq.push_back((o, n_beam, name));
                    }
                }
            }
        }
        if i % 10000 == 0 {
            println!("{}", i);
        }
    }

    println!("{} {} {}", high, low, high * low);
}
