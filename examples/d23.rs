#[macro_use]
extern crate scan_rules;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Bot {
    x: i64,
    y: i64,
    z: i64,
    r: i64,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Box {
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
    min_z: i64,
    max_z: i64,
}

fn main() {
    let input = std::fs::read_to_string("input/d23input.txt").unwrap();
    let lines = input.split("\n");
    let mut bots = Vec::new();
    for l in lines {
        if l.len() == 0 {
            continue;
        }
        let_scan!(l; ("pos=<", let x: i64, ",", let y: i64, ",", let z: i64, ">, r=", let r: i64));
        bots.push(Bot { x, y, z, r });
    }

    let mut max_r = 0;
    let mut max_pos = (0, 0, 0);
    for b in bots.iter() {
        if b.r > max_r {
            max_r = b.r;
            max_pos = (b.x, b.y, b.z);
        }
    }

    let mut count = 0;
    for b in bots.iter() {
        if (max_pos.0 - b.x).abs() + (max_pos.1 - b.y).abs() + (max_pos.2 - b.z).abs() <= max_r {
            count += 1;
        }
    }
    println!("Part 1: {}", count);
    // Need to redo Part 2 properly.
}
