use std::{collections::HashMap, fs, str::Lines};

struct Solution {
    pattern: Vec<char>,
    rules: Vec<u64>,
    cache: HashMap<(usize, usize, u64), u64>,
}

impl Solution {
    fn new(s: &str, multiplier: usize) -> Solution {
        let mut spl = s.split(" ");
        let pattern = spl.next().unwrap();
        let rules = spl
            .next()
            .unwrap()
            .split(",")
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        let pattern_chars = pattern.chars().collect::<Vec<_>>();
        Solution {
            pattern: if multiplier > 1 {
                let mut vec = Vec::new();
                for i in 0..multiplier {
                    if i > 0 {
                        vec.push('?');
                    }
                    vec.extend(&pattern_chars);
                }
                vec
            } else {
                pattern_chars
            },
            rules: if multiplier > 1 {
                let mut vec = Vec::new();
                for i in 0..multiplier {
                    vec.extend(&rules);
                }
                vec
            } else {
                rules
            },
            cache: HashMap::new(),
        }
    }

    fn solve0(&mut self, start_idx: usize, start_rule: usize, broken_count: u64) -> u64 {
        // println!("{} {} {}", start_idx, start_rule, broken_count);
        let mut current_broken_count = broken_count;
        let mut current_rule = start_rule;
        for i in start_idx..self.pattern.len() {
            let ch = self.pattern[i];
            match ch {
                '#' => {
                    if current_rule == self.rules.len() {
                        // no broken expected and here's one
                        return 0;
                    } else {
                        current_broken_count += 1;
                        if current_broken_count > self.rules[current_rule] {
                            // too many broken, doesn't match
                            return 0;
                        }
                    }
                }
                '.' => {
                    if current_broken_count != 0 {
                        if current_broken_count != self.rules[current_rule] {
                            return 0;
                        }
                        current_broken_count = 0;
                        current_rule += 1;
                    }
                }
                '?' => {
                    if current_broken_count != 0 {
                        // broken group is started
                        if current_broken_count < self.rules[current_rule] {
                            // need more broken here
                            // assume it's a #
                            current_broken_count += 1;
                        } else if current_broken_count == self.rules[current_rule] {
                            // had enough broken already
                            // assume it's a .
                            current_broken_count = 0;
                            current_rule += 1;
                        }
                    } else {
                        if current_rule == self.rules.len() {
                            // no more broken allowed so can only be a .
                        } else {
                            // can either be a # (to start a new group) or a .
                            return self.solve0Cached(i + 1, current_rule, 1)
                                + self.solve0Cached(i + 1, current_rule, 0);
                        }
                    }
                }
                _ => {
                    panic!("unexpected char: {ch}");
                }
            }
        }
        if current_broken_count > 0 {
            if current_rule == self.rules.len() {
                return 0;
            } else {
                if current_broken_count == self.rules[current_rule] {
                    current_broken_count = 0;
                    current_rule += 1;
                }
            }
        }
        if current_rule == self.rules.len() {
            1
        } else {
            0
        }
    }

    fn solve0Cached(&mut self, start_idx: usize, start_rule: usize, broken_count: u64) -> u64 {
        let cached_solution = self.cache.get(&(start_idx, start_rule, broken_count));
        if cached_solution.is_some() {
            return *cached_solution.unwrap();
        } else {
            let sol = self.solve0(start_idx, start_rule, broken_count);
            self.cache
                .insert((start_idx, start_rule, broken_count), sol);
            return sol;
        }
    }

    fn solve(&mut self) -> u64 {
        self.solve0Cached(0, 0, 0)
    }
}

fn part1(lines: &Vec<&str>) {
    let mut sum = 0;
    for line in lines {
        let mut sol = Solution::new(line, 1);
        let sol_res = sol.solve();
        sum += sol_res;
    }
    println!("{}", sum);
}

fn part2(lines: &Vec<&str>) {
    let mut sum = 0;
    for line in lines {
        let mut sol = Solution::new(line, 5);
        let sol_res = sol.solve();
        sum += sol_res;
    }
    println!("{}", sum);
}

fn main() {
    let content = fs::read_to_string("inputs/day12.txt").unwrap();
    let parsed = content.lines().collect::<Vec<_>>();

    // part 1
    // part1(&parsed);

    // part 2
    part2(&parsed);
}
