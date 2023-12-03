use std::{collections::HashMap, fs, str::Lines};

fn is_symbol(ch: char) -> bool {
    ch != '.' && !ch.is_digit(10)
}

fn is_gear(ch: char) -> bool {
    ch == '*'
}

fn is_adjacent_to_symbol(lines: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    let mut res = false;
    if (row > 0) {
        // previous row
        res |= is_symbol(lines[row - 1][col]);
        if col > 0 {
            res |= is_symbol(lines[row - 1][col - 1]);
        }
        if col < lines[0].len() - 1 {
            res |= is_symbol(lines[row - 1][col + 1]);
        }
    }
    if (row < lines.len() - 1) {
        // next row
        res |= is_symbol(lines[row + 1][col]);
        if col > 0 {
            res |= is_symbol(lines[row + 1][col - 1]);
        }
        if col < lines[0].len() - 1 {
            res |= is_symbol(lines[row + 1][col + 1]);
        }
    }
    // current row
    if col > 0 {
        res |= is_symbol(lines[row][col - 1]);
    }
    if col < lines[0].len() - 1 {
        res |= is_symbol(lines[row][col + 1]);
    }
    res
}

fn is_adjacent_to_gear(lines: &Vec<Vec<char>>, row: usize, col: usize) -> Option<(usize, usize)> {
    if (row > 0) {
        // previous row
        if is_gear(lines[row - 1][col]) {
            return Some((row - 1, col));
        }
        if col > 0 {
            if is_gear(lines[row - 1][col - 1]) {
                return Some((row - 1, col - 1));
            }
        }
        if col < lines[0].len() - 1 {
            if is_gear(lines[row - 1][col + 1]) {
                return Some((row - 1, col + 1));
            }
        }
    }
    if (row < lines.len() - 1) {
        // next row
        if is_gear(lines[row + 1][col]) {
            return Some((row + 1, col));
        }
        if col > 0 {
            if is_gear(lines[row + 1][col - 1]) {
                return Some((row + 1, col - 1));
            }
        }
        if col < lines[0].len() - 1 {
            if is_gear(lines[row + 1][col + 1]) {
                return Some((row + 1, col + 1));
            }
        }
    }
    // current row
    if col > 0 {
        if is_gear(lines[row][col - 1]) {
            return Some((row, col - 1));
        }
    }
    if col < lines[0].len() - 1 {
        if is_gear(lines[row][col + 1]) {
            return Some((row, col + 1));
        }
    }
    None
}

fn part1(lines: Lines<'_>) -> u64 {
    let l: Vec<_> = lines.map(|x| x.chars().collect::<Vec<_>>()).collect();
    let mut s = String::new();
    let mut is_part_number = false;
    let mut part_numbers = Vec::new();
    for i in 0..l.len() {
        let line = &l[i];
        s.clear();
        is_part_number = false;
        for j in 0..line.len() {
            let ch = line[j];
            if ch.is_digit(10) {
                s.push(ch);
                is_part_number |= is_adjacent_to_symbol(&l, i, j);
            } else {
                if !s.is_empty() {
                    if is_part_number {
                        part_numbers.push(s.clone());
                    }
                    s.clear();
                    is_part_number = false;
                }
            }
        }
        if !s.is_empty() {
            if is_part_number {
                part_numbers.push(s.clone());
            }
        }
    }
    part_numbers.iter().map(|x| x.parse::<u64>().unwrap()).sum()
}

fn part2(lines: Lines<'_>) -> u64 {
    let l: Vec<_> = lines.map(|x| x.chars().collect::<Vec<_>>()).collect();
    let mut s = String::new();
    let mut gear_of: Option<(usize, usize)> = None;
    let mut gears = HashMap::new();
    for i in 0..l.len() {
        let line = &l[i];
        s.clear();
        gear_of = None;
        for j in 0..line.len() {
            let ch = line[j];
            if ch.is_digit(10) {
                s.push(ch);
                let g = is_adjacent_to_gear(&l, i, j);
                if g.is_some() {
                    gear_of = g;
                }
            } else {
                if !s.is_empty() {
                    if gear_of.is_some() {
                        gears
                            .entry(gear_of.clone().unwrap())
                            .or_insert(Vec::new())
                            .push(s.parse::<u64>().unwrap());
                    }
                    s.clear();
                    gear_of = None;
                }
            }
        }
        if !s.is_empty() {
            if gear_of.is_some() {
                gears
                    .entry(gear_of.clone().unwrap())
                    .or_insert(Vec::new())
                    .push(s.parse::<u64>().unwrap());
            }
        }
    }
    gears
        .iter()
        .map(|x| if x.1.len() == 2 { Some(x.1) } else { None })
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .map(|x| x[0] * x[1])
        .sum()
}

fn main() {
    let contents = fs::read_to_string("inputs/day3.txt").unwrap();

    // part 1
    // println!("{}", part1(contents.lines()));

    // part 2
    println!("{}", part2(contents.lines()));
}
