use hashbrown::HashMap;
use itertools::Either;
use sscanf::sscanf;
use xxcalc::calculator::Calculator;
use xxcalc::linear_solver::LinearSolver;

type Monkey<T> = Either<T, (String, String, u8)>;

fn parse_input(inp: &str) -> HashMap<String, Monkey<i64>> {
    inp.split('\n')
        .map(|row| {
            sscanf!(row, "{String}: {i64}")
                .map(|(name, val)| (name, Either::Left(val)))
                .unwrap_or_else(|_| {
                    sscanf!(row, "{String}: {String} {String} {String}")
                        .map(|(name, m1, op, m2)| {
                            (
                                name,
                                Either::Right((m1, m2, op.chars().next().unwrap() as u8)),
                            )
                        })
                        .unwrap()
                })
        })
        .collect()
}

fn solve(monkeys: &mut HashMap<String, Monkey<i64>>, target: &str) -> i64 {
    let m = &monkeys[target].clone();
    let res = match m {
        Either::Left(val) => *val,
        Either::Right((m1, m2, op)) => {
            let m1 = solve(monkeys, m1);
            let m2 = solve(monkeys, m2);
            match op {
                b'+' => m1 + m2,
                b'-' => m1 - m2,
                b'*' => m1 * m2,
                b'/' => m1 / m2,
                _ => unreachable!(),
            }
        }
    };
    monkeys.insert(target.to_string(), Either::Left(res));
    res
}

fn solve_sym(monkeys: &mut HashMap<String, Monkey<String>>, target: &str) -> String {
    let m = &monkeys[target].clone();
    let res = match m {
        Either::Left(val) => val.to_string(),
        Either::Right((m1, m2, op)) => {
            let m1 = solve_sym(monkeys, m1);
            let m2 = solve_sym(monkeys, m2);
            format!("({m1} {} {m2})", *op as char)
        }
    };
    monkeys.insert(target.to_string(), Either::Left(res.clone()));
    res
}

pub fn prob1(inp: &str) -> i64 {
    let mut monkeys = parse_input(inp);
    solve(&mut monkeys, "root")
}

pub fn prob2(inp: &str) -> i64 {
    let monkeys = parse_input(inp);
    let mut monkeys: HashMap<_, _> = monkeys
        .into_iter()
        .map(|(k, v)| {
            let r = if k == "humn" {
                Either::Left("x".to_string())
            } else {
                match v {
                    Either::Left(val) => Either::Left(val.to_string()),
                    Either::Right(r) => Either::Right(r),
                }
            };
            (k, r)
        })
        .collect();
    let root = monkeys.get_mut("root").unwrap();
    match root {
        Either::Left(_) => unreachable!(),
        Either::Right(val) => val.2 = b'=',
    }
    let expr = solve_sym(&mut monkeys, "root");
    LinearSolver
        .process(&expr)
        .unwrap()
        .as_f64()
        .unwrap()
        .round() as i64
}

#[cfg(test)]
mod tests {
    use crate::{prob1, prob2};
    use std::fs;

    #[test]
    fn part_1_example() {
        assert_eq!(
            prob1(
                &fs::read_to_string("inputs/task21/example.txt")
                    .unwrap()
                    .trim()
            ),
            152,
        );
    }

    #[test]
    fn part_2_example() {
        assert_eq!(
            prob2(
                &fs::read_to_string("inputs/task21/example.txt")
                    .unwrap()
                    .trim()
            ),
            301,
        );
    }
}
