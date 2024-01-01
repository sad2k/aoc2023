use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
    fs,
};

use priority_queue::PriorityQueue;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Down,
    Up,
    Left,
    Right,
    None,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    row: i32,
    col: i32,
    dir: Direction,
    no_turn_steps: i32,
}

impl Position {
    fn new(row: i32, col: i32, dir: Direction, no_turn_steps: i32) -> Position {
        Position {
            row,
            col,
            dir,
            no_turn_steps,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Edge {
    pos: Position,
    weight: u32,
}

impl Edge {
    fn new(pos: Position, weight: u32) -> Edge {
        Edge { pos, weight }
    }
}

fn build_map_part1(weights: &Vec<Vec<u32>>) -> HashMap<Position, Vec<Edge>> {
    let mut edges: HashMap<Position, Vec<Edge>> = HashMap::new();

    // build the graph
    let num_rows = weights.len();
    let num_cols = weights[0].len();
    for row in 0..num_rows {
        for col in 0..num_cols {
            /////////////////////////////////// DIR DOWN
            if row > 0 {
                if row < (num_rows - 1) {
                    // can still go down further
                    let weight_down = weights[row + 1][col];
                    for steps in 1..=2 {
                        edges
                            .entry(Position::new(
                                row as i32,
                                col as i32,
                                Direction::Down,
                                steps,
                            ))
                            .or_insert(Vec::new())
                            .push(Edge::new(
                                Position::new(
                                    (row + 1) as i32,
                                    col as i32,
                                    Direction::Down,
                                    steps + 1,
                                ),
                                weight_down,
                            ));
                    }
                }

                if col > 0 {
                    // can turn left
                    let weight_left = weights[row][col - 1];
                    for steps in 1..=3 {
                        edges
                            .entry(Position::new(
                                row as i32,
                                col as i32,
                                Direction::Down,
                                steps,
                            ))
                            .or_insert(Vec::new())
                            .push(Edge::new(
                                Position::new(row as i32, (col - 1) as i32, Direction::Left, 1),
                                weight_left,
                            ));
                    }
                }

                if col < num_cols - 1 {
                    // can turn right
                    let weight_right = weights[row][col + 1];
                    for steps in 1..=3 {
                        edges
                            .entry(Position::new(
                                row as i32,
                                col as i32,
                                Direction::Down,
                                steps,
                            ))
                            .or_insert(Vec::new())
                            .push(Edge::new(
                                Position::new(row as i32, (col + 1) as i32, Direction::Right, 1),
                                weight_right,
                            ));
                    }
                }
            }

            /////////////////////////////////// DIR UP
            if row < (num_rows - 1) {
                if row > 0 {
                    // can still go up further
                    let weight_up = weights[row - 1][col];
                    for steps in 1..=2 {
                        edges
                            .entry(Position::new(row as i32, col as i32, Direction::Up, steps))
                            .or_insert(Vec::new())
                            .push(Edge::new(
                                Position::new(
                                    (row - 1) as i32,
                                    col as i32,
                                    Direction::Up,
                                    steps + 1,
                                ),
                                weight_up,
                            ));
                    }
                }

                if col > 0 {
                    // can turn left
                    let weight_left = weights[row][col - 1];
                    for steps in 1..=3 {
                        edges
                            .entry(Position::new(row as i32, col as i32, Direction::Up, steps))
                            .or_insert(Vec::new())
                            .push(Edge::new(
                                Position::new(row as i32, (col - 1) as i32, Direction::Left, 1),
                                weight_left,
                            ));
                    }
                }

                if col < num_cols - 1 {
                    // can turn right
                    let weight_right = weights[row][col + 1];
                    for steps in 1..=3 {
                        edges
                            .entry(Position::new(row as i32, col as i32, Direction::Up, steps))
                            .or_insert(Vec::new())
                            .push(Edge::new(
                                Position::new(row as i32, (col + 1) as i32, Direction::Right, 1),
                                weight_right,
                            ));
                    }
                }
            }

            ////////////////////////////////// DIR RIGHT
            if col > 0 {
                if col < (num_cols - 1) {
                    // can still go right further
                    let weight_right = weights[row][col + 1];
                    for steps in 1..=2 {
                        edges
                            .entry(Position::new(
                                row as i32,
                                col as i32,
                                Direction::Right,
                                steps,
                            ))
                            .or_insert(Vec::new())
                            .push(Edge::new(
                                Position::new(
                                    row as i32,
                                    (col + 1) as i32,
                                    Direction::Right,
                                    steps + 1,
                                ),
                                weight_right,
                            ));
                    }
                }

                if row > 0 {
                    // can turn up
                    let weight_up = weights[row - 1][col];
                    for steps in 1..=3 {
                        edges
                            .entry(Position::new(
                                row as i32,
                                col as i32,
                                Direction::Right,
                                steps,
                            ))
                            .or_insert(Vec::new())
                            .push(Edge::new(
                                Position::new((row - 1) as i32, col as i32, Direction::Up, 1),
                                weight_up,
                            ));
                    }
                }

                if row < num_rows - 1 {
                    // can turn down
                    let weight_down = weights[row + 1][col];
                    for steps in 1..=3 {
                        edges
                            .entry(Position::new(
                                row as i32,
                                col as i32,
                                Direction::Right,
                                steps,
                            ))
                            .or_insert(Vec::new())
                            .push(Edge::new(
                                Position::new((row + 1) as i32, col as i32, Direction::Down, 1),
                                weight_down,
                            ));
                    }
                }
            }

            ////////////////////////////////// DIR LEFT
            if col < (num_cols - 1) {
                if col > 0 {
                    // can still go left further
                    let weight_left = weights[row][col - 1];
                    for steps in 1..=2 {
                        edges
                            .entry(Position::new(
                                row as i32,
                                col as i32,
                                Direction::Left,
                                steps,
                            ))
                            .or_insert(Vec::new())
                            .push(Edge::new(
                                Position::new(
                                    row as i32,
                                    (col - 1) as i32,
                                    Direction::Left,
                                    steps + 1,
                                ),
                                weight_left,
                            ));
                    }
                }

                if row > 0 {
                    // can turn up
                    let weight_up = weights[row - 1][col];
                    for steps in 1..=3 {
                        edges
                            .entry(Position::new(
                                row as i32,
                                col as i32,
                                Direction::Left,
                                steps,
                            ))
                            .or_insert(Vec::new())
                            .push(Edge::new(
                                Position::new((row - 1) as i32, col as i32, Direction::Up, 1),
                                weight_up,
                            ));
                    }
                }

                if row < num_rows - 1 {
                    // can turn down
                    let weight_down = weights[row + 1][col];
                    for steps in 1..=3 {
                        edges
                            .entry(Position::new(
                                row as i32,
                                col as i32,
                                Direction::Left,
                                steps,
                            ))
                            .or_insert(Vec::new())
                            .push(Edge::new(
                                Position::new((row + 1) as i32, col as i32, Direction::Down, 1),
                                weight_down,
                            ));
                    }
                }
            }

            if row == 0 && col == 0 {
                // initial steps
                edges.insert(
                    Position::new(row as i32, col as i32, Direction::None, 0),
                    vec![
                        Edge::new(Position::new(0, 1, Direction::Right, 1), weights[0][1]),
                        Edge::new(Position::new(1, 0, Direction::Down, 1), weights[1][0]),
                    ],
                );
            }
        }
    }

    edges
}

fn solve_map(map: &HashMap<Position, Vec<Edge>>, num_rows: u32, num_cols: u32) -> u64 {
    let mut pq: PriorityQueue<Position, Reverse<u64>> = PriorityQueue::new();
    let mut dist: HashMap<Position, u64> = HashMap::new();
    let mut visited: HashSet<Position> = HashSet::new();
    let start = Position::new(0, 0, Direction::None, 0);
    let mut finishing_positions = Vec::new();
    pq.push(start, Reverse(0));
    dist.insert(start, 0);
    while !pq.is_empty() {
        let (pos, _) = pq.pop().unwrap();
        let cur_dist = *dist.get(&pos).unwrap();
        let edges = map.get(&pos).expect(&format!("No edges for {:?}", pos));
        for edge in edges {
            if !visited.contains(&edge.pos) {
                let old_dist = dist.get(&edge.pos).map(|x| *x).or(Some(u64::MAX)).unwrap();
                let new_dist = cur_dist + edge.weight as u64;
                if new_dist < old_dist {
                    let cur_prio = pq.get(&edge.pos);
                    if cur_prio.is_some() {
                        pq.change_priority(&edge.pos, Reverse(new_dist));
                    } else {
                        pq.push(edge.pos, Reverse(new_dist));
                    }
                    dist.insert(edge.pos, new_dist);
                }
            }
        }
        visited.insert(pos);
        if pos.row == (num_rows - 1) as i32 && pos.col == (num_cols - 1) as i32 {
            finishing_positions.push(pos);
        }
    }
    let mut res = u64::MAX;
    for fp in finishing_positions {
        let d = *dist.get(&fp).unwrap();
        if d < res {
            res = d as u64;
        }
    }
    res
}

fn build_map_part2(weights: &Vec<Vec<u32>>) -> HashMap<Position, Vec<Edge>> {
    let mut edges: HashMap<Position, Vec<Edge>> = HashMap::new();

    // build the graph
    let num_rows = weights.len();
    let num_cols = weights[0].len();
    for row in 0..num_rows {
        for col in 0..num_cols {
            /////////////////////////////////// DIR DOWN
            if row > 0 {
                if row < (num_rows - 1) {
                    // can still go down further
                    let possible_steps_down = num_rows - 1 - row;
                    let weight_down = weights[row + 1][col];
                    for steps in 4 - possible_steps_down.min(3)..=9 {
                        edges
                            .entry(Position::new(
                                row as i32,
                                col as i32,
                                Direction::Down,
                                steps as i32,
                            ))
                            .or_insert(Vec::new())
                            .push(Edge::new(
                                Position::new(
                                    (row + 1) as i32,
                                    col as i32,
                                    Direction::Down,
                                    (steps + 1) as i32,
                                ),
                                weight_down,
                            ));
                    }
                }

                if col >= 4 {
                    // can turn left
                    let weight_left = weights[row][col - 1];
                    for steps in 4..=10 {
                        edges
                            .entry(Position::new(
                                row as i32,
                                col as i32,
                                Direction::Down,
                                steps,
                            ))
                            .or_insert(Vec::new())
                            .push(Edge::new(
                                Position::new(row as i32, (col - 1) as i32, Direction::Left, 1),
                                weight_left,
                            ));
                    }
                }

                if col < num_cols - 4 {
                    // can turn right
                    let weight_right = weights[row][col + 1];
                    for steps in 4..=10 {
                        edges
                            .entry(Position::new(
                                row as i32,
                                col as i32,
                                Direction::Down,
                                steps,
                            ))
                            .or_insert(Vec::new())
                            .push(Edge::new(
                                Position::new(row as i32, (col + 1) as i32, Direction::Right, 1),
                                weight_right,
                            ));
                    }
                }
            }

            /////////////////////////////////// DIR UP
            if row < (num_rows - 1) {
                if row > 0 {
                    // can still go up further
                    let possible_steps_up = row;
                    let weight_up = weights[row - 1][col];
                    for steps in 4 - possible_steps_up.min(3)..=9 {
                        edges
                            .entry(Position::new(
                                row as i32,
                                col as i32,
                                Direction::Up,
                                steps as i32,
                            ))
                            .or_insert(Vec::new())
                            .push(Edge::new(
                                Position::new(
                                    (row - 1) as i32,
                                    col as i32,
                                    Direction::Up,
                                    (steps + 1) as i32,
                                ),
                                weight_up,
                            ));
                    }
                }

                if col >= 4 {
                    // can turn left
                    let weight_left = weights[row][col - 1];
                    for steps in 4..=10 {
                        edges
                            .entry(Position::new(row as i32, col as i32, Direction::Up, steps))
                            .or_insert(Vec::new())
                            .push(Edge::new(
                                Position::new(row as i32, (col - 1) as i32, Direction::Left, 1),
                                weight_left,
                            ));
                    }
                }

                if col < num_cols - 4 {
                    // can turn right
                    let weight_right = weights[row][col + 1];
                    for steps in 4..=10 {
                        edges
                            .entry(Position::new(row as i32, col as i32, Direction::Up, steps))
                            .or_insert(Vec::new())
                            .push(Edge::new(
                                Position::new(row as i32, (col + 1) as i32, Direction::Right, 1),
                                weight_right,
                            ));
                    }
                }
            }

            ////////////////////////////////// DIR RIGHT
            if col > 0 {
                if col < (num_cols - 1) {
                    // can still go right further
                    let possible_steps_right = num_cols - 1 - col;
                    let weight_right = weights[row][col + 1];
                    for steps in 4 - possible_steps_right.min(3)..=9 {
                        edges
                            .entry(Position::new(
                                row as i32,
                                col as i32,
                                Direction::Right,
                                steps as i32,
                            ))
                            .or_insert(Vec::new())
                            .push(Edge::new(
                                Position::new(
                                    row as i32,
                                    (col + 1) as i32,
                                    Direction::Right,
                                    (steps + 1) as i32,
                                ),
                                weight_right,
                            ));
                    }
                }

                if row >= 4 {
                    // can turn up
                    let weight_up = weights[row - 1][col];
                    for steps in 4..=10 {
                        edges
                            .entry(Position::new(
                                row as i32,
                                col as i32,
                                Direction::Right,
                                steps,
                            ))
                            .or_insert(Vec::new())
                            .push(Edge::new(
                                Position::new((row - 1) as i32, col as i32, Direction::Up, 1),
                                weight_up,
                            ));
                    }
                }

                if row < num_rows - 4 {
                    // can turn down
                    let weight_down = weights[row + 1][col];
                    for steps in 4..=10 {
                        edges
                            .entry(Position::new(
                                row as i32,
                                col as i32,
                                Direction::Right,
                                steps,
                            ))
                            .or_insert(Vec::new())
                            .push(Edge::new(
                                Position::new((row + 1) as i32, col as i32, Direction::Down, 1),
                                weight_down,
                            ));
                    }
                }
            }

            ////////////////////////////////// DIR LEFT
            if col < (num_cols - 1) {
                if col > 0 {
                    // can still go left further
                    let possible_steps_left = col;
                    let weight_left = weights[row][col - 1];
                    for steps in 4 - possible_steps_left.min(3)..=9 {
                        edges
                            .entry(Position::new(
                                row as i32,
                                col as i32,
                                Direction::Left,
                                steps as i32,
                            ))
                            .or_insert(Vec::new())
                            .push(Edge::new(
                                Position::new(
                                    row as i32,
                                    (col - 1) as i32,
                                    Direction::Left,
                                    (steps + 1) as i32,
                                ),
                                weight_left,
                            ));
                    }
                }

                if row >= 4 {
                    // can turn up
                    let weight_up = weights[row - 1][col];
                    for steps in 4..=10 {
                        edges
                            .entry(Position::new(
                                row as i32,
                                col as i32,
                                Direction::Left,
                                steps,
                            ))
                            .or_insert(Vec::new())
                            .push(Edge::new(
                                Position::new((row - 1) as i32, col as i32, Direction::Up, 1),
                                weight_up,
                            ));
                    }
                }

                if row < num_rows - 4 {
                    // can turn down
                    let weight_down = weights[row + 1][col];
                    for steps in 4..=10 {
                        edges
                            .entry(Position::new(
                                row as i32,
                                col as i32,
                                Direction::Left,
                                steps,
                            ))
                            .or_insert(Vec::new())
                            .push(Edge::new(
                                Position::new((row + 1) as i32, col as i32, Direction::Down, 1),
                                weight_down,
                            ));
                    }
                }
            }

            if row == 0 && col == 0 {
                // initial steps
                edges.insert(
                    Position::new(row as i32, col as i32, Direction::None, 0),
                    vec![
                        Edge::new(Position::new(0, 1, Direction::Right, 1), weights[0][1]),
                        Edge::new(Position::new(1, 0, Direction::Down, 1), weights[1][0]),
                    ],
                );
            }
        }
    }

    edges
}

fn main() {
    let content = fs::read_to_string("inputs/day17.txt").unwrap();
    let weights = content
        .lines()
        .map(|x| {
            x.chars()
                .map(|y| y.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // part 1
    // let map = build_map_part1(&weights);
    // println!(
    // "{}",
    // solve_map(&map, weights.len() as u32, weights[0].len() as u32)
    // );

    // part 2
    let map = build_map_part2(&weights);
    println!(
        "{}",
        solve_map(&map, weights.len() as u32, weights[0].len() as u32)
    );
}
