use std::{
    collections::{HashSet, VecDeque},
    fs,
};
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Down,
    Up,
    Right,
    Left,
}

fn move_in_direction(row: i32, col: i32, direction: Direction) -> (i32, i32) {
    match direction {
        Direction::Down => (row + 1, col),
        Direction::Up => (row - 1, col),
        Direction::Right => (row, col + 1),
        Direction::Left => (row, col - 1),
    }
}

fn part1(map: &Vec<Vec<char>>, row: i32, col: i32, dir: Direction) -> u64 {
    let mut beams: VecDeque<(i32, i32, Direction)> = VecDeque::from([(row, col, dir)]);
    let num_rows = map.len() as i32;
    let num_cols = map[0].len() as i32;
    let mut all_beams: HashSet<(i32, i32, Direction)> = HashSet::new();
    let mut energized: HashSet<(i32, i32)> = HashSet::new();
    while !beams.is_empty() {
        let (mut beam_row, mut beam_col, mut beam_dir) = beams.pop_front().unwrap();
        // println!("Executing beam: {},{} {:?}", beam_row, beam_col, beam_dir);
        let mut visited: HashSet<(i32, i32, Direction)> = HashSet::new();
        loop {
            if beam_row < 0 || beam_row >= num_rows || beam_col < 0 || beam_col >= num_cols {
                break;
            }
            if visited.contains(&(beam_row, beam_col, beam_dir)) {
                break;
            }
            visited.insert((beam_row, beam_col, beam_dir));
            energized.insert((beam_row, beam_col));
            let tile = map[beam_row as usize][beam_col as usize];
            // println!(
                // "Tile {} at {},{} dir {:?}",
                // tile, beam_row, beam_col, beam_dir
            // );
            match tile {
                '.' => {
                    (beam_row, beam_col) = move_in_direction(beam_row, beam_col, beam_dir);
                }
                '/' => {
                    beam_dir = match beam_dir {
                        Direction::Down => Direction::Left,
                        Direction::Up => Direction::Right,
                        Direction::Left => Direction::Down,
                        Direction::Right => Direction::Up,
                    };
                    (beam_row, beam_col) = move_in_direction(beam_row, beam_col, beam_dir);
                }
                '\\' => {
                    beam_dir = match beam_dir {
                        Direction::Down => Direction::Right,
                        Direction::Up => Direction::Left,
                        Direction::Left => Direction::Up,
                        Direction::Right => Direction::Down,
                    };
                    (beam_row, beam_col) = move_in_direction(beam_row, beam_col, beam_dir);
                }
                '-' => match beam_dir {
                    Direction::Down | Direction::Up => {
                        let beam1 = (beam_row, beam_col - 1, Direction::Left);
                        if !all_beams.contains(&beam1) {
                            // println!("New beam: {:?}", beam1);
                            beams.push_back(beam1);
                            all_beams.insert(beam1);
                        }

                        let beam2 = (beam_row, beam_col + 1, Direction::Right);
                        if !all_beams.contains(&beam2) {
                            // println!("New beam: {:?}", beam2);
                            beams.push_back(beam2);
                            all_beams.insert(beam2);
                        }

                        break;
                    }
                    _ => {
                        (beam_row, beam_col) = move_in_direction(beam_row, beam_col, beam_dir);
                    }
                },
                '|' => match beam_dir {
                    Direction::Left | Direction::Right => {
                        let beam1 = (beam_row - 1, beam_col, Direction::Up);
                        // println!("New beam: {:?}", beam1);
                        beams.push_back(beam1);

                        let beam2 = (beam_row + 1, beam_col, Direction::Down);
                        // println!("New beam: {:?}", beam2);
                        beams.push_back(beam2);

                        break;
                    }
                    _ => {
                        (beam_row, beam_col) = move_in_direction(beam_row, beam_col, beam_dir);
                    }
                },
                _ => panic!("bad tile: {tile}"),
            }
        }
    }
    energized.len() as u64
}

fn part2(map: &Vec<Vec<char>>) -> u64 {
    let mut res = 0;
    for row in 0..map.len() {
        res = res.max(part1(map, row as i32, 0, Direction::Right));
        res = res.max(part1(map, row as i32, (map[0].len()-1) as i32, Direction::Left));
    }
    for col in 0..map[0].len() {
        res = res.max(part1(map, 0, col as i32, Direction::Down));
        res = res.max(part1(map, (map.len()-1) as i32, col as i32, Direction::Up));
    }
    res
}

fn main() {
    let content = fs::read_to_string("inputs/day16.txt").unwrap();
    let lines = content
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // part 1
    // println!("{}", part1(&lines, 0, 0, Direction::Right));

    // part 2
    println!("{}", part2(&lines));
}
