use regex::Regex;
use std::vec::Vec;
use text_io::scan;

type Mutator = dyn FnMut(&mut [Vec<u8>], usize, usize, usize);

fn solve(mutate: &mut Mutator) -> Vec<Vec<u8>> {
    let mut stacks: Vec<Vec<u8>> = Vec::new();
    let re = Regex::new(r"(\[(?P<ch>\w)\]| (?P<empty> )  ?)").unwrap();

    // First part: header
    loop {
        let inp: String;
        scan!("{}\n", inp);
        if !inp.is_empty() {
            // Ignore numbering 1..n
            if !inp.chars().nth(2).unwrap().is_numeric() {
                for (i, ch) in re.captures_iter(&inp).enumerate() {
                    if stacks.get(i).is_none() {
                        stacks.push(Vec::new());
                    }
                    if ch.name("ch").is_some() {
                        stacks[i].push(
                            ch["ch"].chars().next().expect("Non-empty string expected") as u8,
                        );
                    }
                }
            }
        } else {
            break;
        }
    }

    // We were reading top-to-bottom, reverse
    stacks.iter_mut().for_each(|x| x.reverse());

    loop {
        let inp: String;
        scan!("{}\n", inp);
        if !inp.is_empty() {
            let (count, mut from, mut to): (usize, usize, usize);
            scan!(inp.bytes() => "move {} from {} to {}", count, from, to);
            from -= 1;
            to -= 1;
            mutate(&mut stacks, from, to, count);
        } else {
            break;
        }
    }
    stacks
}

fn display(stacks: Vec<Vec<u8>>) {
    stacks
        .iter()
        .for_each(|s| print!("{}", *s.last().expect("Can't be empty") as char));
    println!();
}

#[allow(dead_code)]
pub fn prob1() {
    fn mutate(stacks: &mut [Vec<u8>], from: usize, to: usize, count: usize) {
        for _ in 0..count {
            let tmp = stacks[from].pop().expect("Not enough items");
            stacks[to].push(tmp);
        }
    }
    let ans = solve(&mut mutate);
    display(ans);
}

#[allow(dead_code)]
pub fn prob2() {
    fn mutate(stacks: &mut [Vec<u8>], from: usize, to: usize, count: usize) {
        let idx = stacks[from].len() - count;
        let tmp: Vec<u8> = stacks[from].drain(idx..).collect();
        stacks[to].extend(tmp);
    }
    let ans = solve(&mut mutate);
    display(ans);
}
