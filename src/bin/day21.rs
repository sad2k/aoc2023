use std::{
    collections::{HashSet, VecDeque},
    fs,
};

#[derive(Debug)]
struct Map {
    rocks: Vec<(i32, i32)>,
    start: (i32, i32),
}

fn parse(lines: &Vec<&str>) -> Map {
    let mut rocks = Vec::new();
    let mut start = (0, 0);
    for row in 0..lines.len() {
        let line = lines[row].chars().collect::<Vec<_>>();
        for col in 0..line.len() {
            let ch = line[col];
            match ch {
                '#' => {
                    rocks.push((row as i32, col as i32));
                }
                'S' => {
                    start = (row as i32, col as i32);
                }
                _ => {}
            }
        }
    }
    Map { rocks, start }
}

fn neighbours(row: i32, col: i32, map: &Map) -> Vec<(i32, i32)> {
    let mut res = Vec::new();
    res.push((row - 1, col));
    res.push((row + 1, col));
    res.push((row, col - 1));
    res.push((row, col + 1));
    res.retain(|x| !map.rocks.contains(x));
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

fn main() {
    let content = fs::read_to_string("inputs/day21.txt").unwrap();
    let lines = content.lines().collect::<Vec<_>>();
    let map = parse(&lines);
    println!("{:?}", part1(&map, 64));
}
