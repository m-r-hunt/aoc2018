#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Team {
    None,
    Elf,
    Goblin,
}

#[derive(Debug, Clone)]
struct Actor {
    x: i32,
    y: i32,
    hp: i32,
    team: Team,
    dead: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Ground {
    Empty,
    Wall,
}

#[derive(Debug, Copy, Clone)]
struct Tile {
    ground: Ground,
    contains: Team,
}

fn main() {
    let input = std::fs::read_to_string("input/d15input.txt").unwrap();
    let lines = input.split("\n");
    let mut grid = Vec::new();
    let mut actors = Vec::new();
    for (y, l) in lines.enumerate() {
        grid.push(Vec::new());
        for (x, c) in l.chars().enumerate() {
            match c {
                '#' => grid[y].push(Tile {
                    ground: Ground::Wall,
                    contains: Team::None,
                }),
                '.' => grid[y].push(Tile {
                    ground: Ground::Empty,
                    contains: Team::None,
                }),
                'E' => {
                    actors.push(Actor {
                        x: x as i32,
                        y: y as i32,
                        hp: 200,
                        team: Team::Elf,
                        dead: false,
                    });
                    grid[y].push(Tile {
                        ground: Ground::Empty,
                        contains: Team::Elf,
                    });
                }
                'G' => {
                    actors.push(Actor {
                        x: x as i32,
                        y: y as i32,
                        hp: 200,
                        team: Team::Goblin,
                        dead: false,
                    });
                    grid[y].push(Tile {
                        ground: Ground::Empty,
                        contains: Team::Goblin,
                    });
                }
                '\r' => (),
                _ => panic!(format!("Unknown tile {}", c)),
            }
        }
    }

    let (outcome, _) = run_simulation(actors.clone(), grid.clone(), 3);
    println!("Part 1: {}", outcome);

    let initial_elf_count = actors.iter().filter(|a| a.team == Team::Elf).count();
    for elf_power in 4..std::i32::MAX {
        let (outcome, elves_alive) = run_simulation(actors.clone(), grid.clone(), elf_power);
        if elves_alive == initial_elf_count {
            println!("Part 2: {} (final elf power: {})", outcome, elf_power);
            break;
        }
    }
}

fn run_simulation(
    mut actors: Vec<Actor>,
    mut grid: Vec<Vec<Tile>>,
    elf_power: i32,
) -> (i32, usize) {
    let mut round = 0;
    'outer: loop {
        actors.retain(|a| !a.dead);
        actors.sort_by(|a, b| {
            if a.y.cmp(&b.y) == std::cmp::Ordering::Equal {
                a.x.cmp(&b.x)
            } else {
                a.y.cmp(&b.y)
            }
        });

        for a in 0..actors.len() {
            if actors[a].dead {
                continue;
            }

            let next_move = find_move(&actors[a], &grid);
            match next_move {
                Some((x, y)) => {
                    let a = &mut actors[a];
                    grid[a.y as usize][a.x as usize].contains = Team::None;
                    a.x = x;
                    a.y = y;
                    grid[a.y as usize][a.x as usize].contains = a.team;
                }
                None => (),
            }

            let team = actors[a].team;
            if !actors.iter().any(|a| !a.dead && a.team != team) {
                break 'outer;
            }

            let target = find_target(&actors, a, &grid);
            match target {
                Some(target) => {
                    let target = &mut actors[target];
                    if target.team == Team::Goblin {
                        target.hp -= elf_power;
                    } else {
                        target.hp -= 3;
                    }
                    if target.hp <= 0 {
                        target.dead = true;
                        grid[target.y as usize][target.x as usize].contains = Team::None;
                        if cfg!(feature = "dumpCombat") {
                            println!("Guy died");
                        }
                    }
                }
                None => (),
            }
        }
        round += 1;

        if cfg!(feature = "dumpCombat") {
            println!("{}", round);
            for line in grid.iter() {
                for t in line.iter() {
                    print!(
                        "{}",
                        match t.contains {
                            Team::Elf => 'E',
                            Team::Goblin => 'G',
                            Team::None => match t.ground {
                                Ground::Empty => '.',
                                Ground::Wall => '#',
                            },
                        }
                    );
                }
                println!("");
            }
            println!("{:?}", actors);
        }
    }
    actors.retain(|a| !a.dead);
    if cfg!(feature = "dumpCombat") {
        println!("Ended");
        println!("Actors: {:?}", actors);
    }

    let elves_alive = actors.iter().filter(|a| a.team == Team::Elf).count();
    let tot_hp = actors.iter().fold(0, |acc, a| acc + a.hp);
    return (tot_hp * round, elves_alive);
}

fn find_move(a: &Actor, grid: &Vec<Vec<Tile>>) -> Option<(i32, i32)> {
    let enemy_adjacent = |x: usize, y: usize, t: Team| {
        (y >= 1 && grid[y - 1][x].contains != Team::None && grid[y - 1][x].contains != t)
            || (y + 1 < grid.len()
                && grid[y + 1][x].contains != Team::None
                && grid[y + 1][x].contains != t)
            || (x >= 1 && grid[y][x - 1].contains != Team::None && grid[y][x - 1].contains != t)
            || (x + 1 < grid.len()
                && grid[y][x + 1].contains != Team::None
                && grid[y][x + 1].contains != t)
    };
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    visited[a.y as usize][a.x as usize] = true;
    let mut queue = std::collections::VecDeque::new();
    queue.push_back((a.x, a.y, None));
    while queue.len() > 0 {
        let (x, y, mv) = queue.pop_front().unwrap();
        if enemy_adjacent(x as usize, y as usize, a.team) {
            return mv;
        }
        for (x, y) in vec![(x, y - 1), (x - 1, y), (x + 1, y), (x, y + 1)] {
            if x >= 0
                && (x as usize) < grid[0].len()
                && y >= 0
                && (y as usize) < grid.len()
                && !visited[y as usize][x as usize]
                && grid[y as usize][x as usize].contains == Team::None
                && grid[y as usize][x as usize].ground == Ground::Empty
            {
                visited[y as usize][x as usize] = true;
                queue.push_back((x, y, mv.or(Some((x, y)))));
            }
        }
    }
    return None;
}

fn find_target(actors: &Vec<Actor>, a: usize, grid: &Vec<Vec<Tile>>) -> Option<usize> {
    let x = actors[a].x;
    let y = actors[a].y;
    let team = actors[a].team;
    let mut out = None;
    let mut min_hp = 999;

    for (x, y) in vec![(x, y - 1), (x - 1, y), (x + 1, y), (x, y + 1)] {
        if x >= 0
            && (x as usize) < grid[0].len()
            && y >= 0
            && (y as usize) < grid.len()
            && grid[(y) as usize][x as usize].contains != Team::None
            && grid[(y) as usize][x as usize].contains != team
        {
            let (i, a) = actors
                .iter()
                .enumerate()
                .find(|(_, a)| !a.dead && a.x == x && a.y == y)
                .unwrap();
            if a.hp < min_hp {
                min_hp = a.hp;
                out = Some(i);
            }
        }
    }

    return out;
}
