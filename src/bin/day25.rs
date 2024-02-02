use rand::{thread_rng, Rng};
use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn parse(lines: &Vec<&str>) -> HashMap<String, Vec<String>> {
    let mut res: HashMap<String, Vec<String>> = HashMap::new();
    for line in lines {
        let spl = line.split(": ").collect::<Vec<_>>();
        let node = String::from(spl[0]);
        let spl = spl[1]
            .split(" ")
            .map(|x| String::from(x))
            .collect::<Vec<_>>();
        for node2 in spl {
            res.entry(node.clone())
                .or_insert(Vec::new())
                .push(node2.clone());
            res.entry(node2.clone())
                .or_insert(Vec::new())
                .push(node.clone());
        }
    }
    res
}

fn sorted_edge(n1: String, n2: String) -> (String, String) {
    let mut v = vec![n1, n2];
    v.sort();
    return (v[0].clone(), v[1].clone());
}

fn part1(graph: &HashMap<String, Vec<String>>) -> usize {
    let mut rng = thread_rng();
    loop {
        let mut g = graph.clone();
        let mut edges_hs = HashSet::new();
        for (n, ns) in graph {
            for n2 in ns {
                edges_hs.insert(sorted_edge(n.clone(), n2.clone()));
            }
        }
        let mut edges: Vec<_> = edges_hs.into_iter().collect();
        while g.len() > 2 {
            let e: usize = rng.gen_range(0..edges.len());
            let edge = edges.remove(e);
            // println!("--> removed edge {:?}", edge);
            // println!("edges: {:?}", edges);
            let left = &edge.0;
            let right = &edge.1;
            let new_name = format!("{}{}", edge.0, edge.1);
            // println!("new name {:?}", new_name);

            // first delete all other occurences of the same edge
            edges.retain(|x| *x != edge);

            // now replace all connections to left and right
            for edge in &mut edges {
                if *edge.0 == *left {
                    *edge = sorted_edge(new_name.clone(), edge.1.clone());
                } else if *edge.1 == *left {
                    *edge = sorted_edge(new_name.clone(), edge.0.clone());
                } else if *edge.0 == *right {
                    *edge = sorted_edge(new_name.clone(), edge.1.clone());
                } else if *edge.1 == *right {
                    *edge = sorted_edge(new_name.clone(), edge.0.clone());
                }
            }

            // modify the graph
            let mut connections = Vec::new();
            for other in g.remove(left).unwrap() {
                if other != *right {
                    connections.push(other.clone());
                }
            }
            for other in g.remove(right).unwrap() {
                if other != *left {
                    connections.push(other.clone());
                }
            }
            for c in connections.iter().collect::<HashSet<_>>() {
                // go through unique connections
                for other in g.get_mut(c).unwrap() {
                    if *other == *left || *other == *right {
                        *other = new_name.clone();
                    }
                }
            }

            g.insert(new_name.clone(), connections.clone());

            // println!("edges: {:?}", edges);
            // println!("graph: {:?}", g);
        }
        // println!("final edges {:?}", edges);
        // println!("final graph {:?}", g);
        // println!("{}", edges.len());
        if edges.len() == 3 {
            return g.keys().map(|x| x.len() /3 ).product::<usize>();
        }
    }
    0
}

fn main() {
    let content = fs::read_to_string("inputs/day25.txt").unwrap();
    let lines = content.lines().collect::<Vec<_>>();
    let graph = parse(&lines);
    println!("{:?}", part1(&graph));
}
