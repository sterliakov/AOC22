use std::collections::VecDeque;
use std::vec::Vec;

type Coord = (usize, usize);

#[derive(Debug)]
struct Point {
    elevation: u8,
    visited: bool,
}

#[derive(Debug)]
struct HillMap {
    elevations: Vec<Vec<Point>>,
    start: Coord,
    end: Coord,
}
impl HillMap {
    fn from_string(inp: &str) -> Self {
        let mut start: Coord = (0, 0);
        let mut end: Coord = (0, 0);
        let elevations = inp
            .split('\n')
            .enumerate()
            .map(|(i, row)| {
                row.chars()
                    .enumerate()
                    .map(|(j, c)| match c {
                        'S' => {
                            start = (i, j);
                            Point {
                                elevation: 0,
                                visited: false,
                            }
                        }
                        'E' => {
                            end = (i, j);
                            Point {
                                elevation: b'z' - b'a',
                                visited: false,
                            }
                        }
                        _ => Point {
                            elevation: c as u8 - b'a',
                            visited: false,
                        },
                    })
                    .collect()
            })
            .collect();
        HillMap {
            elevations,
            start,
            end,
        }
    }
    fn neighbours_unvisited(&self, p: Coord) -> Vec<Coord> {
        let v_empty = Vec::<Point>::new();
        let current_elev = self.elevations[p.0][p.1].elevation;
        [(0, 1), (0, -1), (1, 0), (-1, 0)]
            .into_iter()
            .filter_map(|(dx, dy)| {
                let x = (p.0 as isize + dx) as usize;
                let y = (p.1 as isize + dy) as usize;
                let el = self.elevations.get(x).unwrap_or(&v_empty).get(y);
                if let Some(el) = el {
                    // Reverse direction (moving from z to a)
                    if current_elev <= el.elevation + 1 && !el.visited {
                        Some((x, y))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect()
    }
}

fn solve(map: &mut HillMap, is_end: &mut dyn FnMut(&HillMap, &Coord) -> bool) -> Option<usize> {
    let mut queue = VecDeque::from(vec![(map.end, 0)]);
    while let Some((pos, len)) = queue.pop_front() {
        if is_end(map, &pos) {
            return Some(len);
        }
        for next in map.neighbours_unvisited(pos) {
            queue.push_back((next, len + 1));
            map.elevations[next.0][next.1].visited = true;
        }
    }
    None
}

pub fn prob1(inp: &str) -> usize {
    let mut map = HillMap::from_string(inp);
    solve(&mut map, &mut |map, at| map.start == *at).expect("Should be solvable!")
}
pub fn prob2(inp: &str) -> usize {
    let mut map = HillMap::from_string(inp);
    solve(&mut map, &mut |map, at| {
        map.elevations[at.0][at.1].elevation == 0
    })
    .expect("Should be solvable!")
}

#[cfg(test)]
mod tests {
    use crate::{prob1, prob2};
    use std::fs;

    #[test]
    fn part_1_example() {
        assert_eq!(
            prob1(&fs::read_to_string("inputs/task12/example.txt").unwrap()),
            31,
        );
    }

    #[test]
    fn part_2_example() {
        assert_eq!(
            prob2(&fs::read_to_string("inputs/task12/example.txt").unwrap()),
            29,
        );
    }
}
