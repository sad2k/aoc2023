use std::fs;

use regex::Regex;

fn calculate_race(time: u64, best_distance: u64) -> u64 {
    let mut res = 0;
    for speed in 1..time {
        let dist = speed * (time-speed);
        if dist > best_distance {
            res += 1;
        }
    }
    res
}

fn part1(times: &Vec<u64>, distances: &Vec<u64>) -> u64 {
    let mut res = 1;
    for i in 0..times.len() {
        res *= calculate_race(times[i], distances[i]);
        
    }
    res
}

fn main() {
    let content = fs::read_to_string("inputs/day6.txt").unwrap();
    let lines: Vec<_> = content.lines().collect();
    let regex = Regex::new(r" +").unwrap();
    let times = &(regex.split(lines[0]).collect::<Vec<_>>())[1..].iter().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<_>>();
    let distances = &(regex.split(lines[1]).collect::<Vec<_>>())[1..].iter().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<_>>();

    // part 1
    println!("{}", part1(times, distances));
}
