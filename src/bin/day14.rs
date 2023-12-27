use std::{collections::HashMap, fs};

fn load(v: &Vec<Vec<char>>) -> u64 {
    let mut res = 0;
    for i in 0..v.len() {
        let mult = v.len() - i;
        res += (v[i].iter().filter(|x| **x == 'O').count() * mult) as u64;
    }
    res
}

fn tilt_vertical(mut v: &mut Vec<Vec<char>>, towards_beginning: bool) -> () {
    for col in 0..v[0].len() {
        let mut free: Option<usize> = None;
        let rng = if towards_beginning {
            (0..v.len()).collect::<Vec<_>>()
        } else {
            (0..v.len()).rev().collect::<Vec<_>>()
        };
        let delta = if towards_beginning { 1 } else { -1 };
        for row in rng {
            let ch = v[row][col];
            match ch {
                'O' => {
                    if free.is_some() {
                        v[free.unwrap()][col] = 'O';
                        v[row][col] = '.';
                        free = Some((free.unwrap() as i32 + delta) as usize);
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
}

fn tilt_horizontal(mut v: &mut Vec<Vec<char>>, towards_beginning: bool) -> () {
    for row in 0..v.len() {
        let mut free: Option<usize> = None;
        let rng = if towards_beginning {
            (0..v[0].len()).collect::<Vec<_>>()
        } else {
            (0..v[0].len()).rev().collect::<Vec<_>>()
        };
        let delta = if towards_beginning { 1 } else { -1 };
        for col in rng {
            let ch = v[row][col];
            match ch {
                'O' => {
                    if free.is_some() {
                        v[row][free.unwrap()] = 'O';
                        v[row][col] = '.';
                        free = Some((free.unwrap() as i32 + delta) as usize);
                    }
                }
                '.' => {
                    if free.is_none() {
                        free = Some(col);
                    }
                }
                '#' => free = None,
                _ => panic!("unexpected char {ch}"),
            }
        }
    }
}

fn tilt_cycle(mut v: &mut Vec<Vec<char>>) {
    tilt_vertical(&mut v, true);
    tilt_horizontal(&mut v, true);
    tilt_vertical(&mut v, false);
    tilt_horizontal(&mut v, false);
}

fn part2(v: &Vec<Vec<char>>) -> u64 {
    let mut vv = v.clone();
    let mut cache: HashMap<Vec<Vec<char>>, (i32, Vec<Vec<char>>)> = HashMap::new();
    let mut cache2: HashMap<i32, Vec<Vec<char>>> = HashMap::new();
    let mut orig: i32 = 0;
    let mut i = 0;
    let mut cycle_start: Option<i32> = None;
    let mut cycle_length: i32;

    loop {
        if cache.contains_key(&vv) {
            (orig, vv) = cache.get(&vv).unwrap().clone();
            if cycle_start.is_none() {
                cycle_start = Some(i);
                cycle_length = i-orig;
                break;
            } 
        } else {
            let key = vv.clone();
            tilt_cycle(&mut vv);
            cache.insert(key, (i, vv.clone()));
            cache2.insert(i, vv.clone());
        }
        i += 1;
    }
    let actual_cycle_start = cycle_start.unwrap() - cycle_length;
    let real = ((1000000000 -1 - actual_cycle_start) % cycle_length) + actual_cycle_start;
    let cached = &cache2[&real];
    load(&cached)
}

fn part1(v: &Vec<Vec<char>>) -> u64 {
    let mut vv = v.clone();
    tilt_vertical(&mut vv, true);
    load(&vv)
}

fn print(v: &Vec<Vec<char>>) {
    for i in 0..v.len() {
        println!("{:?}", v[i].iter().collect::<String>());
    }
}

fn main() {
    let content = fs::read_to_string("inputs/day14.txt").unwrap();
    let lines = content
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // part 1
    // println!("{}", part1(&lines));

    // part 2
    println!("{}", part2(&lines));
}
