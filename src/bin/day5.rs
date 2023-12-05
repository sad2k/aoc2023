use std::{collections::HashMap, fs};

fn parse_nums(s: &str) -> Vec<u64> {
    s.split(" ")
        .map(|x| x.trim().parse::<u64>().unwrap())
        .collect()
}

fn resolve_rule(src: u64, rules: &Vec<Vec<u64>>) -> u64 {
    for rule in rules {
        if src >= rule[1] && src < (rule[1] + rule[2]) {
            return rule[0] + src - rule[1];
        }
    }
    src
}

fn resolve_seed(seed: u64, rules: &HashMap<(String, String), Vec<Vec<u64>>>) -> u64 {
    let rule_order = vec![
        String::from("seed"),
        String::from("soil"),
        String::from("fertilizer"),
        String::from("water"),
        String::from("light"),
        String::from("temperature"),
        String::from("humidity"),
        String::from("location"),
    ];

    let mut res = seed;

    for i in 1..rule_order.len() {
        let key = (rule_order[i-1].clone(), rule_order[i].clone());
        let key_rules = &rules[&key];
        res = resolve_rule(res, key_rules);
    }
    res
}

struct Parsed {
    seeds: Vec<u64>,
    rules: HashMap<(String, String), Vec<Vec<u64>>>
}

fn parse(s: &str) -> Parsed {
    let blocks: Vec<_> = s.split("\n\n").collect();
    let seeds = blocks[0].split(": ").collect::<Vec<_>>()[1];
    let seeds = seeds
        .split(" ")
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let mut rules: HashMap<(String, String), Vec<Vec<u64>>> = HashMap::new();
    for i in 1..blocks.len() {
        let block_lines = blocks[i].lines().collect::<Vec<_>>();
        let header = block_lines[0].split(" ").next().unwrap();
        let header = header.split("-").collect::<Vec<_>>();
        let rule_key = (String::from(header[0]), String::from(header[2]));
        rules.insert(
            rule_key,
            block_lines[1..]
                .iter()
                .map(|x| parse_nums(*x))
                .collect::<Vec<_>>(),
        );
    }
    Parsed {
        seeds, rules
    }
}

fn part1(s: &str) -> u64 {
    let p = parse(s);
    p.seeds.iter().map(|x| resolve_seed(*x, &p.rules)).min().unwrap()
}

fn main() {
    let contents = fs::read_to_string("inputs/day5.txt").unwrap();

    // part 1
    println!("{}", part1(&contents));
}
