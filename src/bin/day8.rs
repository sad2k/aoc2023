use std::{fs, collections::HashMap};



fn part1(rules_chars: &Vec<char>, graph: &HashMap<String, (String, String)>) -> u64 {

    let aaa = String::from("AAA");
    let zzz = String::from("ZZZ");
    let mut cur = &aaa;
    let mut idx = 0;
    let mut iter = 0;
    loop {
        // println!("{}", cur);
        if cur == &zzz {
            break;
        }
        if idx == rules_chars.len() {
            idx = 0;
        }
        let rule = rules_chars[idx];
        let next = match rule {
            'L' => &graph[cur].0,
            'R' => &graph[cur].1,
            _ => panic!("bad rule {rule}") 
        };
        cur = next;
        iter += 1;
        idx += 1;
    }
    iter
}



fn main() {
    let content = fs::read_to_string("inputs/day8.txt").unwrap();
    let lines = content.lines().collect::<Vec<_>>();
    let lines_split = lines.split(|x| x.trim().is_empty()).collect::<Vec<_>>();
    let rules = lines_split[0][0];
    let rules_chars = rules.chars().collect::<Vec<_>>();
    let mut graph: HashMap<String, (String, String)> = HashMap::new();
    for s in lines_split[1] {
        let spl = s.split(" = ").collect::<Vec<_>>();
        let from = spl[0];
        let to = spl[1];
        let to = &to[1..to.len()-1];
        let to_spl = to.split(", ").collect::<Vec<_>>();
        let left = to_spl[0];
        let right = to_spl[1];
        graph.insert(String::from(from), (String::from(left), String::from(right)));
    }

    // part 1
    println!("{}", part1(&rules_chars, &graph));

    // part 2
    // println!("{}", part2(&lines));

}