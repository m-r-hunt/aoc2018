extern crate regex;
use regex::Regex;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Instr {
    op: fn([i32; 6], i32, i32, i32) -> [i32; 6],
    a: i32,
    b: i32,
    c: i32,
}

fn main() {
    let input = std::fs::read_to_string("input/d19input.txt").unwrap();

    let mut ops: std::collections::HashMap<String, fn([i32; 6], i32, i32, i32) -> [i32; 6]> =
        std::collections::HashMap::new();
    ops.insert("banr".to_string(), banr);
    ops.insert("addr".to_string(), addr);
    ops.insert("eqri".to_string(), eqri);
    ops.insert("setr".to_string(), setr);
    ops.insert("gtrr".to_string(), gtrr);
    ops.insert("bori".to_string(), bori);
    ops.insert("gtir".to_string(), gtir);
    ops.insert("seti".to_string(), seti);
    ops.insert("borr".to_string(), borr);
    ops.insert("bani".to_string(), bani);
    ops.insert("eqir".to_string(), eqir);
    ops.insert("eqrr".to_string(), eqrr);
    ops.insert("gtri".to_string(), gtri);
    ops.insert("addi".to_string(), addi);
    ops.insert("muli".to_string(), muli);
    ops.insert("mulr".to_string(), mulr);
    let input: Vec<_> = input.split("\n").skip(1).collect();
    let re = Regex::new(r"(\w*) (\d*) (\d*) (\d*)").unwrap();
    let mut program = Vec::new();
    for l in input {
        if l.len() == 0 {
            continue;
        }
        let cap = re.captures(l).unwrap();
        program.push(Instr {
            op: ops[&cap[1]],
            a: cap[2].parse::<i32>().unwrap(),
            b: cap[3].parse::<i32>().unwrap(),
            c: cap[4].parse::<i32>().unwrap(),
        });
    }

    let mut regs: [i32; 6] = [0; 6];
    regs[0] = 0;
    let ipr = 3;
    while (regs[ipr] as usize) < program.len() {
        let instr = program[regs[ipr] as usize];
        regs = (instr.op)(regs, instr.a, instr.b, instr.c);
        regs[ipr] += 1;
    }
    println!("Part 1: {}", regs[0]);

    let mut total = 0;
    for f in 1..=10551389 {
        if 10551389 % f == 0 {
            total += f;
        }
    }
    println!("Part 2: {}", total);
}

fn addr(mut regs: [i32; 6], a: i32, b: i32, c: i32) -> [i32; 6] {
    regs[c as usize] = regs[a as usize] + regs[b as usize];
    regs
}

fn addi(mut regs: [i32; 6], a: i32, b: i32, c: i32) -> [i32; 6] {
    regs[c as usize] = regs[a as usize] + b;
    regs
}

fn mulr(mut regs: [i32; 6], a: i32, b: i32, c: i32) -> [i32; 6] {
    regs[c as usize] = regs[a as usize] * regs[b as usize];
    regs
}

fn muli(mut regs: [i32; 6], a: i32, b: i32, c: i32) -> [i32; 6] {
    regs[c as usize] = regs[a as usize] * b;
    regs
}

fn banr(mut regs: [i32; 6], a: i32, b: i32, c: i32) -> [i32; 6] {
    regs[c as usize] = regs[a as usize] & regs[b as usize];
    regs
}

fn bani(mut regs: [i32; 6], a: i32, b: i32, c: i32) -> [i32; 6] {
    regs[c as usize] = regs[a as usize] & b;
    regs
}

fn borr(mut regs: [i32; 6], a: i32, b: i32, c: i32) -> [i32; 6] {
    regs[c as usize] = regs[a as usize] | regs[b as usize];
    regs
}

fn bori(mut regs: [i32; 6], a: i32, b: i32, c: i32) -> [i32; 6] {
    regs[c as usize] = regs[a as usize] | b;
    regs
}

fn setr(mut regs: [i32; 6], a: i32, _b: i32, c: i32) -> [i32; 6] {
    regs[c as usize] = regs[a as usize];
    regs
}

fn seti(mut regs: [i32; 6], a: i32, _b: i32, c: i32) -> [i32; 6] {
    regs[c as usize] = a;
    regs
}

fn gtir(mut regs: [i32; 6], a: i32, b: i32, c: i32) -> [i32; 6] {
    regs[c as usize] = if a > regs[b as usize] { 1 } else { 0 };
    regs
}

fn gtri(mut regs: [i32; 6], a: i32, b: i32, c: i32) -> [i32; 6] {
    regs[c as usize] = if regs[a as usize] > b { 1 } else { 0 };
    regs
}

fn gtrr(mut regs: [i32; 6], a: i32, b: i32, c: i32) -> [i32; 6] {
    regs[c as usize] = if regs[a as usize] > regs[b as usize] {
        1
    } else {
        0
    };
    regs
}

fn eqir(mut regs: [i32; 6], a: i32, b: i32, c: i32) -> [i32; 6] {
    regs[c as usize] = if a == regs[b as usize] { 1 } else { 0 };
    regs
}

fn eqri(mut regs: [i32; 6], a: i32, b: i32, c: i32) -> [i32; 6] {
    regs[c as usize] = if regs[a as usize] == b { 1 } else { 0 };
    regs
}

fn eqrr(mut regs: [i32; 6], a: i32, b: i32, c: i32) -> [i32; 6] {
    regs[c as usize] = if regs[a as usize] == regs[b as usize] {
        1
    } else {
        0
    };
    regs
}
