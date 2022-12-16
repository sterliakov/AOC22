use sscanf::sscanf;
use std::cmp::Reverse;
use std::collections::HashMap;
use std::iter;
use std::vec::Vec;

type ValveName = u64;

#[derive(Debug)]
struct Valve {
    // name: ValveName,
    rate: u32,
    targets: Vec<ValveName>,
}

#[derive(Debug)]
struct State {
    who: ValveName,
    opened_mask: u64,
    pressure: u32,
}

fn parse_input(inp: &str) -> (HashMap<ValveName, Valve>, HashMap<String, u64>) {
    let mut indices = HashMap::<String, u64>::new();
    (
        inp.trim()
            .split('\n')
            .map(|row| {
                let (name, rate, targets) = sscanf!(
                    row,
                    "Valve {String} has flow rate={u32}; tunnels lead to valves {String}"
                )
                .unwrap_or_else(|_| {
                    sscanf!(
                        row,
                        "Valve {String} has flow rate={u32}; tunnel leads to valve {String}"
                    )
                    .unwrap()
                });
                (
                    name,
                    rate,
                    targets
                        .split(", ")
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>(),
                )
            })
            .map(|(name, rate, targets)| {
                let i = 1u64 << indices.len();
                indices.insert(name, i);
                (i, rate, targets)
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
    graph: &HashMap<ValveName, Valve>,
    indices: &HashMap<String, u64>,
    steps: u32,
) -> HashMap<(ValveName, u64), u32> {
    let mut best = HashMap::<(ValveName, u64), (u32, u32)>::new();
    let mut states = vec![State {
        who: indices["AA"],
        opened_mask: 0,
        pressure: 0,
    }];
    for t in 0..steps {
        states = states
            .iter()
            .filter_map(
                |State {
                     who,
                     opened_mask: mask,
                     pressure,
                 }| {
                    let key = (*who, *mask);
                    if best.contains_key(&key) && pressure <= &best[&key].0 {
                        return None;
                    }
                    best.insert(key, (*pressure, t));

                    let Valve { rate, targets, .. } = &graph[who];
                    Some(
                        iter::once(if mask & *who as u64 == 0 && *rate > 0 {
                            Some(State {
                                who: *who,
                                opened_mask: mask | *who as u64,
                                pressure: pressure + rate * (steps - t - 1),
                            })
                        } else {
                            None
                        })
                        .chain(targets.iter().map(|dest| {
                            Some(State {
                                who: *dest,
                                opened_mask: *mask,
                                pressure: *pressure,
                            })
                        })),
                    )
                },
            )
            .flatten()
            .flatten()
            .collect();
    }
    best.retain(|_, v| v.1 == steps - 1);
    best.into_iter().map(|(k, v)| (k, v.0)).collect()
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
    'outer: for ((_, m2), v2) in best.iter() {
        for ((_, m1), v1) in best.iter() {
            if m1 & m2 == 0 {
                a = a.max(**v1 + **v2);
                if v1 > v2 {
                    break 'outer;
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
