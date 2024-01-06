use std::fs;

fn print(v: &Vec<Vec<char>>) {
    for i in 0..v.len() {
        println!("{:?}", v[i].iter().collect::<String>());
    }
}

fn part1(lines: &Vec<(&str, u32, &str)>) -> u64 {
    // determine dimensions
    // actually not needed anymore but can't be bothered to remove
    let mut row: i32 = 0;
    let mut col: i32 = 0;
    let mut min_row = i32::MAX;
    let mut min_col = i32::MAX;
    let mut max_row = i32::MIN;
    let mut max_col = i32::MIN;
    for (dir, num, _) in lines {
        match *dir {
            "R" => {
                col += *num as i32;
            }
            "L" => {
                col -= *num as i32;
            }
            "U" => {
                row -= *num as i32;
            }
            "D" => {
                row += *num as i32;
            }
            _ => {
                panic!("bad dir: {dir}")
            }
        }
        min_row = min_row.min(row);
        min_col = min_col.min(col);
        max_row = max_row.max(row);
        max_col = max_col.max(col);
    }
    // println!("{} {} {} {}", min_row, min_col, max_row, max_col);
    let mut num_rows = max_row - min_row + 1;
    let mut num_cols = max_col - min_col + 1;
    // let mut map = Vec::new();
    // for i in 0..num_rows {
    // map.push((0..num_cols).map(|_| '.').collect::<Vec<_>>());
    // }
    let mut coords = Vec::new();
    // print(&map);
    row = -min_row;
    col = -min_col;
    // map[row as usize][col as usize] = '#';
    coords.push((col, row));
    for (dir, num, _) in lines {
        match *dir {
            "R" => {
                for i in 0..*num {
                    col += 1;
                    coords.push((col, row));
                    // map[row as usize][col as usize] = '#';
                }
            }
            "L" => {
                for i in 0..*num {
                    col -= 1;
                    coords.push((col, row));
                    // map[row as usize][col as usize] = '#';
                }
            }
            "U" => {
                for i in 0..*num {
                    row -= 1;
                    coords.push((col, row));
                    // map[row as usize][col as usize] = '#';
                }
            }
            "D" => {
                for i in 0..*num {
                    row += 1;
                    coords.push((col, row));
                    // map[row as usize][col as usize] = '#';
                }
            }
            _ => {
                panic!("bad dir: {dir}")
            }
        }
    }
    // println!("{:?}", coords);
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
    println!("{}", part1(&lines));
}
