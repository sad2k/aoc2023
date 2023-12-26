use std::fs;

fn solve0(v: &Vec<u64>) -> Option<usize> {
    let mut best: Option<usize> = None;
    for i in 0..(v.len() - 1) {
        let max_len = (i + 1).min(v.len() - i - 1);
        let slice1 = &v[0..i + 1].iter().rev().map(|x| *x).collect::<Vec<_>>()[0..max_len];
        let slice2 = &v[i + 1..(v.len()).min(i + 1 + max_len)];
        if (slice1 == slice2) {
            if let Some(b) = best {
                if i + 1 > b {
                    best = Some(i + 1);
                }
            } else {
                best = Some(i + 1);
            }
        }
    }
    best
}

fn solve(pattern: &Vec<String>) -> u64 {
    // horizontal
    let binary = pattern
        .iter()
        .map(|x| x.replace("#", "1").replace(".", "0"))
        .collect::<Vec<_>>();
    let horiz = binary
        .iter()
        .map(|x| u64::from_str_radix(&x, 2).unwrap())
        .collect::<Vec<_>>();
    let best_horiz = solve0(&horiz);
    if best_horiz.is_some() {
        return (best_horiz.unwrap() * 100) as u64;
    }

    // vertical
    let mut vert: Vec<String> = Vec::new();
    let binary_chars = binary
        .iter()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    for i in 0..binary_chars[0].len() {
        let mut s = String::new();
        for j in 0..binary_chars.len() {
            s.push(binary_chars[j][i]);
        }
        vert.push(s);
    }
    let vert_nums = vert
        .iter()
        .map(|x| u64::from_str_radix(&x, 2).unwrap())
        .collect::<Vec<_>>();

    return solve0(&vert_nums).unwrap() as u64;
}

fn part1(lines: &Vec<Vec<String>>) -> u64 {
    let mut res = 0;
    for pattern in lines {
        res += solve(pattern);
    }
    res
}

fn main() {
    let content = fs::read_to_string("inputs/day13.txt").unwrap();
    let lines = content.lines().collect::<Vec<_>>();
    let groups = lines
        .split(|x| x.trim().is_empty())
        .map(|x| x.iter().map(|y| String::from(*y)).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // part 1
    println!("{}", part1(&groups));
}
