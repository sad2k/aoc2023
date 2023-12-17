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

    // part 1
    let regex = Regex::new(r" +").unwrap();
    let times = &(regex.split(lines[0]).collect::<Vec<_>>())[1..].iter().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<_>>();
    let distances = &(regex.split(lines[1]).collect::<Vec<_>>())[1..].iter().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<_>>();
    println!("{}", part1(times, distances));

    // part 2
    let lines: Vec<_> = lines.iter().map(|x| x.replace(" ", "")).collect();
    let time = lines[0].split(":").collect::<Vec<_>>()[1].parse::<u64>().unwrap();
    let best_distance = lines[1].split(":").collect::<Vec<_>>()[1].parse::<u64>().unwrap();
    println!("{:?}", calculate_race(time, best_distance));
}
