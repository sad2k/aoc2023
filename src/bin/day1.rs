use std::{fs, str::Lines};

fn get_digits(line: &str) -> Vec<u32> {
    let digits: Vec<u32> = line
        .chars()
        .map(|c| c.to_digit(10))
        .filter(|o| o.is_some())
        .map(|o| o.unwrap())
        .collect();
    digits
}

fn part1(lines: Lines<'_>) -> u32 {
    let mut res = 0;
    for line in lines {
        let digits = get_digits(line);
        res += digits[0] * 10 + digits[digits.len() - 1];
    }
    res
}

fn replace_spelled_digits(s: String) -> String {
    let mapping: [(Vec<char>, char); 9] = [
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]
    .map(|t| (t.0.chars().collect(), t.1));
    let mut working: Vec<char> = s.chars().collect();
    let mut i = 0;
    let t = &working[0..2];
    let mut res = String::new();
    while i < working.len() {
        if working[i].is_digit(10) {
            res.push(working[i])
        } else {
            for m in mapping.iter() {
                let candidate = &m.0;
                if i + candidate.len() <= working.len() {
                    let str_slice = &working[i..i + candidate.len()];
                    if str_slice == candidate {
                        res.push(m.1);
                        break;
                    }
                }
            }
        }
        i += 1;
    }
    res
}

fn part2(lines: Lines<'_>) -> u32 {
    let mut res = 0;
    for line in lines {
        let mut line_str = replace_spelled_digits(String::from(line));
        let digits = get_digits(&line_str);
        println!("{:?}", digits);
        res += digits[0] * 10 + digits[digits.len() - 1];
    }
    res
}

fn main() {
    let contents = fs::read_to_string("inputs/day1.txt").unwrap();

    // day 1
    //println!("{}", part1(contents.lines()));

    // day 2
    println!("{}", part2(contents.lines()));
}
