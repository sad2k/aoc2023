use std::{collections::HashMap, fs};

fn parse_nums(s: &str) -> Vec<u64> {
    s.split(" ")
        .map(|x| x.trim().parse::<u64>().unwrap())
        .collect()
}

fn resolve_rule(src: u64, rules: &Vec<Vec<u64>>) -> u64 {
    if src < rules[0][1] || src >= (rules[rules.len()-1][1] + rules[rules.len()-1][2]) {
        return src;
    }
    // for rule in rules {
        // if src >= rule[1] && src < (rule[1] + rule[2]) {
            // return rule[0] + src - rule[1];
        // }
    // }
    match rules.binary_search_by_key(&src, |x| x[1]) {
        Ok(n) => rules[n][0],
        Err(n) => if n == 0 {
            // shouldn't happen
            src
        } else {
            let rule = &rules[n-1];
            if src >= rule[1] && src < (rule[1] + rule[2]) {
                rule[0] + src - rule[1]
            } else {
                src
            }
        }
    }
}

fn resolve_seed(seed: u64, rules: &Vec<Vec<Vec<u64>>>) -> u64 {
    let mut res = seed;

    for rule in rules {
        res = resolve_rule(res, rule);
    }
    res
}

struct Parsed {
    seeds: Vec<u64>,
    rules: Vec<Vec<Vec<u64>>>,
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
    let mut rules_vec: Vec<Vec<Vec<u64>>> = Vec::new();
    for i in 1..rule_order.len() {
        let key = (rule_order[i - 1].clone(), rule_order[i].clone());
        let mut key_rules = (&rules[&key]).clone();
        key_rules.sort_by_key(|x| x[1]);
        rules_vec.push(key_rules);
    }
    Parsed {
        seeds,
        rules: rules_vec,
    }
}

fn part1(s: &str) -> u64 {
    let p = parse(s);
    p.seeds
        .iter()
        .map(|x| resolve_seed(*x, &p.rules))
        .min()
        .unwrap()
}

fn part2(s: &str) -> u64 {
    let p = parse(s);
    let seeds = p.seeds;
    let mut res = u64::MAX;
    for i in (0..(seeds.len())).step_by(2) {
        let start = seeds[i];
        let sz = seeds[i + 1];
        println!("calculating {} {}", start, sz);
        let mut cnt = 0;
        for j in start..(start + sz) {
            let l = resolve_seed(j, &p.rules);
            if l < res {
                res = l;
            }
            cnt += 1;
            if cnt % 10000000 == 0 {
                println!("{}", cnt);
            }
        }
    }
    res
    // p.seeds.iter().map(|x| resolve_seed(*x, &p.rules)).min().unwrap()
}

fn main() {
    let contents = fs::read_to_string("inputs/day5.txt").unwrap();

    // part 1
    // println!("{}", part1(&contents));

    // part 2
    println!("{}", part2(&contents));
}
