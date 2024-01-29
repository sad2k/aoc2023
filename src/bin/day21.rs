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
        self.rocks.contains(&(row, col))
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
    expand(&mut map);
    println!("{:?}", part1(&map, 131));
}
