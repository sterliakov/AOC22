use std::collections::VecDeque;
use std::vec::Vec;
use text_io::{read, scan};

#[derive(Debug)]
struct Test {
    divisor: u64,
    target_true: usize,
    target_false: usize,
}
impl Test {
    fn read_next() -> Test {
        let divisor: u64 = read!("  Test: divisible by {}\n");
        let target_true: usize = read!("    If true: throw to monkey {}\n");
        let target_false: usize = read!("    If false: throw to monkey {}\n");
        Test {
            divisor,
            target_true,
            target_false,
        }
    }
    fn get_target(&self, worry: &u64) -> usize {
        if worry % self.divisor == 0 {
            self.target_true
        } else {
            self.target_false
        }
    }
}

#[derive(Debug)]
enum Op {
    Add(u64),
    Mul(u64),
    Square,
}
impl Op {
    fn read_next() -> Op {
        let op: String = read!("  Operation: new = old {}\n");
        let (op, right) = op.split_once(' ').unwrap();
        match (op, right) {
            ("*", "old") => Self::Square,
            ("*", r) => Self::Mul(r.parse().expect("Must be a number")),
            ("+", r) => Self::Add(r.parse().expect("Must be a number")),
            _ => unreachable!("Unknown input format"),
        }
    }
    fn exec(&self, left: &u64) -> u64 {
        match self {
            Self::Square => left * left,
            Self::Mul(r) => left * r,
            Self::Add(r) => left + r,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    hand: VecDeque<u64>,
    op: Op,
    test: Test,
    inspects: u64,
}
impl Monkey {
    fn read_next() -> Monkey {
        let line: String = read!("  Starting items: {}\n");
        let hand: VecDeque<u64> = line
            .split(", ")
            .map(|x| x.parse().expect("Bad number"))
            .collect();
        Monkey {
            hand,
            op: Op::read_next(),
            test: Test::read_next(),
            inspects: 0,
        }
    }
}

fn read_input() -> Vec<Monkey> {
    let mut monkeys = Vec::<Monkey>::new();
    loop {
        let mut inp: String;
        scan!("{}\n", inp);
        if inp.is_empty() {
            break monkeys;
        }
        monkeys.push(Monkey::read_next());
        scan!("{}\n", inp);
    }
}

fn solve<const ROUNDS: u64, const RELAX: u64>() {
    let mut monkeys = read_input();
    let modulo =
        monkeys
            .iter()
            .map(|x| x.test.divisor)
            .fold(1, |acc, x| if acc % x == 0 { acc } else { acc * x });
    for _ in 1..=ROUNDS {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].hand.pop_front() {
                let m = &monkeys[i];
                let worry = m.op.exec(&item) % modulo / RELAX;
                let idx = m.test.get_target(&worry);
                monkeys[idx].hand.push_back(worry);
                monkeys[i].inspects += 1;
            }
            monkeys[i].hand.clear();
        }
    }
    let mut inspects: Vec<_> = monkeys.iter().map(|m| m.inspects).collect();
    inspects.sort();
    println!("{}", inspects.iter().rev().take(2).product::<u64>());
}

#[allow(dead_code)]
pub fn prob1() {
    solve::<20, 3>();
}

#[allow(dead_code)]
pub fn prob2() {
    solve::<10_000, 1>();
}
