use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

#[derive(Debug)]
struct Graph {
    node_edges: HashMap<(i32, i32), Vec<(i32, i32)>>,
}

#[derive(Debug)]
struct Path {
    previous: HashSet<(i32, i32)>,
    last: (i32, i32),
}

fn parse_graph(lines: &Vec<Vec<char>>, use_slopes: bool) -> Graph {
    let mut node_edges: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();
    for row in 0..lines.len() {
        let line = &lines[row];
        for col in 0..line.len() {
            let mut ch = line[col];
            if !use_slopes {
                if ch != '#' {
                    ch = '.';
                }
            }
            let edges = match ch {
                '.' => {
                    let mut v = Vec::new();
                    if row > 0 && lines[row - 1][col] != '#' {
                        v.push(((row - 1) as i32, col as i32));
                    }
                    if row < (lines.len() - 1) && lines[row + 1][col] != '#' {
                        v.push(((row + 1) as i32, col as i32));
                    }
                    if col > 0 && lines[row][col - 1] != '#' {
                        v.push((row as i32, (col - 1) as i32));
                    }
                    if col < (line.len() - 1) && lines[row][col + 1] != '#' {
                        v.push((row as i32, (col + 1) as i32));
                    }
                    Some(v)
                }
                '>' => Some(vec![(row as i32, (col + 1) as i32)]),
                '<' => Some(vec![(row as i32, (col - 1) as i32)]),
                'v' => Some(vec![((row + 1) as i32, col as i32)]),
                '^' => Some(vec![((row - 1) as i32, col as i32)]),
                _ => None,
            };

            if edges.is_some() {
                node_edges.insert((row as i32, col as i32), edges.unwrap());
            }
        }
    }
    Graph { node_edges }
}

fn solve(graph: &Graph, dest: (i32, i32)) -> i32 {
    let mut q = VecDeque::new();
    q.push_front(Path {
        previous: HashSet::new(),
        last: (0, 1),
    });
    let mut cnt = 0;
    let mut res = 0;
    while !q.is_empty() {
        let p = q.pop_front().unwrap();
        if p.last == dest {
            res = res.max(p.previous.len());
            cnt += 1;
            println!("{}: max so far {}", cnt, res)
        } else {
            for e in &graph.node_edges[&p.last] {
                if !p.previous.contains(e) {
                    let mut new_prev = p.previous.clone();
                    new_prev.insert(p.last);
                    q.push_front(Path {
                        previous: new_prev,
                        last: *e,
                    });
                }
            }
        }
    }
    res as i32
}

fn main() {
    let content = fs::read_to_string("inputs/day23.txt").unwrap();
    let lines = content
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let graph = parse_graph(&lines, true);

    // part 1
    // println!("{}", part1(&graph, ((lines.len()-1) as i32, (lines[0].len()-2) as i32)));

    // part 2
    let graph2 = parse_graph(&lines, false);
    println!(
        "{}",
        solve(
            &graph2,
            ((lines.len() - 1) as i32, (lines[0].len() - 2) as i32)
        )
    );
}
