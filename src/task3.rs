use std::collections::HashSet;
use text_io::scan;

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn score(c: char) -> usize {
    match ALPHABET.chars().position(|a| a == c) {
        Some(x) => x + 1,
        None => panic!("Unknown letter"),
    }
}

#[allow(dead_code)]
pub fn prob1() {
    let mut ans: usize = 0;
    loop {
        let inp: String;
        scan!("{}\n", inp);
        let half_length = inp.len() / 2;
        if half_length != 0 {
            let s1 = &inp[..half_length];
            let s2: HashSet<char> = inp[half_length..].chars().collect();
            let rpt = match s1.chars().find(|c| s2.contains(c)) {
                Some(x) => x,
                None => {
                    println!("{inp}");
                    panic!("Repeated char not found.");
                }
            };
            ans += score(rpt);
        } else {
            break;
        }
    }
    println!("{ans}");
}

#[allow(dead_code)]
pub fn prob2() {
    let mut ans: usize = 0;

    loop {
        let inp1: String;
        scan!("{}\n", inp1);
        if !inp1.is_empty() {
            let inp2: String;
            let inp3: String;
            scan!("{}\n{}\n", inp2, inp3);
            let s2: HashSet<char> = inp2.chars().collect();
            let s3: HashSet<char> = inp3.chars().collect();
            let rpt = match inp1.chars().find(|c| s2.contains(c) && s3.contains(c)) {
                Some(x) => x,
                None => {
                    println!("{inp1} {inp2} {inp3}");
                    panic!("Repeated char not found.");
                }
            };
            ans += score(rpt);
        } else {
            break;
        }
    }
    println!("{ans}");
}
