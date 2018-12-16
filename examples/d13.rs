#[derive(PartialEq, Eq, Debug)]
enum Turn {
    Left,
    Straight,
    Right,
}

fn incr_turn(t: &Turn) -> Turn {
    match t {
        Turn::Left => Turn::Straight,
        Turn::Straight => Turn::Right,
        Turn::Right => Turn::Left,
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

fn incr_direction(d: &Direction) -> Direction {
    match d {
        Direction::Right => Direction::Up,
        Direction::Up => Direction::Left,
        Direction::Left => Direction::Down,
        Direction::Down => Direction::Right,
    }
}
fn decr_direction(d: &Direction) -> Direction {
    match d {
        Direction::Right => Direction::Down,
        Direction::Up => Direction::Right,
        Direction::Left => Direction::Up,
        Direction::Down => Direction::Left,
    }
}

#[derive(Debug)]
struct Cart {
    x: usize,
    y: usize,
    heading: Direction,
    last_turn: Turn,
}

enum Track {
    None,
    Straight,
    LD,
    LU,
    Intersection,
}

fn main() {
    let input = std::fs::read_to_string("input/d13input.txt").unwrap();
    let input = input.split("\n");
    let mut tracks = Vec::new();
    let mut carts = Vec::new();
    for (y, line) in input.enumerate() {
        if line.len() == 0 {
            continue;
        }
        tracks.push(Vec::new());
        for (x, c) in line.chars().enumerate() {
            tracks[y].push(match c {
                '-' | '|' => Track::Straight,
                '\\' => Track::LD,
                '/' => Track::LU,
                '+' => Track::Intersection,
                ' ' => Track::None,
                '>' => {
                    carts.push(Cart {
                        x,
                        y,
                        heading: Direction::Right,
                        last_turn: Turn::Right,
                    });
                    Track::Straight
                }
                '<' => {
                    carts.push(Cart {
                        x,
                        y,
                        heading: Direction::Left,
                        last_turn: Turn::Right,
                    });
                    Track::Straight
                }
                '^' => {
                    carts.push(Cart {
                        x,
                        y,
                        heading: Direction::Up,
                        last_turn: Turn::Right,
                    });
                    Track::Straight
                }
                'v' => {
                    carts.push(Cart {
                        x,
                        y,
                        heading: Direction::Down,
                        last_turn: Turn::Right,
                    });
                    Track::Straight
                }
                _ => {
                    println!("{}", c);
                    panic!("Bad track")
                }
            });
        }
    }

    let mut dumped_first = false;
    loop {
        carts.sort_by(|a, b| {
            if a.y.cmp(&b.y) == std::cmp::Ordering::Equal {
                a.x.cmp(&b.x)
            } else {
                a.y.cmp(&b.y)
            }
        });

        let mut cc: isize = 0;
        while cc < carts.len() as isize {
            {
                let c = &mut carts[cc as usize];
                //println!("{:?}", c);
                match c.heading {
                    Direction::Left => c.x -= 1,
                    Direction::Right => c.x += 1,
                    Direction::Up => c.y -= 1,
                    Direction::Down => c.y += 1,
                }
                match tracks[c.y][c.x] {
                    Track::None => panic!("Bad move."),
                    Track::Straight => (),
                    Track::LU => {
                        if c.heading == Direction::Left {
                            c.heading = Direction::Down;
                        } else if c.heading == Direction::Down {
                            c.heading = Direction::Left;
                        } else if c.heading == Direction::Up {
                            c.heading = Direction::Right;
                        } else if c.heading == Direction::Right {
                            c.heading = Direction::Up;
                        }
                    }
                    Track::LD => {
                        if c.heading == Direction::Left {
                            c.heading = Direction::Up;
                        } else if c.heading == Direction::Up {
                            c.heading = Direction::Left;
                        } else if c.heading == Direction::Down {
                            c.heading = Direction::Right;
                        } else if c.heading == Direction::Right {
                            c.heading = Direction::Down;
                        }
                    }
                    Track::Intersection => {
                        c.last_turn = incr_turn(&c.last_turn);
                        match c.last_turn {
                            Turn::Left => {
                                c.heading = incr_direction(&c.heading);
                            }
                            Turn::Right => {
                                c.heading = decr_direction(&c.heading);
                            }
                            Turn::Straight => (),
                        }
                    }
                }
            }
            
            for c in 0..carts.len() {
                if (c as isize) != cc  && carts[c].x == carts[cc as usize].x && carts[c as usize].y == carts[cc as usize].y {
                    if !dumped_first {
                        println!("Part 1: {},{}", carts[c as usize].x, carts[c as usize].y);
                        dumped_first = true;
                    }
                    carts.remove(c as usize);
                    if (c as isize) <= cc {
                        cc-=1;
                    }
                    carts.remove(cc as usize);
                    cc-=1;
                    break;
                }
            }
            
            cc += 1;
        }
            if carts.len() == 1 {
                println!("Part 2: {},{}", carts[0].x, carts[0].y);
                break;
            }
    }
}
