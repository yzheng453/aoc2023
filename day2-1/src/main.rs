use std::{fs::read_to_string, cmp::max, arch::x86_64::_mm256_storeu2_m128};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
        
}

struct Cubes(i64, i64, i64);

impl Cubes {
    fn power(&self) -> i64 {
        self.0 * self.1 * self.2
    }
    
    fn max(self, other: Cubes) -> Cubes {
        Cubes(max(self.0, other.0), max(self.1, other.1), max(self.2, other.2))
    }
}

fn main() {
    let lines = read_lines("input");
    let sum = lines.iter().map(|line| {
        let p1 = line.find(' ').unwrap() + 1;
        let p2 = line.find(':').unwrap();
        let game_num = &line[p1..p2].parse::<i64>().unwrap();
        let minimal_cubes = line[(p2 + 1)..].split(';')
            .flat_map(|game_str| game_str.trim().split(','))
            .map(|cubes_str| {
                let mut it = cubes_str.trim().split(' ');
                let n = it.next().unwrap().parse::<i64>().unwrap();
                let color = it.next().unwrap().trim();
                match color  {
                    "red" => Cubes(n, 0, 0),
                    "green" => Cubes(0, n, 0),
                    "blue" => Cubes(0, 0, n),
                    _ => panic!("Unknown color"),
                }
            })
            .fold(Cubes(0, 0, 0), Cubes::max);
        minimal_cubes.power()
    }).fold(0, i64::saturating_add);
    println!("{}", sum);
}