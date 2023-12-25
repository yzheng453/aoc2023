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

use z3::ast::Int;
use z3::ast::Ast;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Hailstone {
    p: [i64; 3],
    v: [i64; 3]
}

const L_BOUND: f64 = 200000000000000.0;
const R_BOUND: f64 = 400000000000000.0;
//const L_BOUND: f64 = 7.0;
//const R_BOUND: f64 = 27.0;

fn main() {
    let lines = read_lines("input");
    let hailstones: Vec<Hailstone> = lines.iter().map(|line| {
        let mut i = line.split('@');
        let ps = i.next().unwrap();
        let mut p = [0, 0, 0];
        ps.split(',').enumerate().for_each(|(i, s)| p[i] = s.trim().parse().unwrap());
        let vs = i.next().unwrap();
        let mut v = [0, 0, 0];
        vs.split(',').enumerate().for_each(|(i, s)| v[i] = s.trim().parse().unwrap());
        Hailstone{p, v}
    }).collect();
    
    let config = z3::Config::new();
    let context = z3::Context::new(&config);
    let solver = z3::Solver::new(&context);

    let p: Vec<Int<'_>> = (0..3).map(|i| z3::ast::Int::new_const(&context, format!("p{i}"))).collect();
    let v: Vec<Int<'_>> = (0..3).map(|i| z3::ast::Int::new_const(&context, format!("v{i}"))).collect();
    
    let zero = z3::ast::Int::from_i64(&context, 0);
    
    for (i, h) in hailstones[3..6].iter().enumerate() {
        let hp: Vec<Int<'_>> = h.p.iter().map(|v| z3::ast::Int::from_i64(&context, *v)).collect();
        let hv: Vec<Int<'_>> = h.v.iter().map(|v| z3::ast::Int::from_i64(&context, *v)).collect();

        let t = z3::ast::Int::new_const(&context, format!("t{i}"));
        solver.assert(&t.gt(&zero));
        (0..3).for_each(|i| {
            solver.assert(&(v[i].clone() * t.clone() + p[i].clone())._eq(&(hv[i].clone() * t.clone() + hp[i].clone())));        
        })
    }
    
    if solver.check() != z3::SatResult::Sat {
        panic!();
    }
    let model = solver.get_model().unwrap();
    println!("{}", model.eval(&(p[0].clone() + p[1].clone() + p[2].clone()), true).unwrap());
}
