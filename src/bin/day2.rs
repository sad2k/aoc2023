use std::{fs, str::Lines};

#[derive(Debug)]
struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

impl CubeSet {
    fn power(&self) -> u64 {
        self.red as u64 * self.green as u64 * self.blue as u64
    }

    fn max(&self, other: &CubeSet) -> CubeSet {
        CubeSet { red: self.red.max(other.red), green: self.green.max(other.green), blue: self.blue.max(other.blue) }
    }

    fn empty() -> CubeSet {
        CubeSet { red: 0, green: 0, blue: 0 }
    }
}

fn parse(line: &str) -> Vec<CubeSet> {
    let mut res = Vec::new();
    let sets = line.split(": ").collect::<Vec<_>>()[1];
    for s in sets.split("; ") {
        let cubes = s.split(", ");
        let mut cs = CubeSet {
            red: 0,
            green: 0,
            blue: 0,
        };
        for cube in cubes {
            let split_cube = cube.split(" ").collect::<Vec<_>>();
            let num: u32 = split_cube[0].parse().unwrap();
            match split_cube[1] {
                "red" => cs.red = num,
                "green" => cs.green = num,
                "blue" => cs.blue = num,
                _ => panic!("unexpected colour: {}", split_cube[1]),
            }
        }
        res.push(cs);
    }
    res
}

fn part1(lines: Lines<'_>) -> u32 {
    let mut res = 0;
    let mut game = 0;
    for line in lines {
        game += 1;
        let parsed_sets = parse(line);
        let bad_sets: Vec<_> = parsed_sets
            .iter()
            .filter(|cs| cs.red > 12 || cs.green > 13 || cs.blue > 14)
            .collect();
        if bad_sets.is_empty() {
            res += game;
        }
    }
    res
}

fn part2(lines: Lines<'_>) -> u64 {
    let mut res = 0;
    let mut game = 0;
    for line in lines {
        game += 1;
        let parsed_sets = parse(line);
        let power_set = parsed_sets.iter().fold(CubeSet::empty(), |acc,x| acc.max(x));
        res += power_set.power();
    }
    res
}

fn main() {
    let contents = fs::read_to_string("inputs/day2.txt").unwrap();

    // part 1
    // println!("{}", part1(contents.lines()));

    // part 2
    println!("{}", part2(contents.lines()));
}
