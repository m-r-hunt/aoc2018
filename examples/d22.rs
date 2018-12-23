use std::collections::{BinaryHeap, HashMap};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Tool {
    Torch,
    Gear,
    None,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct State {
    x: i32,
    y: i32,
    tool: Tool,
    time: i32,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> std::cmp::Ordering {
        other.time.cmp(&self.time)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let target = (15, 700);
    let depth = 4848;
    //let target = (10, 10);
    //let depth = 510;
    let mut erosion_levels = HashMap::new();
    erosion_levels.insert((0, 0), (0 + depth) % 20183);
    for x in 1..=target.0 {
        erosion_levels.insert((x, 0), (x * 16807 + depth) % 20183);
    }
    for y in 1..=target.1 {
        erosion_levels.insert((0, y), (y * 48271 + depth) % 20183);
    }
    for x in 1..=target.0 {
        for y in 1..=target.1 {
            let gi = if (x, y) == target {
                0
            } else {
                erosion_levels.get(&(x - 1, y)).unwrap() * erosion_levels.get(&(x, y - 1)).unwrap()
            };
            erosion_levels.insert((x, y), (gi + depth) % 20183);
        }
    }

    for y in 0..=target.1 {
        for x in 0..=target.0 {
            print!(
                "{}",
                match erosion_levels.get(&(x, y)).unwrap() % 3 {
                    0 => ".",
                    1 => "=",
                    2 => "|",
                    _ => panic!("Unexpected thing mod 3"),
                }
            );
        }
        println!("");
    }

    let risk = erosion_levels.iter().fold(0, |acc, (_, el)| acc + (el % 3));
    println!("Part 1: {}", risk);

    let mut erosion_levels = HashMap::new();
    erosion_levels.insert((0, 0), (0 + depth) % 20183);
    for x in 1..=5 * target.0 {
        erosion_levels.insert((x, 0), (x * 16807 + depth) % 20183);
    }
    for y in 1..=depth {
        erosion_levels.insert((0, y), (y * 48271 + depth) % 20183);
    }
    for x in 1..=5 * target.0 {
        for y in 1..=depth {
            let gi = if (x, y) == target {
                0
            } else {
                erosion_levels.get(&(x - 1, y)).unwrap() * erosion_levels.get(&(x, y - 1)).unwrap()
            };
            erosion_levels.insert((x, y), (gi + depth) % 20183);
        }
    }

    let valid_tool = |x, y, tool| match erosion_levels.get(&(x, y)).unwrap() % 3 {
        0 => tool == Tool::Torch || tool == Tool::Gear,
        1 => tool == Tool::None || tool == Tool::Gear,
        2 => tool == Tool::Torch || tool == Tool::None,
        _ => panic!("Unexpected thing mod 3"),
    };

    let mut queue = BinaryHeap::new();
    let mut visited = HashMap::new();
    queue.push(State {
        x: 0,
        y: 0,
        tool: Tool::Torch,
        time: 0,
    });
    //visited.insert((0, 0, Tool::Torch), true);
    while queue.len() > 0 {
        let State { x, y, tool, time } = queue.pop().unwrap();
        if (x, y) == target && tool == Tool::Torch {
            println!("Part 2: {}", time);
            break;
        }

        match visited.get(&(x, y, tool)) {
            Some(_) => {
                continue;
            }
            None => {
                visited.insert((x, y, tool), true);
            }
        }

        if tool != Tool::Torch
            && valid_tool(x, y, Tool::Torch)
            && !visited.get(&(x, y, Tool::Torch)).unwrap_or(&false)
        {
            queue.push(State {
                x,
                y,
                tool: Tool::Torch,
                time: time + 7,
            });
        }

        if tool != Tool::Gear
            && valid_tool(x, y, Tool::Gear)
            && !visited.get(&(x, y, Tool::Gear)).unwrap_or(&false)
        {
            queue.push(State {
                x,
                y,
                tool: Tool::Gear,
                time: time + 7,
            });
        }

        if tool != Tool::None
            && valid_tool(x, y, Tool::None)
            && !visited.get(&(x, y, Tool::None)).unwrap_or(&false)
        {
            queue.push(State {
                x,
                y,
                tool: Tool::None,
                time: time + 7,
            });
        }

        {
            let c = (x - 1, y);
            if c.0 >= 0
                && c.0 <= target.0 * 5
                && c.1 >= 0
                && c.1 <= depth
                && valid_tool(c.0, c.1, tool)
                && !visited.get(&(c.0, c.1, tool)).unwrap_or(&false)
            {
                queue.push(State {
                    x: c.0,
                    y: c.1,
                    tool,
                    time: time + 1,
                });
            }
        }

        {
            let c = (x + 1, y);
            if c.0 >= 0
                && c.0 <= target.0 * 5
                && c.1 >= 0
                && c.1 <= depth
                && valid_tool(c.0, c.1, tool)
                && !visited.get(&(c.0, c.1, tool)).unwrap_or(&false)
            {
                queue.push(State {
                    x: c.0,
                    y: c.1,
                    tool,
                    time: time + 1,
                });
            }
        }

        {
            let c = (x, y - 1);
            if c.0 >= 0
                && c.0 <= target.0 * 5
                && c.1 >= 0
                && c.1 <= depth
                && valid_tool(c.0, c.1, tool)
                && !visited.get(&(c.0, c.1, tool)).unwrap_or(&false)
            {
                queue.push(State {
                    x: c.0,
                    y: c.1,
                    tool,
                    time: time + 1,
                });
            }
        }

        {
            let c = (x, y + 1);
            if c.0 >= 0
                && c.0 <= target.0 * 5
                && c.1 >= 0
                && c.1 <= depth
                && valid_tool(c.0, c.1, tool)
                && !visited.get(&(c.0, c.1, tool)).unwrap_or(&false)
            {
                queue.push(State {
                    x: c.0,
                    y: c.1,
                    tool,
                    time: time + 1,
                });
            }
        }
    }
}
