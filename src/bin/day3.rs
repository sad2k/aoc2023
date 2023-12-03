use std::{fs, str::Lines};

fn is_symbol(ch: char) -> bool {
    ch != '.' && !ch.is_digit(10)
}

fn is_adjacent_to_symbol(lines: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    let mut res = false;
    if (row > 0) {
        // previous row
        res |= is_symbol(lines[row-1][col]);
        if col > 0 {
            res |= is_symbol(lines[row-1][col-1]);
        }
        if col < lines[0].len()-1 {
            res |= is_symbol(lines[row-1][col+1]);
        }
    }
    if (row < lines.len()-1) {
        // next row
        res |= is_symbol(lines[row+1][col]);
        if col > 0 {
            res |= is_symbol(lines[row+1][col-1]);
        }
        if col < lines[0].len()-1 {
            res |= is_symbol(lines[row+1][col+1]);
        }
    }
    // current row
    if col > 0 {
        res |= is_symbol(lines[row][col-1]);
    }
    if col < lines[0].len()-1 {
        res |= is_symbol(lines[row][col+1]);
    }
    res
}

fn part1(lines: Lines<'_>) -> u64 {
    let l: Vec<_> = lines.map(|x| x.chars().collect::<Vec<_>>()).collect();
    let mut s = String::new();
    let mut is_part_number = false;
    let mut part_numbers = Vec::new();
    for i in 0..l.len() {
        // println!("{:?}", l[i]);
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

fn main() {
    let contents = fs::read_to_string("inputs/day3.txt").unwrap();

    // part 1
    println!("{}", part1(contents.lines()));
}
