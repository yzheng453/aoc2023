use std::{fs::read_to_string, collections::HashMap, cmp::Ordering, collections::VecDeque, default, slice::Iter};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
        
}

#[derive(Clone, Copy)]
enum Action<'a> {
    Accept,
    Reject,
    Jump{s: &'a str},
}

enum Op {
    Gt,
    Lt,
}

enum Rule<'a> {
    Unconditional{a: Action<'a>},
    Conditional {c: u8, v: i32, a: Action<'a>, op: Op},
}

use Action::*;
use Rule::*;

struct Workflow<'a> {
    rules: Vec<Rule<'a>>,
}

fn parse_action(s: &str) -> Action {
    match s {
        "A" => Accept,
        "R" => Reject,
        _ => Jump{s},
    }
}

fn build_rule(rule_str: &str) -> Rule {
    let bytes = rule_str.as_bytes();
    let option_colon = bytes.iter().enumerate().find(|(_, &c)| c == b':');
    if let Some((id_colon, _)) = option_colon {
        let v_str = &bytes[2..id_colon];
        let v: i32 = std::str::from_utf8(v_str).unwrap().parse().unwrap();
        let a_str = std::str::from_utf8(&bytes[id_colon+1..]).unwrap();
        let op = match bytes[1] {
            b'<' => Op::Lt,
            b'>' => Op::Gt,
            _ => panic!(),
        };
        Conditional { c: bytes[0], v: v, a: parse_action(a_str), op: op }
    } else {
        Unconditional { a: parse_action(rule_str) }
    }
}

fn apply_workflows<'a>(x: i32, m: i32, a: i32, s: i32, workflows: &'a HashMap<&'a str, Workflow<'a>>) -> Action<'a> {
    let mut current = Jump{s: "in"};
    while let Jump{s: name} = current {
        let wf = workflows.get(name).unwrap();
        for rule in wf.rules.iter() {
            match rule {
                Unconditional { a } => {
                    current = a.clone();
                    break;
                },
                Conditional { c, v, a: action , op} => {
                    let actual_value = match c {
                        b'x' => x,
                        b'm' => m,
                        b'a' => a,
                        b's' => s,
                        _ => panic!(),
                    };
                    match op {
                        Op::Gt => if actual_value > *v {
                            current = action.clone();
                            break;
                        },
                        Op::Lt => if actual_value < *v {
                            current = action.clone();
                            break;
                        },
                    }
                }
            }
        }
    }
    current
}

fn main() {
    let lines = read_lines("input");
    let (id_blank, _) = lines.iter().enumerate().find(|(_, line)| *line == "").unwrap();
    let mut workflows: HashMap<&str, Workflow> = HashMap::new();
    for workflow_line in &lines[0..id_blank] {
        let id0 = workflow_line.find('{').unwrap();
        let name = &workflow_line[0..id0];
        let rules_str = &workflow_line[(id0+1)..workflow_line.len() - 1];
        let rls: Vec<Rule> = rules_str.split(',').map(build_rule).collect();
        workflows.insert(name, Workflow{rules: rls});
    }
    
    let mut sum = 0;
    for part in &lines[id_blank+1 ..] {
        let pattern: &[_] = &['{', '}', 'x', 'm', 'a', 's', '='];
        let mut it = part.split(',');
        let x: i32 = it.next().unwrap().trim_matches(pattern).parse().unwrap();
        let m: i32 = it.next().unwrap().trim_matches(pattern).parse().unwrap();
        let a: i32 = it.next().unwrap().trim_matches(pattern).parse().unwrap();
        let s: i32 = it.next().unwrap().trim_matches(pattern).parse().unwrap();
        
        let action = apply_workflows(x, m, a, s, &workflows);
        if matches!(action, Accept) {
            sum += x + m + a + s;
        }
    }


    println!("{}", sum);
}
