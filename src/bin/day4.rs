use std::{collections::HashSet, fs, str::Lines};

use regex::Regex;

fn part1(lines: Lines<'_>) -> u64 {
    let regex = Regex::new(r"\s+").unwrap();
    let mut res = 0;
    for line in lines {
        let spl: Vec<_> = line
            .split(|c| c == ':' || c == '|')
            .map(|x| x.trim())
            .collect();
        let winning: Vec<_> = regex
            .split(spl[1])
            .map(|x| x.trim().parse::<u64>().unwrap())
            .collect();
        let winning: HashSet<u64> = HashSet::from_iter(winning);
        let have_winning: Vec<_> = regex
            .split(spl[2])
            .map(|x| x.trim().parse::<u64>().unwrap())
            .filter(|x| winning.contains(x))
            .collect();
        let score = if have_winning.is_empty() { 0 } else { 2_u64.pow((have_winning.len() - 1) as u32) };
        res += score;
    }
    res
}

fn main() {
    let contents = fs::read_to_string("inputs/day4.txt").unwrap();

    // part 1
    println!("{}", part1(contents.lines()));
}
