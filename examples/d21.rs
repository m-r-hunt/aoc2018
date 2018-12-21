/* Here are my notes from decompiling the code:

#ip 5
 0: seti 123 0 4		r4 = 123
 1: bani 4 456 4		r4 = r4 & 456
 2: eqri 4 72 4			r4 = r4 == 72
 3: addr 4 5 5			GOTO 4 or 5
 4: seti 0 0 5			GOTO 1
 5: seti 0 0 4			r4 = 0
 6: bori 4 65536 3		r3 = r4 | 65536
 7: seti 4332021 4 4		r4 = 4332021
 8: bani 3 255 2   		r2 = r3 & 255
 9: addr 4 2 4 			r4 = r4 + r2
10: bani 4 16777215 4		r4 = r4 & 16777215
11: muli 4 65899 4  		r4 = r4 * 65899
12: bani 4 16777215 4		r4 = r4 & 16777215
13: gtir 256 3 2    		r2 = 256 > r3
14: addr 2 5 5			GOTO 15 or 16
15: addi 5 1 5			GOTO 17
16: seti 27 5 5			GOTO 28
17: seti 0 2 2			r2 = 0
18: addi 2 1 1			r1 = r2 + 1
19: muli 1 256 1		r1 = r1 * 256
20: gtrr 1 3 1 			r1 = r1 > r3
21: addr 1 5 5			GOTO 22 or 23
22: addi 5 1 5			GOTO 24
23: seti 25 2 5			GOTO 26
24: addi 2 1 2			r2 = r2 + 1
25: seti 17 3 5			GOTO 18
26: setr 2 7 3			r3 = r2
27: seti 7 1 5			GOTO 8
28: eqrr 4 0 2			r2 = r4 == r0
29: addr 2 5 5			GOTO 30 or 31
30: seti 5 6 5			GOTO 6

r4 = 123;
loop {
    r4 &= 456;
    if r4 == 72 {
        break;
    }
}
r4 = 0;

// 6
r3 = r4 | 65536; // 0b0001_0000_0000_0000_0000
r4 = 4332021; // 0b0100_0010_0001_1001_1111_0101

// 8
r2 = r3 & 255; // 0b1111_1111
r4 += r2;
r4 &= 16777215;
r4 *= 65899;
r4 &= 16777215;
if r3 < 256 {
    //goto 28
}
r3 /= 256;
// goto 8

// 28
if r4 == r0 {
    // halt
}
// goto 6

After this we've got the core loops in a sensible form, worked out the
halting condition, and we can write code to quickly iterate the core
loop and produce values of r0 to make it halt. The question then is
easy.

*/

fn main() {
    let mut seen = std::collections::HashMap::new();
    let mut r4: u64 = 0;
    let mut last = 0;
    let mut p1 = false;
    loop {
        let mut r3: u64 = r4 | 65536;
        r4 = 4332021;

        loop {
            r4 += r3 & 255;
            r4 &= 16777215;
            r4 *= 65899;
            r4 &= 16777215;

            if r3 < 256 {
                match seen.get(&r4) {
                    None => {
                        seen.insert(r4, true);
                        last = r4;
                        if !p1 {
                            println!("Part 1: {}", r4);
                            p1 = true;
                        }
                        break;
                    }
                    Some(_) => {
                        println!("Part 2: {}", last);
                        return;
                    }
                }
            }
            r3 /= 256;
        }
    }
}
