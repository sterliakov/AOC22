use text_io::scan;

fn solve(validator: &mut dyn FnMut((u32, u32), (u32, u32)) -> bool) -> u32 {
    let mut ans: u32 = 0;

    loop {
        let inp: String;
        scan!("{}\n", inp);
        if !inp.is_empty() {
            let (b1, e1, b2, e2): (u32, u32, u32, u32);
            scan!(inp.bytes() => "{}-{},{}-{}", b1, e1, b2, e2);
            if validator((b1, e1), (b2, e2)) {
                ans += 1;
            }
        } else {
            break ans;
        }
    }
}

#[allow(dead_code)]
pub fn prob1() {
    fn validator(a: (u32, u32), b: (u32, u32)) -> bool {
        (a.0 >= b.0 && a.1 <= b.1) || (a.0 <= b.0 && a.1 >= b.1)
    }
    let ans = solve(&mut validator);
    println!("{ans}");
}

#[allow(dead_code)]
pub fn prob2() {
    fn validator(a: (u32, u32), b: (u32, u32)) -> bool {
        (a.0 <= b.0 && b.0 <= a.1)
            || (a.0 <= b.1 && b.1 <= a.1)
            || (a.0 >= b.0 && a.1 <= b.1)
            || (a.0 <= b.0 && a.1 >= b.1)
    }
    let ans = solve(&mut validator);
    println!("{ans}");
}
