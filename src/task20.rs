use linked_list::LinkedList;

fn parse_input(inp: &str) -> LinkedList<(i64, usize)> {
    inp.split('\n')
        .enumerate()
        .map(|(i, row)| (row.parse().unwrap(), i))
        .collect()
}

fn solve(list: &mut LinkedList<(i64, usize)>, repeats: usize) -> i64 {
    let l = list.len() as i64;
    for _ in 0..repeats {
        for step in 0..l as usize {
            let mut index = None;
            for (i, nxt) in list.iter().enumerate() {
                if nxt.1 == step {
                    index = Some(i);
                    break;
                }
            }
            match index {
                None => unreachable!(),
                Some(index) => {
                    let node = list.remove(index).unwrap();
                    let target = (node.0 + index as i64).rem_euclid(l - 1);
                    list.insert(target as usize, node);
                }
            }
        }
    }

    let list: Vec<_> = list.into_iter().collect();
    let f = list.iter().position(|x| x.0 == 0).unwrap();

    list[(f + 1000) % l as usize].0
        + list[(f + 2000) % l as usize].0
        + list[(f + 3000) % l as usize].0
}

pub fn prob1(inp: &str) -> i64 {
    let mut list = parse_input(inp);
    solve(&mut list, 1)
}

pub fn prob2(inp: &str) -> i64 {
    let mut list = parse_input(inp);
    list = list.iter().map(|(x, i)| (x * 811589153, *i)).collect();
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
