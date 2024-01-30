#![feature(btree_cursors)]

use std::{
    collections::{BTreeMap, HashMap},
    fs,
    ops::Bound,
    thread::current,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Brick {
    id: u64,
    x0: i32,
    y0: i32,
    z0: i32,
    x1: i32,
    y1: i32,
    z1: i32,
}

fn parse(lines: &Vec<&str>) -> Vec<Brick> {
    let mut res = Vec::new();
    let mut id = 0;
    for line in lines {
        let spl = line
            .split("~")
            .map(|y| {
                y.split(",")
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        res.push(Brick {
            id: id,
            x0: spl[0][0],
            y0: spl[0][1],
            z0: spl[0][2],
            x1: spl[1][0],
            y1: spl[1][1],
            z1: spl[1][2],
        });
        id += 1;
    }
    res
}

fn part1(bricks: &Vec<Brick>) -> u64 {
    let mut bricks_by_z: BTreeMap<i32, Vec<Brick>> = BTreeMap::new();
    for brick in bricks {
        for z in brick.z0..=brick.z1 {
            bricks_by_z.entry(z).or_insert(Vec::new()).push(*brick);
        }
    }
    let mut bricks_sorted_by_z0 = bricks.clone();
    bricks_sorted_by_z0.sort_by_key(|x| x.z0);
    let mut moved_bricks = Vec::new();
    for i in 0..bricks_sorted_by_z0.len() {
        let brick = bricks_sorted_by_z0[i];
        // println!("---> Trying brick {:?}", brick);
        if brick.z0 == 1 {
            // println!("z0 is 1, can't move further down");
            moved_bricks.push(brick);
            continue;
        }
        let mut best_z = brick.z0;
        loop {
            let cursor = bricks_by_z.upper_bound(Bound::Excluded(&best_z));
            if let Some((z, bricks_at_z)) = cursor.peek_prev() {
                // println!("{:?}", z);
                if is_intersection(&bricks_at_z, &brick) {
                    best_z = (z + 1).min(best_z);
                    // println!("setting best_z to {best_z}");
                    break;
                } else {
                    best_z = *z;
                    // println!("setting best_z to {z}");
                }
            } else {
                // nothing below so can just move to 1
                best_z = 1;
                break;
            }
        }
        // println!("Can move from z0={} to z={}", brick.z0, best_z);
        if (brick.z0 != best_z) {
            // need to update the btreemap
            // delete first
            for z in brick.z0..=brick.z1 {
                bricks_by_z.get_mut(&z).unwrap().retain(|x| x != &brick);
            }
            // add
            let updated_brick = Brick {
                id: brick.id,
                x0: brick.x0,
                y0: brick.y0,
                z0: best_z,
                x1: brick.x1,
                y1: brick.y1,
                z1: brick.z1 - brick.z0 + best_z,
            };
            for z in updated_brick.z0..=updated_brick.z1 {
                bricks_by_z
                    .entry(z)
                    .or_insert(Vec::new())
                    .push(updated_brick);
            }
            // println!("new bricks_by_z={:?}", bricks_by_z);
            moved_bricks.push(updated_brick);
        } else {
            moved_bricks.push(brick);
        }
    }
    for mb in &moved_bricks {
        println!("*** {:?}", mb);
    }
    // build the graph
    let mut supported_by: HashMap<u64, Vec<u64>> = HashMap::new();
    let mut supports: HashMap<u64, Vec<u64>> = HashMap::new();
    let mut bricks_by_id: HashMap<u64, Brick> = HashMap::new();
    let mut bricks_by_z0: BTreeMap<i32, Vec<Brick>> = BTreeMap::new();
    let mut bricks_by_z1: HashMap<i32, Vec<Brick>> = HashMap::new();
    for b in moved_bricks {
        bricks_by_id.insert(b.id, b);
        bricks_by_z0.entry(b.z0).or_insert(Vec::new()).push(b);
        bricks_by_z1.entry(b.z1).or_insert(Vec::new()).push(b);
    }
    for (z0, bs) in bricks_by_z0.iter() {
        if *z0 > 1 {
            let bricks_below_opt = bricks_by_z1.get(&(z0 - 1));
            if let Some(bricks_below) = bricks_below_opt {
                for b in bs {
                    for bb in bricks_below {
                        if is_brick_intersection(&b, &bb) {
                            // b is supported by bb
                            supported_by.entry(b.id).or_insert(Vec::new()).push(bb.id);
                            supports.entry(bb.id).or_insert(Vec::new()).push(b.id);
                        }
                    }
                }
            }
        }
    }
    // println!("supports: {:?}", supports);
    // println!("supported_by: {:?}", supported_by);
    let mut res = 0;
    for id in bricks_by_id.keys() {
        let id_supports_opt = supports.get(id);
        if let Some(id_supports) = id_supports_opt {
            let mut disintegrate = true;
            for id2 in id_supports {
                let id_supported_by = supported_by.get(id2).unwrap();
                if id_supported_by.len() == 1 {
                    disintegrate = false;
                    break;
                }
            }
            if disintegrate {
                res += 1;
            }
        } else {
            // doesn't support anything, can be disintegrated
            res += 1;
        }
    }
    res
}

fn is_brick_intersection(b1: &Brick, b2: &Brick) -> bool {
    return ((b1.x0 >= b2.x0 && b1.x0 <= b2.x1)
        || (b1.x1 >= b2.x0 && b1.x1 <= b2.x1)
        || (b2.x0 >= b1.x0 && b2.x0 <= b1.x1)
        || (b2.x1 >= b1.x0 && b2.x1 <= b1.x1))
        && ((b1.y0 >= b2.y0 && b1.y0 <= b2.y1)
            || (b1.y1 >= b2.y0 && b1.y1 <= b2.y1)
            || (b2.y0 >= b1.y0 && b2.y0 <= b1.y1)
            || (b2.y1 >= b1.y0 && b2.y1 <= b1.y1));
}

fn is_intersection(bricks_at_z: &Vec<Brick>, brick: &Brick) -> bool {
    for brick_at_z in bricks_at_z {
        if is_brick_intersection(brick, brick_at_z) {
            return true;
        }
    }
    false
}

fn main() {
    let content = fs::read_to_string("inputs/day22.txt").unwrap();
    let lines = content.lines().collect::<Vec<_>>();
    let bricks = parse(&lines);

    // part 1
    println!("{:?}", part1(&bricks));
}
