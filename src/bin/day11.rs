use std::{collections::HashSet, fs};

fn is_empty_col(lines: &Vec<Vec<char>>, col: usize) -> bool {
    for i in 0..lines.len() {
        if lines[i][col] != '.' {
            return false;
        }
    }
    true
}

fn get_coords(lines: &Vec<Vec<char>>, expansion_coeff: i64) -> Vec<(i64, i64)> {
    let col_num = lines[0].len();
    let mut empty_cols = HashSet::new();
    for i in 0..col_num {
        if is_empty_col(lines, i) {
            empty_cols.insert(i);
        }
    }

    let mut coords = Vec::new();
    let mut row_idx: i64 = 0;
    for i in 0..lines.len() {
        let row = &lines[i];
        if row.iter().all(|x| *x == '.') {
            row_idx += expansion_coeff;
        } else {
            let mut col_idx: i64 = 0;
            for j in 0..row.len() {
                if empty_cols.contains(&j) {
                    col_idx += expansion_coeff;
                } else {
                    if row[j] == '#' {
                        coords.push((row_idx, col_idx));
                    }
                }
                col_idx += 1;
            }
        }
        row_idx += 1;
    }
    coords
}

fn part1(coords: &Vec<(i64,i64)>) -> i64 {
    let mut res: i64 = 0;
    for i in 0..coords.len() {
        for j in (i+1)..coords.len() {
            let c1 = coords[i];
            let c2 = coords[j];
            let dist = (c1.0 - c2.0).abs() + (c1.1 - c2.1).abs();
            res += dist;
        }
    }
    res
}

fn main() {
    let content = fs::read_to_string("inputs/day11.txt").unwrap();
    let lines = content
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // part 1
    let coords = get_coords(&lines, 1);
    println!("{}", part1(&coords));
}
