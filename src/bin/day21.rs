use std::{
    collections::{HashSet, VecDeque},
    fs,
};

#[derive(Debug)]
struct Map {
    num_rows: i32,
    num_cols: i32,
    rocks: HashSet<(i32, i32)>,
    start: (i32, i32),
}

impl Map {
    fn is_rock(&self, row: i32, col: i32) -> bool {
        self.rocks.contains(&(row.rem_euclid(self.num_rows), col.rem_euclid(self.num_cols)))
    }
}

fn parse(lines: &Vec<&str>) -> Map {
    let num_rows = lines.len() as i32;
    let num_cols = lines[0].len() as i32;
    let mut rocks = HashSet::new();
    let mut start = (0, 0);
    for row in 0..lines.len() {
        let line = lines[row].chars().collect::<Vec<_>>();
        for col in 0..line.len() {
            let ch = line[col];
            match ch {
                '#' => {
                    rocks.insert((row as i32, col as i32));
                }
                'S' => {
                    start = (row as i32, col as i32);
                }
                _ => {}
            }
        }
    }
    Map {
        rocks,
        start,
        num_rows,
        num_cols,
    }
}

fn neighbours(row: i32, col: i32, map: &Map) -> Vec<(i32, i32)> {
    let mut res = Vec::new();
    res.push((row - 1, col));
    res.push((row + 1, col));
    res.push((row, col - 1));
    res.push((row, col + 1));
    res.retain(|x| !map.is_rock(x.0, x.1));
    res
}

fn part1(map: &Map, steps: u32) -> u32 {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut level_pos = Vec::new();
    level_pos.push(map.start);
    for i in 0..steps {
        let mut moves = HashSet::new();
        for (row, col) in &level_pos {
            for n in neighbours(*row, *col, map) {
                moves.insert((n.0, n.1));
            }
        }
        level_pos.clear();
        for m in moves {
            level_pos.push(m);
        }
    }
    level_pos.len() as u32
}

fn expand(map: &mut Map) {
    let base_rocks = map.rocks.clone();
    for row_mult in -10..=10 {
        for col_mult in -10..=10 {
            for (row, col) in &base_rocks {
                map.rocks
                    .insert((row + map.num_rows * row_mult, col + map.num_cols * col_mult));
            }
        }
    }
}

fn main() {
    let content = fs::read_to_string("inputs/day21.txt").unwrap();
    let lines = content.lines().collect::<Vec<_>>();
    let mut map = parse(&lines);

    // part 1
    // println!("{:?}", part1(&map, 64));

    // part 2
    // https://topaz.github.io/paste/#XQAAAQBCBgAAAAAAAAAzHIoib6pXbueH4X9F244lVRDcOZab5q1+VXn364pOX+QC8T+QV8WY66I0rlmXGVk2qledecBa8UqLTfbupnkeF9qCUXHb55rmt4QwFIG3YQhCWUwDqXpC8tvUoRjqgPvhJY6rhOsjUOhRSfFlVFhYRnT4WYnecvD2lfJgnZJKR5FAo/F4wQ+TKFaDphxVWfQmPq2wMsAoskpBFj3EWnInG/f7ID7m4hLLxZU3ACvsA6x3Npt/H2e6HWwtdxtaha9pDzEe58JFhW84zvJqrhsPIzcQiyIwuDIgOlXFk9BALecsEP2u0uzEFKqf7yPqg2g7b96VkEtN/u/naFCmrjE+Jils6jO1pPLesFAknj2QRzNTiWRTDsPtokWj+3vlscYiyO8Gp2iMBgxtSoC8WcRGmqb0duuxIW3ge1o+W2pOv/zX12apI8fMc/W99XewLzd0GcBHqw79FCaphY6v7hJPMmRgwwLvzEzP9QY5o/6JUJ3KK2JaplhCxqzjDQnL7sFYcFXloztlP0zvJyIELBQfpzpFWyehtN1lasLy5/M8bRUjvQIkjh7BL0KLvTgeD5va5owh4ArKiAi8e6u5ClXVukVLQ60Z4CJpjzfznnaSTaJ8Oc8WOIYIFIQ4L2VNdafXUTsWOLZrm1X/w9AW7qBySq6XOEqjZkHUyZWpee4mqmoZkAzSebK426Dbl2rCFFK0e+Bmjc3uUNUWOzb6RZL3vInt95DFqMjqeVD5vgblAOEQSI/y7QcOzvG9HKl3XfJnFn6ovCiArotKJI7PbuSKELnB1htUCyVq+DU5Ag96Ny720XP8IZAZQCaurqmbwHtzPYbdpLHqC+m/Ukgqx+39bkHR4lPYjvdzcPwxHRRvMakZeHDMOqVt5J9fDGzor8drCyd3bkl8dlZPUtcfzrZBiCTcdSvC3rH5W/bZdbul6/515R2RfP/paEZQ
    println!("{:?}", part1(&map, 65));
    println!("{:?}", part1(&map, 65+131));
    println!("{:?}", part1(&map, 65+131*2));
    // then wolfram alpha
    // quadratic fit {{65, 3848}, {196, 34310}, {327, 95144}}
    // then (15186/17161)*(26501365^2) + (26976/17161)*26501365 + (121238/17161)
}
