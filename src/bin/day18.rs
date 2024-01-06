use std::fs;

fn print(v: &Vec<Vec<char>>) {
    for i in 0..v.len() {
        println!("{:?}", v[i].iter().collect::<String>());
    }
}

fn part1(lines: &Vec<(&str, u32, &str)>) -> u64 {
    let mut coords = Vec::new();
    let mut row = 0;
    let mut col = 0;
    coords.push((col, row));
    for (dir, num, _) in lines {
        match *dir {
            "R" => {
                for i in 0..*num {
                    col += 1;
                    coords.push((col, row));
                }
            }
            "L" => {
                for i in 0..*num {
                    col -= 1;
                    coords.push((col, row));
                }
            }
            "U" => {
                for i in 0..*num {
                    row -= 1;
                    coords.push((col, row));
                }
            }
            "D" => {
                for i in 0..*num {
                    row += 1;
                    coords.push((col, row));
                }
            }
            _ => {
                panic!("bad dir: {dir}")
            }
        }
    }
    let det = coords
        .windows(2)
        .map(|c| {
            (c[0].0 * c[1].1 - c[0].1 * c[1].0) as i64
        })
        .collect::<Vec<_>>();
    (det.iter().sum::<i64>() / 2) as u64 + (coords.len() / 2) as u64 + 1
}

fn main() {
    let content = fs::read_to_string("inputs/day18.txt").unwrap();
    let lines = content
        .lines()
        .map(|x| {
            let mut spl = x.split(" ");
            let dir = spl.next().unwrap();
            let num: u32 = spl.next().unwrap().parse().unwrap();
            let col = spl.next().unwrap();
            let col = &col[1..col.len() - 1];
            (dir, num, col)
        })
        .collect::<Vec<_>>();

    // part 1
    println!("{}", part1(&lines));
}
