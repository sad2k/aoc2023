use std::fs;

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
    println!("{}", part1(&lines));

    // part 2
    let mut t = lines.clone();
    tilt_vertical(&mut t, false);
    print(&t);
}
