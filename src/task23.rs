use itertools::Itertools;
use std::ops::BitAnd;

const N: (isize, isize) = (-1, 0);
const S: (isize, isize) = (1, 0);
const W: (isize, isize) = (0, -1);
const E: (isize, isize) = (0, 1);

#[derive(Clone, Debug, Default)]
struct Cell {
    occupied: bool,
    wanted: Vec<(usize, usize)>,
}

fn parse_input(inp: &str) -> Vec<Vec<Cell>> {
    inp.lines()
        .map(|row| {
            row.bytes()
                .map(|b| Cell {
                    wanted: vec![],
                    occupied: b == b'#',
                })
                .collect()
        })
        .collect()
}

fn get_actual_width(map: &Vec<Vec<Cell>>) -> (usize, usize) {
    let w = map[0].len() - 1;

    let mut left = usize::MAX;
    for row in map {
        left = left.min(row.iter().position(|c| c.occupied).unwrap_or(usize::MAX));
        if left == 0 {
            break;
        }
    }
    let mut right = 0;
    for row in map {
        right = right.max(row.iter().rposition(|c| c.occupied).unwrap_or(0));
        if right == w {
            break;
        }
    }
    (left, right)
}

fn solve(map: &mut Vec<Vec<Cell>>, max_steps: Option<u32>) -> u32 {
    let mut directions = [N, S, W, E];

    let mut step = 0;
    let mut states = hashbrown::HashSet::<[(isize, isize); 4]>::new();
    loop {
        // Prepare
        let (h, w) = (map.len(), map[0].len());
        let (left, right) = get_actual_width(map);
        if left == 0 {
            map.iter_mut()
                .for_each(|row| row.insert(0, Cell::default()));
        }
        if right == w - 1 {
            map.iter_mut().for_each(|row| row.push(Cell::default()));
        }
        if map[0].iter().any(|c| c.occupied) {
            map.insert(0, vec![Cell::default(); map[0].len()]);
        }
        if map[h - 1].iter().any(|c| c.occupied) {
            map.push(vec![Cell::default(); map[0].len()]);
        }
        let (h, w) = (map.len(), map[0].len());

        // Step 1
        let mut wanted = None;
        for r in 1..h - 1 {
            for c in 1..w - 1 {
                if !map[r][c].occupied
                    || (-1..=1)
                        .cartesian_product(-1..=1)
                        .map(|(dc, dr)| {
                            dr == 0 && dc == 0
                                || !map[(r as isize + dr) as usize][(c as isize + dc) as usize]
                                    .occupied
                        })
                        .reduce(bool::bitand)
                        .unwrap()
                {
                    continue;
                }
                for (i, &(dr, dc)) in directions.iter().enumerate() {
                    let (new_r, new_c) = ((r as isize + dr) as usize, (c as isize + dc) as usize);
                    if dr == 0
                        && (-1..=1)
                            .map(|dr| map[(r as isize + dr) as usize][new_c].occupied)
                            .all(|c| !c)
                        || dc == 0
                            && (-1..=1)
                                .map(|dc| !map[new_r][(c as isize + dc) as usize].occupied)
                                .reduce(bool::bitand)
                                .unwrap()
                    {
                        match wanted {
                            None => wanted = Some(i),
                            Some(v) => wanted = Some(v.min(i)),
                        }
                        map[new_r][new_c].wanted.push((r, c));
                        break;
                    }
                }
            }
        }

        // Step 2
        let mut moved = false;
        for r in 0..h {
            for c in 0..w {
                if map[r][c].wanted.len() == 1 {
                    map[r][c].occupied = true;
                    let (old_r, old_c) = map[r][c].wanted[0];
                    map[old_r][old_c].occupied = false;
                    moved = true;
                }
                map[r][c].wanted.clear();
            }
        }

        // Finalize
        if let Some(idx) = wanted {
            directions[idx..].rotate_left(1);
        }

        if moved {
            states.clear();
        } else if !states.insert(directions) {
            break;
        }
        step += 1;
        if max_steps.is_some() && step >= max_steps.unwrap() {
            break;
        }
    }
    step
}

pub fn prob1(inp: &str) -> usize {
    let mut map = parse_input(inp);
    solve(&mut map, Some(10));

    // for row in &map {
    //     println!(
    //         "{}",
    //         row.iter()
    //             .map(|c| if c.occupied { "#" } else { "." })
    //             .collect::<Vec<_>>()
    //             .join("")
    //     );
    // }
    // println!();

    let (left, right) = get_actual_width(&map);
    let top = map
        .iter()
        .position(|row| row.iter().any(|c| c.occupied))
        .unwrap();
    let bot = map
        .iter()
        .rposition(|row| row.iter().any(|c| c.occupied))
        .unwrap();
    map[top..=bot]
        .iter()
        .map(|row| row[left..=right].iter().filter(|c| !c.occupied).count())
        .sum()
}

pub fn prob2(inp: &str) -> u32 {
    let mut map = parse_input(inp);
    solve(&mut map, None)
}

#[cfg(test)]
mod tests {
    use super::{prob1, prob2};
    use std::fs;

    #[test]
    fn part_1_example() {
        let inp = &fs::read_to_string("inputs/task23/example.txt").unwrap();
        let inp = inp.strip_suffix('\n').unwrap_or(&inp);
        assert_eq!(prob1(inp), 110);
    }

    #[test]
    fn part_2_example() {
        let inp = &fs::read_to_string("inputs/task23/example.txt").unwrap();
        let inp = inp.strip_suffix('\n').unwrap_or(&inp);
        assert_eq!(prob2(inp), 20);
    }
}
