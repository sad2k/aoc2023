use std::fs;

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

fn main() {
    let content = fs::read_to_string("inputs/day15.txt").unwrap();
    let line = content.lines().next().unwrap();

    // part 1
    println!("{}", part1(line));
}
