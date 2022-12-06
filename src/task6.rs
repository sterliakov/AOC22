use std::collections::{HashSet, VecDeque};
use text_io::scan;

fn all_distinct(collection: &VecDeque<u8>) -> bool {
    let mut unique = HashSet::new();
    collection.iter().all(move |x| unique.insert(x))
}

fn solve(seq_length: usize) -> usize {
    let inp: String;
    scan!("{}\n", inp);
    let mut curr: VecDeque<u8> = VecDeque::with_capacity(seq_length);
    let mut first_idx: usize = seq_length;
    for (i, x) in inp.chars().enumerate() {
        if i >= seq_length {
            let old = curr.pop_front().unwrap();
            let x = x as u8;
            curr.push_back(x);
            if x != old && all_distinct(&curr) {
                first_idx = i + 1;
                break;
            }
        } else {
            curr.push_back(x as u8);
        }
    }
    first_idx
}

#[allow(dead_code)]
pub fn prob1() {
    println!("{}", solve(4));
}

#[allow(dead_code)]
pub fn prob2() {
    println!("{}", solve(14));
}
