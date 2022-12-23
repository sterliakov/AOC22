use grid::Grid;
use itertools::Itertools;

#[derive(Clone, Debug, Default)]
struct Cell {
    occupied: bool,
    wanted: Vec<(usize, usize)>,
}

fn parse_input(inp: &str) -> Grid<Cell> {
    let mut len = 0;
    Grid::from_vec(
        inp.lines()
            .flat_map(|row| {
                len = row.len();
                row.bytes().map(|b| Cell {
                    wanted: vec![],
                    occupied: b == b'#',
                })
            })
            .collect(),
        len,
    )
}

fn solve(map: &mut Grid<Cell>, max_steps: Option<u32>) -> u32 {
    let mut directions = [
        (-1, 0), // N
        (1, 0),  // S
        (0, -1), // W
        (0, 1),  // E
    ];
    let neighbours: Vec<_> = (-1..=1)
        .cartesian_product(-1..=1)
        .filter(|&(c, r)| c != 0 || r != 0)
        .collect();

    let mut step = 0;
    let mut states = hashbrown::HashSet::<[(isize, isize); 4]>::new();
    loop {
        // Prepare
        let (h, w) = map.size();
        if map.iter_col(w - 1).any(|c| c.occupied) {
            map.push_col(vec![Cell::default(); h]);
        }
        if map.iter_col(0).any(|c| c.occupied) {
            map.insert_col(0, vec![Cell::default(); h]);
        }
        if map.iter_row(h - 1).any(|c| c.occupied) {
            map.push_row(vec![Cell::default(); map.cols()]);
        }
        if map.iter_row(0).any(|c| c.occupied) {
            map.insert_row(0, vec![Cell::default(); map.cols()]);
        }
        let (h, w) = map.size();

        // Step 1
        let mut wanted: Option<usize> = None;
        for r in 1..h as isize - 1 {
            for c in 1..w as isize - 1 {
                if !map[r as usize][c as usize].occupied
                    || neighbours
                        .iter()
                        .all(|(dc, dr)| !map[(r + dr) as usize][(c + dc) as usize].occupied)
                {
                    continue;
                }
                for (i, &(dr, dc)) in directions.iter().enumerate() {
                    let (new_r, new_c) = ((r + dr) as usize, (c + dc) as usize);
                    if dr == 0
                        && map
                            .iter_col(new_c)
                            .skip((r - 1) as usize)
                            .take(3)
                            .all(|cell| !cell.occupied)
                        || dc == 0
                            && map
                                .iter_row(new_r)
                                .skip((c - 1) as usize)
                                .take(3)
                                .all(|cell| !cell.occupied)
                    {
                        match wanted {
                            Some(v) => wanted = Some(v.min(i)),
                            None => wanted = Some(i),
                        }
                        map[new_r][new_c].wanted.push((r as usize, c as usize));
                        break;
                    }
                }
            }
        }

        let mut has_moved = false;
        if let Some(idx) = wanted {
            // Step 2
            for r in 0..h {
                for c in 0..w {
                    let cell = &mut map[r][c];
                    if cell.wanted.len() == 1 {
                        cell.occupied = true;
                        let (old_r, old_c) = cell.wanted[0];
                        map[old_r][old_c].occupied = false;
                        has_moved = true;
                    }
                    map[r][c].wanted.clear();
                }
            }
            directions[idx..].rotate_left(1);
        }

        // Finalize
        if has_moved {
            states.clear();
        } else if !states.insert(directions) {
            break step;
        }
        step += 1;
        if max_steps.is_some() && step >= max_steps.unwrap() {
            break step;
        }
    }
}

pub fn prob1(inp: &str) -> usize {
    let mut map = parse_input(inp);
    solve(&mut map, Some(10));

    let (h, w) = map.size();
    let left = (0..w)
        .find(|i| map.iter_col(*i).any(|c| c.occupied))
        .unwrap();
    let right = (0..w)
        .rfind(|i| map.iter_col(*i).any(|c| c.occupied))
        .unwrap();
    let top = (0..h)
        .find(|i| map.iter_row(*i).any(|c| c.occupied))
        .unwrap();
    let bot = (0..h)
        .rfind(|i| map.iter_row(*i).any(|c| c.occupied))
        .unwrap();
    (top..=bot)
        .map(|r| (left..=right).filter(|c| !map[r][*c].occupied).count())
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
