use std::fs;

fn load(v: &Vec<Vec<char>>) -> u64 {
    let mut res = 0;
    for i in 0..v.len() {
        let mult = v.len() - i;
        res += (v[i].iter().filter(|x| **x == 'O').count() * mult) as u64;
    }
    res
}

fn part1(v: &Vec<Vec<char>>) -> u64 {
    let mut vv = v.clone();
    for col in 0..vv[0].len() {
        let mut free: Option<usize> = None;
        for row in 0..vv.len() {
            let ch = vv[row][col];
            match ch {
                'O' => {
                    if free.is_some() {
                        vv[free.unwrap()][col] = 'O';
                        vv[row][col] = '.';
                        free = Some(free.unwrap() + 1);
                    }
                }
                '.' => {
                    if free.is_none() {
                        free = Some(row);
                    }
                }
                '#' => free = None,
                _ => panic!("unexpected char {ch}"),
            }
        }
    }
    load(&vv)
}

fn main() {
    let content = fs::read_to_string("inputs/day14.txt").unwrap();
    let lines = content
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    println!("{}", part1(&lines));
}
