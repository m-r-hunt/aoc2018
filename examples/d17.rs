use regex::Regex;
use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq, Eq)]
enum Ground {
    Empty,
    Wall,
    SettledWater,
    FlowingWater,
}

struct Map {
    squares: HashMap<(i32, i32), Ground>,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

impl Map {
    fn _print(&self) {
        for y in self.min_y..=self.max_y {
            for x in self.min_x..=self.max_x {
                match self.squares.get(&(x, y)).unwrap_or(&Ground::Empty) {
                    Ground::Empty => print!(" "),
                    Ground::Wall => print!("#"),
                    Ground::SettledWater => print!("~"),
                    Ground::FlowingWater => print!("|"),
                }
            }
            println!("");
        }
    }
}

fn main() {
    let re = Regex::new(r"(x|y)=(\d*), [xy]=(\d*)..(\d*)").unwrap();
    let input = std::fs::read_to_string("input/d17input.txt").unwrap();
    let mut map = Map {
        squares: HashMap::new(),
        min_x: 9999999,
        max_x: -999999,
        min_y: 9999999,
        max_y: -999999,
    };
    for cap in re.captures_iter(&input) {
        let a = cap[2].parse::<i32>().unwrap();
        let b1 = cap[3].parse::<i32>().unwrap();
        let b2 = cap[4].parse::<i32>().unwrap();
        match &cap[1] {
            "x" => {
                for y in b1..=b2 {
                    map.squares.insert((a, y), Ground::Wall);
                }
                if a < map.min_x {
                    map.min_x = a;
                }
                if a > map.max_x {
                    map.max_x = a;
                }
                if b1 < map.min_y {
                    map.min_y = b1;
                }
                if b2 > map.max_y {
                    map.max_y = b2;
                }
            }
            "y" => {
                for x in b1..=b2 {
                    map.squares.insert((x, a), Ground::Wall);
                }
                if b1 < map.min_x {
                    map.min_x = b1;
                }
                if b2 > map.max_x {
                    map.max_x = b2;
                }
                if a < map.min_y {
                    map.min_y = a;
                }
                if a > map.max_y {
                    map.max_y = a;
                }
            }
            _ => panic!("Wut?"),
        }
    }
    //println!("{} {} {} {}", map.min_x, map.max_x, map.min_y, map.max_y);
    //map.print();
    downflow((500, 0), &mut map);
    //map.print();
    //map.squares.insert((500, 0), Ground::FlowingWater);
    let water_squares = map
        .squares
        .iter()
        .filter(|(k, v)| {
            k.1 >= map.min_y
                && k.1 <= map.max_y
                && (**v == Ground::SettledWater || **v == Ground::FlowingWater)
        })
        .count();
    println!("Part 1: {}", water_squares);
    let settled_water_squares = map
        .squares
        .iter()
        .filter(|(k, v)| k.1 >= map.min_y && k.1 <= map.max_y && **v == Ground::SettledWater)
        .count();
    println!("Part 2: {}", settled_water_squares);
}

fn downflow(src: (i32, i32), map: &mut Map) -> bool {
    let mut current = src;
    loop {
        let next = (current.0, current.1 + 1);
        let next_sq = *map.squares.get(&next).unwrap_or(&Ground::Empty);
        if next_sq == Ground::Wall || next_sq == Ground::SettledWater {
            break;
        } else if next.1 > map.max_y {
            return false;
        }
        current = next;
        if *map.squares.get(&next).unwrap_or(&Ground::Empty) == Ground::Empty {
            map.squares.insert(current, Ground::FlowingWater);
        }
    }

    loop {
        let mut left = current;
        let mut overflowing = false;
        loop {
            let next = (left.0 - 1, left.1);
            if *map.squares.get(&next).unwrap_or(&Ground::Empty) == Ground::Wall {
                break;
            }
            left = next;
            if *map.squares.get(&left).unwrap_or(&Ground::Empty) == Ground::Empty {
                map.squares.insert(left, Ground::FlowingWater);
            }
            let sq = *map
                .squares
                .get(&(left.0, left.1 + 1))
                .unwrap_or(&Ground::Empty);
            if sq == Ground::Empty {
                let filled = downflow(left, map);
                if !filled {
                    overflowing = true;
                    break;
                }
            } else if sq == Ground::FlowingWater {
                overflowing = true;
                break;
            }
        }
        let mut right = current;
        loop {
            let next = (right.0 + 1, right.1);
            if *map.squares.get(&next).unwrap_or(&Ground::Empty) == Ground::Wall {
                break;
            }
            right = next;
            if *map.squares.get(&right).unwrap_or(&Ground::Empty) == Ground::Empty {
                map.squares.insert(right, Ground::FlowingWater);
            }
            let sq = *map
                .squares
                .get(&(right.0, right.1 + 1))
                .unwrap_or(&Ground::Empty);
            if sq == Ground::Empty {
                let filled = downflow(right, map);
                if !filled {
                    overflowing = true;
                    break;
                }
            } else if sq == Ground::FlowingWater {
                overflowing = true;
                break;
            }
        }

        if !overflowing {
            for x in left.0..=right.0 {
                map.squares.insert((x, left.1), Ground::SettledWater);
            }
        } else {
            return false;
        }

        current = (current.0, current.1 - 1);
        if current == src {
            return true;
        }
    }
}
