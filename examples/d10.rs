extern crate regex;
use regex::Regex;
use std::cmp::Ordering;

#[derive(PartialEq, Eq)]
struct Coord {
    x: i32,
    y: i32,
}

struct Light {
    position: Coord,
    velocity: Coord,
}

fn dump(lights: &mut Vec<Light>, min_x: i32, max_x: i32, min_y: i32, max_y: i32) {
    println!("{} {} {}", lights.len(), max_x, max_y);
    lights.sort_by(|a, b| {
        if a.position.y < b.position.y {
            Ordering::Less
        } else if a.position.y > b.position.y {
            Ordering::Greater
        } else {
            a.position.x.cmp(&b.position.x)
        }
    });
    let mut next_l = 0;
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let c = Coord { x, y };
            if next_l < lights.len() && lights[next_l].position == c {
                print!("#");
                next_l += 1;
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn main() {
    let re =
        Regex::new("position=< ?(-?[0-9]*),  ?(-?[0-9]*)> velocity=< ?(-?[0-9]*),  ?(-?[0-9]*)>")
            .unwrap();
    let input = std::fs::read_to_string("input/d10input.txt").unwrap();
    let input = input.split("\n");
    let mut max_x = 0;
    let mut max_y = 0;
    let mut min_x = 999999;
    let mut min_y = 999999;
    let mut lights = Vec::new();
    for line in input {
        println!("{}", line);
        match re.captures(&line) {
            Some(c) => {
                println!("{} {}, {} {}", &c[1], &c[2], &c[3], &c[4]);
                let x = c[1].parse().unwrap();
                let y = c[2].parse().unwrap();
                lights.push(Light {
                    position: Coord { x: x, y: y },
                    velocity: Coord {
                        x: c[3].parse().unwrap(),
                        y: c[4].parse().unwrap(),
                    },
                });
                if x > max_x {
                    max_x = x;
                }
                if x < min_x {
                    min_x = x;
                }
                if y > max_y {
                    max_y = y;
                }
                if y < min_y {
                    min_y = y;
                }
            }
            None => {
                continue;
            }
        }
    }
    'outer: loop {
        for i in 0..lights.len() {
            let l = &mut lights[i];
            l.position.x += l.velocity.x;
            l.position.y += l.velocity.y;
            if l.position.x < min_x
                || l.position.x > max_x
                || l.position.y < min_y
                || l.position.y > max_y
            {
                break 'outer;
            }
        }
        dump(&mut lights, min_x, max_x, min_y, max_y);
    }
}
