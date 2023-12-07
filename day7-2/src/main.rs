use std::{fs::read_to_string, collections::HashMap, cmp::Ordering};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
        
}

#[derive(Clone, Copy)]
enum HandType {
    Five = 6,
    Four = 5,
    FullHouse = 4,
    Three = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0
}

struct Hand {
    cards: String,
    bid: i64,
    t: HandType 
}

fn card_u8_to_val(c: u8) -> i32 {
    match c {
        b'2'..=b'9' => c as i32 - 48,
        b'T' => 10,
        b'J' => 1,
        b'Q' => 12,
        b'K' => 13,
        b'A' => 14,
        _ => panic!(),
    }
}

fn parse_line(line: &String) -> Hand {
    let mut it = line.split(' ');
    let cards: String = it.next().unwrap().to_string();
    let bid: i64 = it.next().unwrap().parse().unwrap();
    let mut cnt_map: HashMap<u8, i64> = HashMap::new();
    let mut j = 0;
    cards.as_bytes().iter().for_each(|b| {
        if *b == b'J' {
            j += 1;
        } else if let Some(v) = cnt_map.get_mut(b) {
            *v += 1;
        } else {
            cnt_map.insert(*b, 1);
        }
    });
    let mut c: Vec<(u8, i64)> = cnt_map.into_iter().collect();
    c.sort_by(|&a, &b| b.1.cmp( &a.1));
    let t = if j == 5 { HandType::Five} 
    else {
        match c[0].1 + j {
        5 => HandType::Five,
        4 => HandType::Four,
        3 => match c[1].1 {
            2 => HandType::FullHouse,
            1 => HandType::Three,
            _ => panic!(),
        },
        2 => match c[1].1 {
            2 => HandType::TwoPair,
            1 => HandType::OnePair,
            _ => panic!(),
        },
        1 => HandType::HighCard,
        _ => panic!(),
    }};
    Hand{cards: cards, bid: bid, t: t}
}

fn main() {
    let lines = read_lines("input");
    let mut hands: Vec<Hand> = lines.iter().map(parse_line).collect(); 
    hands.sort_by(|a, b| {
        if (a.t as u32) < (b.t as u32) {
            Ordering::Less
        } else if (a.t as u32) > (b.t as u32) {
            Ordering::Greater
        } else {
            let (x, y) = a.cards.as_bytes().iter().zip(b.cards.as_bytes().iter())
                .filter(|(&x, &y)| x != y)
                .next().unwrap();
            card_u8_to_val(*x).cmp(&card_u8_to_val(*y))
        }
    });
    let ans = hands.iter().enumerate().map(|(rank, hand)| (rank as i64 + 1) * hand.bid).fold(0, i64::saturating_add);

    println!("{}", ans);
}
