use std::{collections::HashMap, fs};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Result {
    ToWorkflow { name: String },
    Accepted,
    Rejected,
}

#[derive(Debug, Clone)]
enum Rule {
    CategoryRule {
        category: String,
        sign: char,
        value: u64,
        result: Result,
    },
    DefaultRule {
        result: Result,
    },
}

#[derive(Debug, Clone)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

fn parse_workflows(lines: &[&str]) -> Vec<Workflow> {
    let mut res = Vec::new();
    for l in lines {
        let spl = l.split(|ch| ch == '{' || ch == '}').collect::<Vec<_>>();
        let name = String::from(spl[0]);
        let rs = spl[1].split(",").collect::<Vec<_>>();
        let mut rules = Vec::new();
        for r in rs {
            let r_spl = r.split(":").collect::<Vec<_>>();
            let res = match r_spl[r_spl.len() - 1] {
                "A" => Result::Accepted,
                "R" => Result::Rejected,
                s => Result::ToWorkflow {
                    name: String::from(s),
                },
            };
            let rule = if r_spl.len() == 1 {
                // default rule
                Rule::DefaultRule { result: res }
            } else {
                let cond = r_spl[0];
                let sign = if cond.contains("<") { '<' } else { '>' };
                let cond_spl = cond.split(|ch| ch == '<' || ch == '>').collect::<Vec<_>>();
                let cat = String::from(cond_spl[0]);
                let value = cond_spl[1].parse::<u64>().unwrap();
                Rule::CategoryRule {
                    category: cat,
                    sign: sign,
                    value: value,
                    result: res,
                }
            };
            rules.push(rule);
        }
        res.push(Workflow {
            name: name,
            rules: rules,
        });
    }
    res
}

fn workflows_as_map(workflows: &Vec<Workflow>) -> HashMap<String, Workflow> {
    let mut res = HashMap::new();
    for w in workflows {
        res.insert(w.name.clone(), w.clone());
    }
    res
}

fn apply_workflow(p: &HashMap<String, u64>, w: &Workflow) -> Result {
    for r in &w.rules {
        let res = match r {
            Rule::DefaultRule { result } => Some(result),
            Rule::CategoryRule {
                category,
                sign,
                value,
                result,
            } => {
                let actual_value = p[category];
                let passes = match sign {
                    '<' => actual_value < *value,
                    '>' => actual_value > *value,
                    _ => panic!("bad sign: {sign}"),
                };
                if passes {
                    Some(result)
                } else {
                    None
                }
            }
        };
        if res.is_some() {
            return res.unwrap().clone();
        }
    }
    panic!("no default rule?");
}

fn part1(workflows: &Vec<Workflow>, parts: &Vec<HashMap<String, u64>>) -> u64 {
    let mut result = 0;
    let mut workflows_map = workflows_as_map(&workflows);
    for p in parts {
        let mut w = &workflows_map[&String::from("in")];
        loop {
            let r = apply_workflow(p, w);
            match r {
                Result::Accepted => {
                    // println!("{:?} accepted", p);
                    result += p.values().sum::<u64>();
                    break;
                }
                Result::Rejected => {
                    // println!("{:?} rejected", p);
                    break;
                }
                Result::ToWorkflow { name } => {
                    w = &workflows_map[&name];
                }
            }
        }
    }
    result
}

fn main() {
    let content = fs::read_to_string("inputs/day19.txt").unwrap();
    let lines = content.lines().collect::<Vec<_>>();
    let lines_split = lines.split(|x| x.trim().is_empty()).collect::<Vec<_>>();
    let workflows = parse_workflows(lines_split[0]);
    let parts = lines_split[1]
        .iter()
        .map(|x| {
            let s = &x[1..x.len() - 1];
            let spl = s.split(",").collect::<Vec<_>>();
            spl.iter()
                .map(|y| {
                    let spl2 = y.split("=").collect::<Vec<_>>();
                    (String::from(spl2[0]), spl2[1].parse::<u64>().unwrap())
                })
                .collect::<HashMap<_, _>>()
        })
        .collect::<Vec<_>>();

    // part 1
    println!("{}", part1(&workflows, &parts));
}
