use grid::Grid;
use hashbrown::HashSet;

#[derive(Clone, Debug)]
enum Dir {
    Up = 1,
    Right = 2,
    Down = 3,
    Left = 4,
}
use Dir::{Down, Left, Right, Up};

fn parse_input(inp: &str) -> Grid<Vec<Dir>> {
    let mut len = 0;
    let data: Vec<_> = inp.lines().collect();
    Grid::from_vec(
        data.iter()
            .skip(1)
            .take(data.len() - 2)
            .flat_map(|row| {
                len = row.len() - 2;
                row.bytes().skip(1).take(len).map(|b| match b {
                    b'>' => vec![Right],
                    b'v' => vec![Down],
                    b'<' => vec![Left],
                    b'^' => vec![Up],
                    b'.' => vec![],
                    _ => unreachable!(),
                })
            })
            .collect(),
        len,
    )
}

fn solve(
    mut map: Grid<Vec<Dir>>,
    start: (isize, isize),
    end: (isize, isize),
) -> (usize, Grid<Vec<Dir>>) {
    let (h, w) = map.size();
    let mut states = HashSet::<(isize, isize)>::from([start]);
    let mut best = None;
    let mut step = 0;
    loop {
        let mut new_map = Grid::<Vec<Dir>>::new(h, w);
        for r in 0..h {
            for c in 0..w {
                for b in map[r][c].drain(..) {
                    let (new_r, new_c) = match b {
                        Right => (r, (c + 1) % w),
                        Left => (r, (c as isize - 1).rem_euclid(w as isize) as usize),
                        Up => ((r as isize - 1).rem_euclid(h as isize) as usize, c),
                        Down => ((r + 1) % h, c),
                    };
                    new_map[new_r][new_c].push(b)
                }
            }
        }
        map = new_map;

        states = states
            .into_iter()
            .flat_map(|s| {
                [(0, 0), (0, 1), (0, -1), (1, 0), (-1, 0)]
                    .iter()
                    .filter_map(|(dr, dc)| {
                        let new_r = s.0 + dr;
                        let new_c = s.1 + dc;
                        if new_r == end.0 && new_c == end.1 {
                            best = Some(step + 1);
                            None
                        } else if new_r == start.0 && new_c == start.1 {
                            Some((new_r, new_c))
                        } else {
                            match map.get(new_r as usize, new_c as usize) {
                                None => None,
                                Some(p) => {
                                    if p.is_empty() {
                                        Some((new_r, new_c))
                                    } else {
                                        None
                                    }
                                }
                            }
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        match best {
            None => {}
            Some(best) => break (best, map),
        }
        step += 1;
    }
}

pub fn prob1(inp: &str) -> usize {
    let map = parse_input(inp);
    let (h, w) = map.size();
    solve(map, (-1, 0), (h as isize, w as isize - 1)).0
}

pub fn prob2(inp: &str) -> usize {
    let map = parse_input(inp);
    let (h, w) = map.size();
    let (first, map) = solve(map, (-1, 0), (h as isize, w as isize - 1));
    let (second, map) = solve(map, (h as isize, w as isize - 1), (-1, 0));
    let (third, _) = solve(map, (-1, 0), (h as isize, w as isize - 1));
    first + second + third
}

#[cfg(test)]
mod tests {
    use super::{prob1, prob2};
    use std::fs;

    #[test]
    fn part_1_example() {
        let inp = &fs::read_to_string("inputs/task24/example.txt").unwrap();
        let inp = inp.strip_suffix('\n').unwrap_or(&inp);
        assert_eq!(prob1(inp), 18);
    }

    #[test]
    fn part_2_example() {
        let inp = &fs::read_to_string("inputs/task24/example.txt").unwrap();
        let inp = inp.strip_suffix('\n').unwrap_or(&inp);
        assert_eq!(prob2(inp), 54);
    }
}
