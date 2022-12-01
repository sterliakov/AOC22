use text_io::{scan, read};
use std::cmp::max;


fn process_spaced(processor: &mut dyn FnMut(i32)) {
    let mut inp: String;
    let mut last_failed: bool = false;
    let mut sum: i32 = 0;

    loop {
        scan!("{}\n", inp);
        if inp.len() > 0 {
            let a: i32 = read!("{}", inp.bytes());
            sum += a;
            last_failed = false;
        } else {
            if last_failed {break;}
            last_failed = true;
            processor(sum);
            sum = 0;
        }
    }
}


#[allow(dead_code)]
pub fn prob1() {
    let mut max_: i32 = 0;
    process_spaced(&mut |x| {max_ = max(max_, x)});
    println!("{}", max_);
}


#[allow(dead_code)]
pub fn prob2() {
    let mut best3: [i32; 3] = [0; 3];
    process_spaced(&mut |x| {
        best3.sort();
        if best3[0] < x {
            best3[0] = x;
        }
    });
    println!("{}", best3.iter().sum::<i32>());
}