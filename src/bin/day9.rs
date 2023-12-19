use std::fs;

fn calc_next(seq: &Vec<i64>) -> i64 {
    let mut sequences = Vec::new();
    let mut cur = seq.clone();
    let mut next = Vec::new();
    loop {
        next = cur.windows(2).map(|x| x[1] - x[0]).collect::<Vec<_>>();
        sequences.push(cur);
        cur = next;
        if cur.iter().all(|x| *x == 0) {
            break
        }
    }
    sequences.push(cur);
    let l = sequences.len();
    sequences[l-1].push(0);
    for i in (0..(l-1)).rev() {
        let nextseq = &sequences[i+1];
        let delta = nextseq[nextseq.len()-1];
        let mut curseq = &mut sequences[i];
        let last = curseq[curseq.len()-1];
        curseq.push(last + delta);
    }
    let first = &sequences[0];
    first[first.len()-1]
}

fn calc_fst(seq: &Vec<i64>) -> i64 {
    let mut sequences = Vec::new();
    let mut cur = seq.clone();
    let mut next = Vec::new();
    loop {
        next = cur.windows(2).map(|x| x[1] - x[0]).collect::<Vec<_>>();
        sequences.push(cur);
        cur = next;
        if cur.iter().all(|x| *x == 0) {
            break
        }
    }
    sequences.push(cur);
    let l = sequences.len();
    sequences[l-1].insert(0, 0);
    for i in (0..(l-1)).rev() {
        let nextseq = &sequences[i+1];
        let delta = nextseq[0];
        let mut curseq = &mut sequences[i];
        let last = curseq[0];
        curseq.insert(0, last - delta);
    }
    let first = &sequences[0];
    first[0]
}

fn part1(values: &Vec<Vec<i64>>) -> i64 {
    let mut res = 0;
    for v in values {
        let n = calc_next(v);
        res += n;
    }
    res
}

fn part2(values: &Vec<Vec<i64>>) -> i64 {
    let mut res = 0;
    for v in values {
        let n = calc_fst(v);
        res += n;
    }
    res
}

fn main() {
    let content = fs::read_to_string("inputs/day9.txt").unwrap();
    let lines = content.lines().collect::<Vec<_>>();
    let values = lines
        .iter()
        .map(|x| {
            x.split(" ")
                .map(|y| y.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // part 1
    // println!("{}", part1(&values));

    // part 2
    println!("{}", part2(&values));
}
