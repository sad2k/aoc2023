use std::{
    collections::{HashMap, HashSet, VecDeque}, fs, time::Instant
};

#[derive(Debug)]
struct Graph {
    nodes: HashMap<(i32, i32), Vec<(i32, i32, i32)>>,
}

#[derive(Debug)]
struct Path {
    previous_nodes: HashSet<(i32, i32)>,
    previous_costs: Vec<i32>,
    last: (i32, i32, i32),
}

fn parse_graph(lines: &Vec<Vec<char>>, use_slopes: bool) -> Graph {
    let mut node_edges: HashMap<(i32, i32), Vec<(i32, i32, i32)>> = HashMap::new();
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
                        v.push(((row - 1) as i32, col as i32, 1));
                    }
                    if row < (lines.len() - 1) && lines[row + 1][col] != '#' {
                        v.push(((row + 1) as i32, col as i32, 1));
                    }
                    if col > 0 && lines[row][col - 1] != '#' {
                        v.push((row as i32, (col - 1) as i32, 1));
                    }
                    if col < (line.len() - 1) && lines[row][col + 1] != '#' {
                        v.push((row as i32, (col + 1) as i32, 1));
                    }
                    Some(v)
                }
                '>' => Some(vec![(row as i32, (col + 1) as i32, 1)]),
                '<' => Some(vec![(row as i32, (col - 1) as i32, 1)]),
                'v' => Some(vec![((row + 1) as i32, col as i32, 1)]),
                '^' => Some(vec![((row - 1) as i32, col as i32, 1)]),
                _ => None,
            };

            if edges.is_some() {
                node_edges.insert((row as i32, col as i32), edges.unwrap());
            }
        }
    }
    Graph { nodes: node_edges }
}

fn solve(graph: &Graph, dest: (i32, i32)) -> i32 {
    let mut q = VecDeque::new();
    q.push_front(Path {
        previous_nodes: HashSet::new(),
        previous_costs: Vec::new(),
        last: (0, 1, 0),
    });
    let mut cnt = 0;
    let mut res = 0;
    while !q.is_empty() {
        let p = q.pop_front().unwrap();
        if p.last.0 == dest.0 && p.last.1 == dest.1 {
            let path_len = p.previous_costs.iter().sum::<i32>() + p.last.2;
            res = res.max(path_len);
            cnt += 1;
            // println!("{}: max so far {}", cnt, res)
        } else {
            for e in &graph.nodes[&(p.last.0, p.last.1)] {
                if !p.previous_nodes.contains(&(e.0, e.1)) {
                    let mut new_prev_nodes = p.previous_nodes.clone();
                    let mut new_prev_costs = p.previous_costs.clone();
                    new_prev_nodes.insert((p.last.0, p.last.1));
                    new_prev_costs.push(p.last.2);
                    q.push_front(Path {
                        previous_nodes: new_prev_nodes,
                        previous_costs: new_prev_costs,
                        last: *e,
                    });
                }
            }
        }
    }
    res as i32
}

fn contract(graph: &Graph) -> Graph {
    let mut new_nodes = graph.nodes.clone();
    let mut to_contract = VecDeque::new();
    for (n, edges) in &graph.nodes {
        if edges.len() == 2 {
            to_contract.push_back(n);
        }
    }
    while !to_contract.is_empty() {
        let n = to_contract.pop_front().unwrap();
        let edges = new_nodes.remove(n).unwrap();
        let left = edges[0];
        let right = edges[1];
        let mut left_edges = new_nodes.get_mut(&(left.0, left.1)).unwrap();
        left_edges.retain(|x| x.0 != n.0 || x.1 != n.1);
        left_edges.push((right.0, right.1, left.2 + right.2));
        let mut right_edges = new_nodes.get_mut(&(right.0, right.1)).unwrap();
        right_edges.retain(|x| x.0 != n.0 || x.1 != n.1);
        right_edges.push((left.0, left.1, left.2 + right.2));
    }
    Graph { nodes: new_nodes }
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
    let graph2 = contract(&graph2);
    let start = Instant::now();
    println!(
        "{}",
        solve(
            &graph2,
            ((lines.len() - 1) as i32, (lines[0].len() - 2) as i32)
        )
    );
    let duration = start.elapsed();
    println!("Finished part 2 in {:?}", duration);
}
