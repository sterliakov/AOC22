use hashbrown::HashSet;
use rustc_hash::FxHasher as Hasher;
use sscanf::sscanf;
use std::hash::BuildHasherDefault;
use std::{collections::VecDeque, vec::Vec};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct State {
    robots: [i16; 4],
    balance: [i16; 4],
    completed: bool,
    allowed: u8,
    bought: Option<u8>,
}

fn parse_input(inp: &str) -> Vec<[[i16; 4]; 4]> {
    inp.split('\n')
        .map(|row| sscanf!(row, "Blueprint {usize}: Each ore robot costs {i16} ore. Each clay robot costs {i16} ore. Each obsidian robot costs {i16} ore and {i16} clay. Each geode robot costs {i16} ore and {i16} obsidian.").unwrap())
        .map(|(_, ore, clay, obs_1, obs_2, geo_1, geo_2)| [
            [ore, 0, 0, 0],
            [clay, 0, 0, 0],
            [obs_1, obs_2, 0, 0],
            [geo_1, 0, geo_2, 0],
        ])
        .collect()
}

fn score(blueprint: &[[i16; 4]], steps: i16) -> i16 {
    let max_spends = blueprint.iter().fold([0, 0, 0, 0], |acc, x| {
        acc.iter()
            .zip(x)
            .map(|(a, b)| *a.max(b))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    });

    let mut states = VecDeque::from([State {
        robots: [1, 0, 0, 0],
        balance: [0, 0, 0, 0],
        completed: false,
        allowed: 0b1111,
        bought: None,
    }]);
    for step in 0..steps {
        while let Some(State {
            mut robots,
            mut balance,
            completed,
            mut allowed,
            bought,
        }) = states.pop_front()
        {
            if completed {
                states.push_front(State {
                    robots,
                    balance,
                    completed: false,
                    allowed,
                    bought,
                });

                states = HashSet::<_, BuildHasherDefault<Hasher>>::from_iter(
                    states.into_iter().map(|mut s| {
                        s.completed = false;
                        s
                    }),
                )
                .into_iter()
                .collect();
                break;
            }

            allowed = if bought.is_none() {
                if step < steps - 1 {
                    blueprint
                        .iter()
                        .enumerate()
                        .map(|(idx, r)| {
                            let flag = 1 << idx;
                            if allowed & flag == 0 || idx < 3 && robots[idx] >= max_spends[idx] {
                                return 0;
                            }
                            let mut new_balance = balance;
                            for (b, p) in new_balance.iter_mut().zip(r) {
                                *b -= p;
                                if *b < 0 {
                                    return 0;
                                }
                            }
                            states.push_front(State {
                                robots,
                                balance: new_balance,
                                completed: false,
                                allowed,
                                bought: Some(idx as u8),
                            });
                            flag
                        })
                        .fold(0, |acc, x| acc | x)
                } else {
                    0b1111
                }
            } else {
                0
            };

            robots.iter_mut().enumerate().for_each(|(i, r)| {
                balance[i] += *r;
                if i != 3 {
                    balance[i] = balance[i].min((steps - step - 1) * max_spends[i]);
                }
            });
            if let Some(b) = bought {
                robots[b as usize] += 1;
            }

            states.push_back(State {
                robots,
                balance,
                completed: true,
                allowed: 0b1111 & !allowed,
                bought: None,
            });
        }
        println!("Step {step} done, with {} states", states.len());
    }
    states.iter().map(|s| s.balance[3]).max().unwrap()
}

pub fn prob1(inp: &str) -> i16 {
    let blueprints = parse_input(inp);
    blueprints
        .into_iter()
        .enumerate()
        .map(|(i, b)| (i + 1) as i16 * score(&b, 24))
        .sum()
}

pub fn prob2(inp: &str) -> i16 {
    let blueprints = parse_input(inp);
    blueprints
        .into_iter()
        .take(3)
        .map(|b| score(&b, 32))
        .product()
}

#[cfg(test)]
mod tests {
    use crate::{prob1, prob2};
    use std::fs;

    #[test]
    fn part_1_example() {
        assert_eq!(
            prob1(
                &fs::read_to_string("inputs/task19/example.txt")
                    .unwrap()
                    .trim()
            ),
            33,
        );
    }

    #[test]
    fn part_2_example() {
        assert_eq!(
            prob2(
                &fs::read_to_string("inputs/task19/example.txt")
                    .unwrap()
                    .trim()
            ),
            62,
        );
    }
}
