#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Tile {
    Empty,
    Trees,
    Yard,
}

fn main() {
    let input = std::fs::read_to_string("input/d18input.txt").unwrap();
    let lines = input.split("\n");
    let mut grid = Vec::new();
    for (y, l) in lines.enumerate() {
        if l.len() == 0 {
            continue;
        }
        grid.push(Vec::new());
        for c in l.chars() {
            grid[y].push(match c {
                '.' => Tile::Empty,
                '|' => Tile::Trees,
                '#' => Tile::Yard,
                _ => panic!("Unknown char"),
            });
        }
    }

    let mut past_counts = std::collections::HashMap::new();

    let mut g = 0;
    let mut last_g = None;
    for n in 0..1001 {
        let mut next_grid = grid.clone();
        for y in 0..grid.len() {
            for x in 0..grid.len() {
                let (trees, yards) = count_neighbours(&grid, x as i32, y as i32);
                next_grid[y][x] = match grid[y][x] {
                    Tile::Empty => {
                        if trees >= 3 {
                            Tile::Trees
                        } else {
                            Tile::Empty
                        }
                    }
                    Tile::Trees => {
                        if yards >= 3 {
                            Tile::Yard
                        } else {
                            Tile::Trees
                        }
                    }
                    Tile::Yard => {
                        if trees >= 1 && yards >= 1 {
                            Tile::Yard
                        } else {
                            Tile::Empty
                        }
                    }
                }
            }
        }
        grid = next_grid;
        g += 1;

        let trees = grid.iter().fold(0, |acc, v| {
            acc + v.iter().filter(|a| **a == Tile::Trees).count()
        });
        let yards = grid.iter().fold(0, |acc, v| {
            acc + v.iter().filter(|a| **a == Tile::Yard).count()
        });
        if let Some(old_g) = past_counts.get(&(trees * yards)) {
            last_g = Some(*old_g)
        }
        past_counts.insert(trees * yards, g);
        if g == 10 {
            println!("Part 1: {}", trees * yards);
        }
    }

    let last_g = last_g.unwrap();
    let period = g - last_g;
    println!("{}", period);
    let target = 1000000000 % period;
    while g % period != target {}
    println!("Part 2: {}", past_counts.get(&g).unwrap());
}

fn count_neighbours(grid: &Vec<Vec<Tile>>, x: i32, y: i32) -> (i32, i32) {
    let get = |x: i32, y: i32| {
        if x >= 0 && (x as usize) < grid[0].len() && y >= 0 && (y as usize) < grid.len() {
            grid[y as usize][x as usize]
        } else {
            Tile::Empty
        }
    };

    let mut trees = 0;
    let mut yards = 0;
    for (x, y) in vec![
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ] {
        match get(x, y) {
            Tile::Trees => trees += 1,
            Tile::Yard => yards += 1,
            _ => (),
        }
    }
    (trees, yards)
}
