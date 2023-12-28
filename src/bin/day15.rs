use std::{collections::HashMap, fs};

fn hash(s: &str) -> u64 {
    let mut res: u64 = 0;
    let bytes = s.bytes();
    for b in bytes {
        res += b as u64;
        res *= 17;
        res %= 256;
    }
    res
}

fn part1(s: &str) -> u64 {
    let mut res = 0;
    let hashes = s.split(",").map(|x| hash(x));
    hashes.sum()
}

fn part2(s: &str) -> u64 {
    let mut map: HashMap<u64, Vec<(String, u64)>> = HashMap::new();
    let mut res = 0;
    for step in s.split(",") {
        let lens: String;
        let focal_length: Option<u64>;
        if step.contains("-") {
            lens = String::from(&step[0..step.len()-1]);
            focal_length = None;
        } else if step.contains("=") {
            let mut spl = step.split("=");
            lens = String::from(spl.next().unwrap());
            focal_length = Some(spl.next().unwrap().parse::<u64>().unwrap());
        } else {
            panic!("bad step: {step}");
        }
        let b = hash(&lens);
        match focal_length {
            Some(l) => {
                let lenses = map.entry(b).or_insert(Vec::new());
                let idx = lenses.iter().position(|y| y.0 == lens);
                if idx.is_some() {
                    lenses.get_mut(idx.unwrap()).unwrap().1 = l;
                } else {
                    lenses.push((lens, l));
                }
            },
            None => {
                map.get_mut(&b).map(|x| {
                    let idx = x.iter().position(|y| y.0 == lens);
                    if idx.is_some() {
                        x.remove(idx.unwrap());
                    }
                });
            }
        }
    }
    for (b, lenses) in map.iter() {
        for (i, (_, focal_length)) in lenses.iter().enumerate() {
            res += (b+1) * (i+1) as u64 * focal_length;
        }
    }
    res
}

fn main() {
    let content = fs::read_to_string("inputs/day15.txt").unwrap();
    let line = content.lines().next().unwrap();

    // part 1
    // println!("{}", part1(line));

    // part 1
    println!("{}", part2(line));
}
