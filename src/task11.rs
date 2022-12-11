use sscanf::sscanf;
use std::collections::VecDeque;
use std::vec::Vec;

#[derive(Debug)]
struct Test {
    divisor: u64,
    target_true: usize,
    target_false: usize,
}
impl Test {
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
    fn new(op: &str, right: &str) -> Op {
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
    fn parse(block: &str) -> Monkey {
        let (_, items, op, right, divisor, target_true, target_false) = sscanf!(
            block,
            "Monkey {usize}:
  Starting items: {str}
  Operation: new = old {str} {str}
  Test: divisible by {u64}
    If true: throw to monkey {usize}
    If false: throw to monkey {usize}"
        )
        .expect("Invalid input");
        Monkey {
            hand: items
                .split(", ")
                .map(|x| x.parse().expect("Bad number"))
                .collect(),
            op: Op::new(op, right),
            test: Test {
                divisor,
                target_true,
                target_false,
            },
            inspects: 0,
        }
    }
}

fn read_input(inp: &str) -> Vec<Monkey> {
    inp.trim().split("\n\n").map(Monkey::parse).collect()
}

fn solve<const ROUNDS: u64, const RELAX: u64>(monkeys: &mut Vec<Monkey>) -> u64 {
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
    inspects.iter().rev().take(2).product::<u64>()
}

#[allow(dead_code)]
pub fn prob1(inp: &str) -> u64 {
    solve::<20, 3>(&mut read_input(inp))
}

#[allow(dead_code)]
pub fn prob2(inp: &str) -> u64 {
    solve::<10_000, 1>(&mut read_input(inp))
}

#[cfg(test)]
mod tests {
    use crate::{prob1, prob2};
    use std::fs;

    #[test]
    fn part_1_example() {
        assert_eq!(
            prob1(&fs::read_to_string("inputs/task11/example.txt").unwrap()),
            10605,
        );
    }

    #[test]
    fn part_2_example() {
        assert_eq!(
            prob2(&fs::read_to_string("inputs/task11/example.txt").unwrap()),
            2713310158,
        );
    }
}
