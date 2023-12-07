use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
        
}

struct Cubes {
    red: i32,
    green: i32,
    blue: i32,
}

fn main() {
    let lines = read_lines("input");
    let mut sum = 0;
    for line in lines.iter() {
        let p1 = line.find(' ').unwrap() + 1;
        let p2 = line.find(':').unwrap();
        let game_num = &line[p1..p2].parse::<i32>().unwrap();
        let illegal = line[(p2 + 1)..].split(';')
            .flat_map(|game_str| game_str.trim().split(','))
            .any(|cubes_str| {
                let mut it = cubes_str.trim().split(' ');
                let n = it.next().unwrap().parse::<i32>().unwrap();
                let color = it.next().unwrap().trim();
                let limit = match color  {
                    "blue" => 14,
                    "green" => 13,
                    "red" => 12,
                    _ => panic!("Unknown color"),
                };
                n > limit
            });
        
        if !illegal {
            sum += game_num;     
        }
    }
    
    println!("{}", sum);

}