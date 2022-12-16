use sscanf::sscanf;
use std::cmp::Reverse;
use std::collections::HashMap;
use std::iter;
use std::vec::Vec;

#[derive(Debug)]
struct Valve {
    rate: u32,
    targets: Vec<u64>,
}

#[derive(Debug)]
struct State {
    who: u64,
    opened_mask: u64,
    pressure: u32,
}

fn parse_input(inp: &str) -> (HashMap<u64, Valve>, HashMap<&str, u64>) {
    let mut indices = HashMap::<&str, u64>::new();
    (
        inp.trim()
            .split('\n')
            .map(|row| {
                let (name, rate, _, _, _, targets) = sscanf!(
                    row,
                    "Valve {str} has flow rate={u32}; tunnel{str}lead{str}to {str} {str}"
                )
                .unwrap();
                let i = 1u64 << indices.len();
                indices.insert(name, i);
                (i, rate, targets.split(", ").collect::<Vec<_>>())
            })
            .collect::<Vec<_>>()
            .iter()
            .map(|(i, rate, targets)| {
                (
                    *i,
                    Valve {
                        rate: *rate,
                        targets: targets.iter().map(|t| *indices.get(t).unwrap()).collect(),
                    },
                )
            })
            .collect(),
        indices,
    )
}

fn solve(
    graph: &HashMap<u64, Valve>,
    indices: &HashMap<&str, u64>,
    steps: u32,
) -> HashMap<(u64, u64), u32> {
    assert!(graph.len() <= 64, "Does not fir into u64 mask");
    let mut best = HashMap::<(u64, u64), u32>::new();
    let mut states = vec![State {
        who: indices["AA"],
        opened_mask: 0,
        pressure: 0,
    }];
    for t in 1..=steps {
        states = states
            .iter()
            .filter_map(|s| {
                let key = (s.who, s.opened_mask);
                if best.contains_key(&key) && s.pressure <= best[&key] {
                    return None;
                }
                best.insert(key, s.pressure);

                let Valve { rate, targets, .. } = &graph[&s.who];
                Some(
                    iter::once(if s.opened_mask & s.who == 0 && *rate > 0 {
                        Some(State {
                            who: s.who,
                            opened_mask: s.opened_mask | s.who,
                            pressure: s.pressure + rate * (steps - t),
                        })
                    } else {
                        None
                    })
                    .chain(targets.iter().map(|dest| {
                        Some(State {
                            who: *dest,
                            opened_mask: s.opened_mask,
                            pressure: s.pressure,
                        })
                    })),
                )
            })
            .flatten()
            .flatten()
            .collect();
    }
    best
}

pub fn prob1(inp: &str) -> u32 {
    let (graph, indices) = parse_input(inp);
    *solve(&graph, &indices, 30)
        .values()
        .max()
        .expect("Not empty")
}
pub fn prob2(inp: &str) -> u32 {
    let (graph, indices) = parse_input(inp);
    let best = solve(&graph, &indices, 26);
    let mut best = best.iter().collect::<Vec<_>>();
    best.sort_by_key(|(_, v)| Reverse(*v));

    let mut a = 0;
    let mut best_2 = 0;
    'outer: for ((_, m2), v2) in &best {
        for ((_, m1), v1) in &best {
            if **v1 <= best_2 {
                break;
            }
            if m1 & m2 == 0 {
                a = a.max(**v1 + **v2);
                best_2 = **v1;
                if v1 > v2 {
                    break 'outer;
                } else {
                    break;
                }
            }
        }
    }
    a
}

#[cfg(test)]
mod tests {
    use crate::{prob1, prob2};
    use std::fs;

    #[test]
    fn part_1_example() {
        assert_eq!(
            prob1(&fs::read_to_string("inputs/task16/example.txt").unwrap()),
            1651,
        );
    }

    #[test]
    fn part_2_example() {
        assert_eq!(
            prob2(&fs::read_to_string("inputs/task16/example.txt").unwrap()),
            1707,
        );
    }
}
