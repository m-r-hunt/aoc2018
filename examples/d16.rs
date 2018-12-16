extern crate regex;
use regex::Regex;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Sample {
    before_regs: [i32; 4],
    op: i32,
    a: i32,
    b: i32,
    c: i32,
    after_regs: [i32; 4],
}

fn main() {
    let input = std::fs::read_to_string("input/d16input.txt").unwrap();
    let re = Regex::new(r"Before: \[(\d*), (\d*), (\d*), (\d*)\]\r\n(\d*) (\d*) (\d*) (\d*)\r\nAfter:  \[(\d*), (\d*), (\d*), (\d*)\]").unwrap();
    let mut samples = Vec::new();
    for cap in re.captures_iter(&input) {
        samples.push(Sample {
            before_regs: [
                cap[1].parse().unwrap(),
                cap[2].parse().unwrap(),
                cap[3].parse().unwrap(),
                cap[4].parse().unwrap(),
            ],
            op: cap[5].parse().unwrap(),
            a: cap[6].parse().unwrap(),
            b: cap[7].parse().unwrap(),
            c: cap[8].parse().unwrap(),
            after_regs: [
                cap[9].parse().unwrap(),
                cap[10].parse().unwrap(),
                cap[11].parse().unwrap(),
                cap[12].parse().unwrap(),
            ],
        });
    }

    let op_fns: Vec<fn([i32; 4], i32, i32, i32) -> [i32; 4]> = vec![
        addr, addi, mulr, muli, banr, bani, borr, bori, setr, seti, gtir, gtri, gtrr, eqir, eqri,
        eqrr,
    ];

    let mut possible_ops = vec![vec![true; 16]; 16];
    let mut three_count = 0;
    for s in samples {
        let mut looks_like = 0;
        for (i, f) in op_fns.iter().enumerate() {
            if f(s.before_regs, s.a, s.b, s.c) == s.after_regs {
                looks_like += 1;
            } else {
                possible_ops[s.op as usize][i] = false;
            }
        }
        if looks_like >= 3 {
            three_count += 1;
        }
    }
    println!("Part 1: {}", three_count);

    loop {
        for i in 0..16 {
            if possible_ops[i].iter().filter(|a| **a).count() == 1 {
                let (s, _) = possible_ops[i]
                    .iter()
                    .enumerate()
                    .find(|(_, b)| **b)
                    .unwrap();
                for ii in 0..16 {
                    if i == ii {
                        continue;
                    }
                    possible_ops[ii][s] = false;
                }
            }
        }
        if possible_ops
            .iter()
            .filter(|a| a.iter().filter(|a| **a).count() > 1)
            .count()
            == 0
        {
            break;
        }
    }
    let mut opcode_map: Vec<fn([i32; 4], i32, i32, i32) -> [i32; 4]> = Vec::new();
    for op in 0..16 {
        for (i, f) in op_fns.iter().enumerate() {
            if possible_ops[op][i] {
                opcode_map.push(*f);
                break;
            }
        }
    }

    let input: Vec<_> = input.split("\r\n").collect();
    let input = &input[3118..];
    let mut regs: [i32; 4] = [0; 4];
    let re = Regex::new(r"(\d*) (\d*) (\d*) (\d*)").unwrap();
    for line in input.iter() {
        if line.len() == 0 {
            continue;
        }
        let cap = re.captures(line).unwrap();
        regs = opcode_map[cap[1].parse::<usize>().unwrap()](
            regs,
            cap[2].parse::<i32>().unwrap(),
            cap[3].parse::<i32>().unwrap(),
            cap[4].parse::<i32>().unwrap(),
        );
    }
    println!("Part 2: {}", regs[0]);
}

fn addr(mut regs: [i32; 4], a: i32, b: i32, c: i32) -> [i32; 4] {
    regs[c as usize] = regs[a as usize] + regs[b as usize];
    regs
}

fn addi(mut regs: [i32; 4], a: i32, b: i32, c: i32) -> [i32; 4] {
    regs[c as usize] = regs[a as usize] + b;
    regs
}

fn mulr(mut regs: [i32; 4], a: i32, b: i32, c: i32) -> [i32; 4] {
    regs[c as usize] = regs[a as usize] * regs[b as usize];
    regs
}

fn muli(mut regs: [i32; 4], a: i32, b: i32, c: i32) -> [i32; 4] {
    regs[c as usize] = regs[a as usize] * b;
    regs
}

fn banr(mut regs: [i32; 4], a: i32, b: i32, c: i32) -> [i32; 4] {
    regs[c as usize] = regs[a as usize] & regs[b as usize];
    regs
}

fn bani(mut regs: [i32; 4], a: i32, b: i32, c: i32) -> [i32; 4] {
    regs[c as usize] = regs[a as usize] & b;
    regs
}

fn borr(mut regs: [i32; 4], a: i32, b: i32, c: i32) -> [i32; 4] {
    regs[c as usize] = regs[a as usize] | regs[b as usize];
    regs
}

fn bori(mut regs: [i32; 4], a: i32, b: i32, c: i32) -> [i32; 4] {
    regs[c as usize] = regs[a as usize] | b;
    regs
}

fn setr(mut regs: [i32; 4], a: i32, _b: i32, c: i32) -> [i32; 4] {
    regs[c as usize] = regs[a as usize];
    regs
}

fn seti(mut regs: [i32; 4], a: i32, _b: i32, c: i32) -> [i32; 4] {
    regs[c as usize] = a;
    regs
}

fn gtir(mut regs: [i32; 4], a: i32, b: i32, c: i32) -> [i32; 4] {
    regs[c as usize] = if a > regs[b as usize] { 1 } else { 0 };
    regs
}

fn gtri(mut regs: [i32; 4], a: i32, b: i32, c: i32) -> [i32; 4] {
    regs[c as usize] = if regs[a as usize] > b { 1 } else { 0 };
    regs
}

fn gtrr(mut regs: [i32; 4], a: i32, b: i32, c: i32) -> [i32; 4] {
    regs[c as usize] = if regs[a as usize] > regs[b as usize] {
        1
    } else {
        0
    };
    regs
}

fn eqir(mut regs: [i32; 4], a: i32, b: i32, c: i32) -> [i32; 4] {
    regs[c as usize] = if a == regs[b as usize] { 1 } else { 0 };
    regs
}

fn eqri(mut regs: [i32; 4], a: i32, b: i32, c: i32) -> [i32; 4] {
    regs[c as usize] = if regs[a as usize] == b { 1 } else { 0 };
    regs
}

fn eqrr(mut regs: [i32; 4], a: i32, b: i32, c: i32) -> [i32; 4] {
    regs[c as usize] = if regs[a as usize] == regs[b as usize] {
        1
    } else {
        0
    };
    regs
}
