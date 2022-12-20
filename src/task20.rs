use std::collections::VecDeque;

fn parse_input(inp: &str) -> VecDeque<(i64, usize)> {
    inp.split('\n')
        .enumerate()
        .map(|(i, row)| (row.parse().unwrap(), i))
        .collect()
}

fn solve(list: &mut VecDeque<(i64, usize)>, repeats: usize) -> i64 {
    let len = list.len();
    let lm1 = len as i64 - 1;
    for _ in 0..repeats {
        for step in 0..len {
            let index = list.iter().position(|nxt| nxt.1 == step).unwrap();
            let node = list.remove(index).unwrap();
            let target = (node.0 + index as i64).rem_euclid(lm1);
            list.insert(target as usize, node);
        }
    }

    let f = list.iter().position(|x| x.0 == 0).unwrap();
    list[(f + 1000) % len].0 + list[(f + 2000) % len].0 + list[(f + 3000) % len].0
}

pub fn prob1(inp: &str) -> i64 {
    let mut list = parse_input(inp);
    solve(&mut list, 1)
}

pub fn prob2(inp: &str) -> i64 {
    let mut list = parse_input(inp);
    list.iter_mut().for_each(|(x, _)| *x *= 811_589_153);
    solve(&mut list, 10)
}

#[cfg(test)]
mod tests {
    use crate::{prob1, prob2};
    use std::fs;

    #[test]
    fn part_1_example() {
        assert_eq!(
            prob1(
                &fs::read_to_string("inputs/task20/example.txt")
                    .unwrap()
                    .trim()
            ),
            3,
        );
    }

    #[test]
    fn part_2_example() {
        assert_eq!(
            prob2(
                &fs::read_to_string("inputs/task20/example.txt")
                    .unwrap()
                    .trim()
            ),
            1623178306,
        );
    }
}
