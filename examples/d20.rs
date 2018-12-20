#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Direction {
    E,
    N,
    W,
    S,
}

#[derive(Clone, Eq, PartialEq, Debug)]
enum Node {
    Literal(Direction),
    Subtree(Vec<Tree>),
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Tree {
    nodes: Vec<Node>,
}

fn parse(i: &mut usize, chars: &Vec<char>) -> Vec<Tree> {
    let mut out = Vec::new();
    let mut current = Tree { nodes: Vec::new() };
    while *i < chars.len() {
        let c = chars[*i];
        *i += 1;
        match c {
            'E' => current.nodes.push(Node::Literal(Direction::E)),
            'N' => current.nodes.push(Node::Literal(Direction::N)),
            'W' => current.nodes.push(Node::Literal(Direction::W)),
            'S' => current.nodes.push(Node::Literal(Direction::S)),
            '|' => {
                out.push(current);
                current = Tree { nodes: Vec::new() };
            }
            '(' => current.nodes.push(Node::Subtree(parse(i, chars))),
            ')' | '$' => {
                break;
            }
            _ => panic!("Bad char in regex"),
        }
    }
    out.push(current);
    return out;
}

fn explore(map: &mut Map, tree: &Tree, mut pos: (i32, i32)) {
    for node in tree.nodes.iter() {
        match node {
            Node::Literal(d) => {
                match d {
                    Direction::E => {
                        map.squares.entry(pos).or_insert(Room::new()).e = true;
                        pos = (pos.0 + 1, pos.1);
                        map.squares.entry(pos).or_insert(Room::new()).w = true;
                    }
                    Direction::N => {
                        map.squares.entry(pos).or_insert(Room::new()).n = true;
                        pos = (pos.0, pos.1 - 1);
                        map.squares.entry(pos).or_insert(Room::new()).s = true;
                    }
                    Direction::W => {
                        map.squares.entry(pos).or_insert(Room::new()).w = true;
                        pos = (pos.0 - 1, pos.1);
                        map.squares.entry(pos).or_insert(Room::new()).e = true;
                    }
                    Direction::S => {
                        map.squares.entry(pos).or_insert(Room::new()).s = true;
                        pos = (pos.0, pos.1 + 1);
                        map.squares.entry(pos).or_insert(Room::new()).n = true;
                    }
                }
                if pos.0 < map.x_min {
                    map.x_min = pos.0;
                }
                if pos.0 > map.x_max {
                    map.x_max = pos.0;
                }
                if pos.1 < map.y_min {
                    map.y_min = pos.1;
                }
                if pos.1 > map.y_max {
                    map.y_max = pos.1;
                }
            }
            Node::Subtree(subtrees) => {
                for s in subtrees.iter() {
                    explore(map, s, pos)
                }
            }
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Room {
    e: bool,
    n: bool,
    w: bool,
    s: bool,
}

impl Room {
    fn new() -> Room {
        Room {
            e: false,
            n: false,
            w: false,
            s: false,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Map {
    squares: std::collections::HashMap<(i32, i32), Room>,
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

fn main() {
    let input = std::fs::read_to_string("input/d20input.txt").unwrap();
    let trees = parse(&mut 1, &input.chars().collect());
    let mut map = Map {
        squares: std::collections::HashMap::new(),
        x_min: 0,
        x_max: 0,
        y_min: 0,
        y_max: 0,
    };
    for t in trees {
        explore(&mut map, &t, (0, 0));
    }

    if cfg!(feature = "dumpMap") {
        for y in map.y_min..=map.y_max {
            print!("#");
            for x in map.x_min..=map.x_max {
                if map.squares.get(&(x, y)).unwrap_or(&Room::new()).n {
                    print!("-");
                } else {
                    print!("#");
                }
                print!("#");
            }
            println!("");

            print!("#");
            for x in map.x_min..=map.x_max {
                if x == 0 && y == 0 {
                    print!("X");
                } else {
                    print!(".");
                }
                if map.squares.get(&(x, y)).unwrap_or(&Room::new()).e {
                    print!("|");
                } else {
                    print!("#");
                }
            }
            println!("");
        }
    }

    let mut queue = std::collections::VecDeque::new();
    queue.push_back(((0, 0), 0));
    let mut visited = std::collections::HashMap::new();
    visited.insert((0, 0), true);
    let mut max_dist = 0;
    let mut far_count = 0;
    while queue.len() > 0 {
        let (c, dist) = queue.pop_front().unwrap();
        if dist > max_dist {
            max_dist = dist;
        }
        if dist >= 1000 {
            far_count += 1;
        }

        {
            let cc = (c.0 - 1, c.1);
            if map.squares.get(&c).unwrap().w && !visited.get(&cc).unwrap_or(&false) {
                visited.insert(cc, true);
                queue.push_back((cc, dist + 1));
            }
        }

        {
            let cc = (c.0 + 1, c.1);
            if map.squares.get(&c).unwrap().e && !visited.get(&cc).unwrap_or(&false) {
                visited.insert(cc, true);
                queue.push_back((cc, dist + 1));
            }
        }

        {
            let cc = (c.0, c.1 - 1);
            if map.squares.get(&c).unwrap().n && !visited.get(&cc).unwrap_or(&false) {
                visited.insert(cc, true);
                queue.push_back((cc, dist + 1));
            }
        }

        {
            let cc = (c.0, c.1 + 1);
            if map.squares.get(&c).unwrap().s && !visited.get(&cc).unwrap_or(&false) {
                visited.insert(cc, true);
                queue.push_back((cc, dist + 1));
            }
        }
    }
    println!("Part 1: {}", max_dist);
    println!("Part 2: {}", far_count);
}
