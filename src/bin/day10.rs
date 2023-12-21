use std::{collections::HashMap, fs, thread::current};

fn parse(lines: &Vec<Vec<char>>) -> (HashMap<(usize, usize), Vec<(usize, usize)>>, (usize, usize)) {
    let mut map: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
    let mut start: Option<(usize, usize)> = None;
    for i in 0..lines.len() {
        let row = &lines[i];
        for j in 0..row.len() {
            let ch = row[j];
            let current_coord = (i, j);
            match ch {
                'F' => {
                    if j < row.len() - 1 {
                        let right = row[j + 1];
                        if right == '-' || right == '7' || right == 'J' {
                            map.entry(current_coord)
                                .or_insert(Vec::new())
                                .push((i, j + 1));
                        }
                    }
                    if i < lines.len() - 1 {
                        let next_row = &lines[i + 1];
                        let down = next_row[j];
                        if down == '|' || down == 'L' || down == 'J' {
                            map.entry(current_coord)
                                .or_insert(Vec::new())
                                .push((i + 1, j));
                        }
                    }
                }
                '7' => {
                    if j > 0 {
                        let left = row[j - 1];
                        if left == '-' || left == 'F' || left == 'L' {
                            map.entry(current_coord)
                                .or_insert(Vec::new())
                                .push((i, j - 1));
                        }
                    }
                    if i < lines.len() - 1 {
                        let next_row = &lines[i + 1];
                        let down = next_row[j];
                        if down == '|' || down == 'L' || down == 'J' {
                            map.entry(current_coord)
                                .or_insert(Vec::new())
                                .push((i + 1, j));
                        }
                    }
                }
                'J' => {
                    if i > 0 {
                        let prev_row = &lines[i - 1];
                        let up = prev_row[j];
                        if up == '|' || up == '7' || up == 'F' {
                            map.entry(current_coord)
                                .or_insert(Vec::new())
                                .push((i - 1, j));
                        }
                    }
                    if j > 0 {
                        let left = row[j - 1];
                        if left == '-' || left == 'F' || left == 'L' {
                            map.entry(current_coord)
                                .or_insert(Vec::new())
                                .push((i, j - 1));
                        }
                    }
                }
                'L' => {
                    if i > 0 {
                        let prev_row = &lines[i - 1];
                        let up = prev_row[j];
                        if up == '|' || up == '7' || up == 'F' {
                            map.entry(current_coord)
                                .or_insert(Vec::new())
                                .push((i - 1, j));
                        }
                    }
                    if j < row.len() - 1 {
                        let right = row[j + 1];
                        if right == '-' || right == '7' || right == 'J' {
                            map.entry(current_coord)
                                .or_insert(Vec::new())
                                .push((i, j + 1));
                        }
                    }
                }
                '-' => {
                    if j > 0 {
                        let left = row[j - 1];
                        if left == '-' || left == 'F' || left == 'L' {
                            map.entry(current_coord)
                                .or_insert(Vec::new())
                                .push((i, j - 1));
                        }
                    }
                    if j < row.len() - 1 {
                        let right = row[j + 1];
                        if right == '-' || right == '7' || right == 'J' {
                            map.entry(current_coord)
                                .or_insert(Vec::new())
                                .push((i, j + 1));
                        }
                    }
                }
                '|' => {
                    if i > 0 {
                        let prev_row = &lines[i - 1];
                        let up = prev_row[j];
                        if up == '|' || up == '7' || up == 'F' {
                            map.entry(current_coord)
                                .or_insert(Vec::new())
                                .push((i - 1, j));
                        }
                    }
                    if i < lines.len() - 1 {
                        let next_row = &lines[i + 1];
                        let down = next_row[j];
                        if down == '|' || down == 'L' || down == 'J' {
                            map.entry(current_coord)
                                .or_insert(Vec::new())
                                .push((i + 1, j));
                        }
                    }
                }
                'S' => {
                    start = Some(current_coord);
                    let mut start_connections = 0;
                    if i > 0 {
                        let prev_row = &lines[i - 1];
                        let up = prev_row[j];
                        if up == '|' || up == '7' || up == 'F' {
                            map.entry(current_coord)
                                .or_insert(Vec::new())
                                .push((i - 1, j));
                            map.entry((i - 1, j))
                                .or_insert(Vec::new())
                                .push(current_coord);
                            start_connections += 1;
                        }
                    }
                    if i < lines.len() - 1 {
                        let next_row = &lines[i + 1];
                        let down = next_row[j];
                        if down == '|' || down == 'L' || down == 'J' {
                            map.entry(current_coord)
                                .or_insert(Vec::new())
                                .push((i + 1, j));
                            map.entry((i + 1, j))
                                .or_insert(Vec::new())
                                .push(current_coord);
                            start_connections += 1;
                        }
                    }
                    if j > 0 {
                        let left = row[j - 1];
                        if left == '-' || left == 'F' || left == 'L' {
                            map.entry(current_coord)
                                .or_insert(Vec::new())
                                .push((i, j - 1));
                            map.entry((i, j - 1))
                                .or_insert(Vec::new())
                                .push(current_coord);
                            start_connections += 1;
                        }
                    }
                    if j < row.len() - 1 {
                        let right = row[j + 1];
                        if right == '-' || right == '7' || right == 'J' {
                            map.entry(current_coord)
                                .or_insert(Vec::new())
                                .push((i, j + 1));
                            map.entry((i, j + 1))
                                .or_insert(Vec::new())
                                .push(current_coord);
                            start_connections += 1;
                        }
                    }
                    if start_connections != 2 {
                        panic!("bad number of connections to start: {start_connections}");
                    }
                }
                _ => {
                    // ignore
                }
            }
        }
    }
    (map, start.unwrap())
}

fn find_length(map: &HashMap<(usize, usize), Vec<(usize, usize)>>, start: (usize, usize)) -> u64 {
    let mut res = 1;
    let mut prev = start;
    let mut cur = map[&prev][0];
    while cur != start {
        let connections = &map[&cur];
        if connections.len() != 2 {
            panic!("bad connections for coord {:?}: {:?}", cur, connections);
        }
        let next = if connections[0] == prev {
            connections[1]
        } else {
            connections[0]
        };
        prev = cur;
        cur = next;
        res += 1;
    }
    res
}

fn main() {
    let content = fs::read_to_string("inputs/day10.txt").unwrap();
    let lines = content
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let (map, start) = parse(&lines);

    // part 1
    let farthest = find_length(&map, start) / 2;
    println!("{farthest}");
}

