use std::fs;

use regex::Regex;

#[derive(Debug)]
struct Hailstone {
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
}

fn parse(lines: &Vec<&str>) -> Vec<Hailstone> {
    let mut res = Vec::new();
    let regex = Regex::new(r",\s*|\s*@\s*").unwrap();
    for line in lines {
        let spl = regex
            .split(line)
            .map(|x| x.parse::<f64>().unwrap())
            .collect::<Vec<_>>();
        res.push(Hailstone {
            x: spl[0],
            y: spl[1],
            z: spl[2],
            vx: spl[3],
            vy: spl[4],
            vz: spl[5],
        });
    }
    res
}

fn part1(hailstones: &Vec<Hailstone>, minpos: f64, maxpos: f64, vmult: f64) -> usize {
    let mut res = 0;
    for i in 0..hailstones.len() {
        for j in (i + 1)..hailstones.len() {
            let h1 = &hailstones[i];
            let h2 = &hailstones[j];
            // https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection#Given_two_points_on_each_line
            let x1 = h1.x;
            let y1 = h1.y;
            let x2 = h1.x + vmult*h1.vx;
            let y2 = h1.y + vmult*h1.vy;
            let x3 = h2.x;
            let y3 = h2.y;
            let x4 = h2.x + vmult*h2.vx;
            let y4 = h2.y + vmult*h2.vy;
            let denominator = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
            let px =
                ((x1 * y2 - y1 * x2) * (x3 - x4) - (x1 - x2) * (x3 * y4 - y3 * x4)) / denominator;
            let py =
                ((x1 * y2 - y1 * x2) * (y3 - y4) - (y1 - y2) * (x3 * y4 - y3 * x4)) / denominator;
            if (px - h1.x).signum() != h1.vx.signum() {
                continue;
            }
            if (px - h2.x).signum() != h2.vx.signum() {
                continue;
            }
            if (px >= minpos && px <= maxpos && py >= minpos && py <= maxpos) {
                res += 1;
            } 
        }
    }
    res
}

fn main() {
    let content = fs::read_to_string("inputs/day24.txt").unwrap();
    let lines = content.lines().collect::<Vec<_>>();
    let hailstones = parse(&lines);
    // println!("{}", part1(&hailstones, 7.0, 27.0));
    println!(
        "{}",
        part1(&hailstones, 200000000000000.0, 400000000000000.0, 1e9)
    );
}
