use std::{fs::read_to_string, collections::HashMap, cmp::Ordering, collections::VecDeque, default, slice::Iter};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
        
}

enum LensSlot<'a> {
    Lens{label: &'a [u8], fl: i32},
    NoLens,
}


fn main() {
    let lines = read_lines("input");
    let seq: Vec<&str> = lines[0].split(',').collect();
    let mut boxes: Vec<Vec<LensSlot>> = (0..256).map(|_| Vec::new()).collect();
    for step in seq {
        let mut current = 0_i32;
        let sb = step.as_bytes();
        for (i, b) in sb.iter().enumerate() {
            match b {
                b'=' => {
                    let label = &sb[0..i];
                    let fl = (sb[i + 1] - b'0')as i32;
                    let current_box = &mut boxes[current as usize];
                    let i = current_box.iter_mut().find(|ls| {
                        if let LensSlot::Lens { label: l, fl: _ } = ls {
                            if *l == label {
                                return true;
                            }
                        }
                        return false;
                    });
                    match i {
                        None => current_box.push(LensSlot::Lens { label: label, fl: fl }),
                        Some(ls) => *ls = LensSlot::Lens { label: label, fl: fl },
                    }
                },
                b'-' => {
                    let label = &sb[0..i];
                    let current_box = &mut boxes[current as usize];
                    let i = current_box.iter_mut().for_each(|ls| {
                        if let LensSlot::Lens { label: l, fl: _ } = ls {
                            if *l == label {
                                *ls = LensSlot::NoLens;
                            }
                        }
                    });
                },
                _ => {
                    current += *b as i32;
                    current *= 17;
                    current = current % 256;
                }
            }
        }
    }
    let ans = boxes.iter().enumerate().map(|(box_id, current_box)| {
        let mut slot_num = 1;
        let mut sum = 0;
        let bn = box_id as i32 + 1;
        for slot in current_box {
            if let LensSlot::Lens { label: _, fl } = slot {
                sum += bn * slot_num * fl;
                slot_num += 1;
            }
        }
        sum
    }).fold(0, i32::saturating_add);
    println!("{}", ans);
}
