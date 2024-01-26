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

fn count_state(state: &HashMap<String, (u64, u64)>) -> u64 {
    let mut res = 1;
    for k in vec!["x", "m", "a", "s"] {
        let s = state[&String::from(k)];
        res *= (s.1 - s.0 + 1);
    }
    res
}

fn do_part2(
    state: HashMap<String, (u64, u64)>,
    workflows_map: &HashMap<String, Workflow>,
    workflow: &Workflow,
    next_rule: usize,
) -> u64 {
    let rule = &workflow.rules[next_rule];
    match rule {
        Rule::DefaultRule { result } => match result {
            Result::Accepted => {
                count_state(&state)
            }
            Result::Rejected => 0,
            Result::ToWorkflow { name } => {
                let new_workflow = &workflows_map[name];
                do_part2(state, workflows_map, new_workflow, 0)
            }
        },
        Rule::CategoryRule {
            category,
            sign,
            value,
            result,
        } => {
            let mut new_pos_state = state.clone();
            match sign {
                '<' => new_pos_state.get_mut(category).unwrap().1 = value - 1,
                '>' => new_pos_state.get_mut(category).unwrap().0 = value + 1,
                _ => panic!("bad sign: {sign}"),
            };
            let mut res = 0;
            match result {
                Result::Accepted => {
                    println!("accepted state: {:?}", new_pos_state);
                    res += count_state(&new_pos_state);
                }
                Result::Rejected => {}
                Result::ToWorkflow { name } => {
                    res += do_part2(new_pos_state, workflows_map, &workflows_map[name], 0);
                }
            }

            if next_rule < workflow.rules.len() - 1 {
                // should always be the case?
                let mut new_neg_state = state.clone();
                match sign {
                    '<' => new_neg_state.get_mut(category).unwrap().0 = *value,
                    '>' => new_neg_state.get_mut(category).unwrap().1 = *value,
                    _ => panic!("bad sign: {sign}"),
                };
                res += do_part2(new_neg_state, workflows_map, workflow, next_rule + 1)
            }
            res
        }
    }
}

fn part2(workflows: &Vec<Workflow>) -> u64 {
    let mut result = 0;
    let mut workflows_map = workflows_as_map(&workflows);
    let state = HashMap::from([
        (String::from("x"), (1, 4000)),
        (String::from("m"), (1, 4000)),
        (String::from("a"), (1, 4000)),
        (String::from("s"), (1, 4000)),
    ]);
    do_part2(
        state,
        &workflows_map,
        &workflows_map[&String::from("in")],
        0,
    )
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
    // println!("{}", part1(&workflows, &parts));

    // part 2
    println!("{}", part2(&workflows));
}
