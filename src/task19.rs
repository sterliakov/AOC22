use itertools::Itertools;
use sscanf::sscanf;
use std::{collections::VecDeque, vec::Vec};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum Status {
    NotProcessed = 0,
    BoughtRobot = 1,
    Finished = 2,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum Resource {
    Ore(i32),
    Clay(i32),
    Obsidian(i32),
    #[allow(dead_code)]
    Geode(i32),
}
impl Resource {
    fn as_index_and_val(&self) -> (usize, i32) {
        match self {
            Self::Ore(i) => (0, *i),
            Self::Clay(i) => (1, *i),
            Self::Obsidian(i) => (2, *i),
            Self::Geode(i) => (3, *i),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct State {
    rob: [(i32, i32); 4],
    bal: [i32; 4],
    status: Status,
    allowed: u8,
}

fn parse_input(inp: &str) -> Vec<[Vec<Resource>; 4]> {
    inp.split('\n')
        .map(|row| sscanf!(row, "Blueprint {usize}: Each ore robot costs {i32} ore. Each clay robot costs {i32} ore. Each obsidian robot costs {i32} ore and {i32} clay. Each geode robot costs {i32} ore and {i32} obsidian.").unwrap())
        .map(|(_, ore, clay, obs_1, obs_2, geo_1, geo_2)| [
            vec![Resource::Ore(ore)],
            vec![Resource::Ore(clay)],
            vec![Resource::Ore(obs_1), Resource::Clay(obs_2)],
            vec![Resource::Ore(geo_1), Resource::Obsidian(geo_2)],
        ])
        .collect()
}

fn score(blueprint: &[Vec<Resource>], steps: i32) -> i32 {
    let max_spends = blueprint.iter().fold([0, 0, 0, 0], |mut acc, x| {
        x.iter().for_each(|p| {
            let (pos, val) = p.as_index_and_val();
            acc[pos] = acc[pos].max(val);
        });
        acc
    });

    let mut states = VecDeque::from([State {
        rob: [(1, 1), (0, 0), (0, 0), (0, 0)],
        bal: [0, 0, 0, 0],
        status: Status::NotProcessed,
        allowed: 0b1111,
    }]);
    for step in 0..steps {
        while let Some(State {
            mut rob,
            mut bal,
            status,
            mut allowed,
        }) = states.pop_front()
        {
            if status == Status::Finished {
                states.push_front(State {
                    rob,
                    bal,
                    status,
                    allowed,
                });
                states
                    .iter_mut()
                    .for_each(|s| s.status = Status::NotProcessed);
                states = states.into_iter().unique().collect();
                break;
            }

            allowed = if status == Status::NotProcessed {
                if step < steps - 1 {
                    blueprint
                        .iter()
                        .enumerate()
                        .map(|(idx, r)| {
                            if allowed & (1 << idx) == 0 {
                                return 0;
                            }
                            if idx < 3 && rob[idx].1 >= max_spends[idx] {
                                return 0;
                            }
                            let mut new_balance = bal;
                            r.iter().for_each(|p| match p {
                                Resource::Ore(i) => new_balance[0] -= i,
                                Resource::Clay(i) => new_balance[1] -= i,
                                Resource::Obsidian(i) => new_balance[2] -= i,
                                Resource::Geode(i) => new_balance[3] -= i,
                            });
                            if new_balance.iter().min().unwrap() >= &0 {
                                let mut new_rob = rob;
                                new_rob[idx].1 += 1;
                                states.push_front(State {
                                    rob: new_rob,
                                    bal: new_balance,
                                    status: Status::BoughtRobot,
                                    allowed,
                                });
                                1 << idx
                            } else {
                                0
                            }
                        })
                        .fold(0, |acc, x| acc | x)
                } else {
                    0b1111
                }
            } else {
                0
            };

            rob.iter_mut().enumerate().for_each(|(i, r)| {
                bal[i] += r.0;
                if i != 3 {
                    bal[i] = bal[i].min((steps - step - 1) * max_spends[i]);
                }
                r.0 = r.1;
            });
            states.push_back(State {
                rob,
                bal,
                status: Status::Finished,
                allowed: 0b1111 & !allowed,
            });
        }
        // println!("Step {step} done, with {} states", states.len());
    }
    states.iter().map(|s| s.bal[3]).max().unwrap()
}

pub fn prob1(inp: &str) -> i32 {
    let blueprints = parse_input(inp);
    blueprints
        .into_iter()
        .enumerate()
        .map(|(i, b)| (i + 1) as i32 * score(&b, 24))
        .sum()
}

pub fn prob2(inp: &str) -> i32 {
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
