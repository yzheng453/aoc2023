use std::{
    cmp::Ordering,
    collections::VecDeque,
    collections::{btree_map::Range, HashMap},
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

#[derive(Clone, Copy)]
enum Action<'a> {
    Accept,
    Reject,
    Jump { s: &'a str },
}

enum Op {
    Gt,
    Lt,
}

enum Rule<'a> {
    Unconditional {
        a: Action<'a>,
    },
    Conditional {
        c: u8,
        v: i32,
        a: Action<'a>,
        op: Op,
    },
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
        _ => Jump { s },
    }
}

fn build_rule(rule_str: &str) -> Rule {
    let bytes = rule_str.as_bytes();
    let option_colon = bytes.iter().enumerate().find(|(_, &c)| c == b':');
    if let Some((id_colon, _)) = option_colon {
        let v_str = &bytes[2..id_colon];
        let v: i32 = std::str::from_utf8(v_str).unwrap().parse().unwrap();
        let a_str = std::str::from_utf8(&bytes[id_colon + 1..]).unwrap();
        let op = match bytes[1] {
            b'<' => Op::Lt,
            b'>' => Op::Gt,
            _ => panic!(),
        };
        Conditional {
            c: bytes[0],
            v: v,
            a: parse_action(a_str),
            op: op,
        }
    } else {
        Unconditional {
            a: parse_action(rule_str),
        }
    }
}

#[derive(Clone, Copy)]
struct ValueRange {
    min: i32,
    max: i32,
}

#[derive(Clone, Copy)]
struct PartRange {
    x: ValueRange,
    m: ValueRange,
    a: ValueRange,
    s: ValueRange,
}

fn sch_action<'a>(
    r: PartRange,
    action: Action,
    workflows: &'a HashMap<&'a str, Workflow<'a>>,
) -> i64 {
    match action {
        Jump { s } => sch_wf(r, workflows.get(s).unwrap(), 0, workflows),
        Accept => {
            let mut combinations = (r.x.max - r.x.min) as i64;
            combinations *= (r.m.max - r.m.min) as i64;
            combinations *= (r.a.max - r.a.min) as i64;
            combinations *= (r.s.max - r.s.min) as i64;
            combinations
        }
        Reject => 0,
    }
}

fn get_field<'a>(c: &'a u8, r: &'a PartRange) -> &'a ValueRange {
    match *c {
        b'x' => &r.x,
        b'm' => &r.m,
        b'a' => &r.a,
        b's' => &r.s,
        _ => panic!(),
    }
}

fn get_field_mut<'a>(c: &'a u8, r: &'a mut PartRange) -> &'a mut ValueRange {
    match *c {
        b'x' => &mut r.x,
        b'm' => &mut r.m,
        b'a' => &mut r.a,
        b's' => &mut r.s,
        _ => panic!(),
    }
}

fn sch_wf<'a>(
    r: PartRange,
    workflow: &Workflow<'a>,
    i: usize,
    workflows: &'a HashMap<&'a str, Workflow<'a>>,
) -> i64 {
    match &workflow.rules[i] {
        Unconditional { a } => sch_action(r, *a, workflows),
        Conditional {
            c,
            v,
            a: action,
            op,
        } => match op {
            Op::Gt => {
                if get_field(c, &r).min > *v {
                    sch_action(r, *action, workflows)
                } else if get_field(c, &r).max > v + 1 {
                    let mut r_l = r.clone();
                    get_field_mut(c, &mut r_l).max = v + 1;
                    let combs_l = sch_wf(r_l, workflow, i + 1, workflows);
                    let mut r_r = r.clone();
                    get_field_mut(c, &mut r_r).min = v + 1;
                    let combs_r = sch_action(r_r, *action, workflows);
                    combs_l + combs_r
                } else if get_field(c,&r).max <= *v {
                    sch_wf(r, workflow, i + 1, workflows)
                } else {
                    panic!()
                }
            }
            Op::Lt => {
                if get_field(c, &r).max <= *v {
                    sch_action(r, *action, workflows)
                } else if get_field(c, &r).min < *v {
                    let mut r_l = r.clone();
                    get_field_mut(c, &mut r_l).max = *v;
                    let combs_l = sch_action(r_l, *action, workflows);
                    let mut r_r = r.clone();
                    get_field_mut(c, &mut r_r).min = *v;
                    let combs_r = sch_wf(r_r, workflow, i + 1, workflows);
                    combs_l + combs_r
                } else if get_field(c, &r).min >= *v {
                    sch_wf(r, workflow, i + 1, workflows)
                } else {
                    panic!()
                }
            }
        },
    }
}

fn main() {
    let lines = read_lines("input");
    let (id_blank, _) = lines
        .iter()
        .enumerate()
        .find(|(_, line)| *line == "")
        .unwrap();
    let mut workflows: HashMap<&str, Workflow> = HashMap::new();
    for workflow_line in &lines[0..id_blank] {
        let id0 = workflow_line.find('{').unwrap();
        let name = &workflow_line[0..id0];
        let rules_str = &workflow_line[(id0 + 1)..workflow_line.len() - 1];
        let rls: Vec<Rule> = rules_str.split(',').map(build_rule).collect();
        workflows.insert(name, Workflow { rules: rls });
    }

    let r = PartRange {
        x: ValueRange { min: 1, max: 4001 },
        m: ValueRange { min: 1, max: 4001 },
        a: ValueRange { min: 1, max: 4001 },
        s: ValueRange { min: 1, max: 4001 },
    };
    let ans = sch_action(r, Jump { s: "in" }, &workflows);

    println!("{}", ans);
}
