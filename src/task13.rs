use json::{array, JsonValue};
use std::{cmp::Ordering, vec::Vec};

fn parse_input(inp: &str) -> Vec<(JsonValue, JsonValue)> {
    inp.trim()
        .split("\n\n")
        .map(|block| {
            let (left, right) = block.split_once('\n').expect("Should be 2 lines");
            (
                json::parse(left).expect("Parse failed"),
                json::parse(right).expect("Parse failed"),
            )
        })
        .collect()
}

fn my_cmp(left: &JsonValue, right: &JsonValue, index: usize) -> Ordering {
    let (a, b) = (&left[index], &right[index]);
    if a.is_null() && b.is_null() {
        Ordering::Equal
    } else if a.is_null() {
        Ordering::Less
    } else if b.is_null() {
        Ordering::Greater
    } else if a.is_number() && b.is_number() {
        a.as_u32()
            .partial_cmp(&b.as_u32())
            .unwrap()
            .then_with(|| my_cmp(left, right, index + 1))
    } else if a.is_number() {
        my_cmp(&array![a.clone()], b, 0).then_with(|| my_cmp(left, right, index + 1))
    } else if b.is_number() {
        my_cmp(a, &array![b.clone()], 0).then_with(|| my_cmp(left, right, index + 1))
    } else {
        my_cmp(a, b, 0).then_with(|| my_cmp(left, right, index + 1))
    }
}

fn find_insertion_pos(pairs: &[&JsonValue], target: &JsonValue) -> usize {
    pairs
        .iter()
        .filter(|el| my_cmp(el, target, 0).is_le())
        .count()
}

pub fn prob1(inp: &str) -> usize {
    parse_input(inp)
        .iter()
        .enumerate()
        .map(|(i, (left, right))| {
            if my_cmp(left, right, 0).is_le() {
                i + 1
            } else {
                0
            }
        })
        .sum()
}
pub fn prob2(inp: &str) -> usize {
    let pairs = parse_input(inp);
    let pairs: Vec<_> = pairs
        .iter()
        .flat_map(|(left, right)| [left, right])
        .collect();
    let a = array![array![2]];
    let b = array![array![6]];
    (1 + find_insertion_pos(&pairs, &a)) * (find_insertion_pos(&pairs, &b) + 2)
}

#[cfg(test)]
mod tests {
    use crate::{prob1, prob2};
    use std::fs;

    #[test]
    fn part_1_example() {
        assert_eq!(
            prob1(&fs::read_to_string("inputs/task13/example.txt").unwrap()),
            13,
        );
    }

    #[test]
    fn part_2_example() {
        assert_eq!(
            prob2(&fs::read_to_string("inputs/task13/example.txt").unwrap()),
            140,
        );
    }
}
