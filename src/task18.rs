use itertools::Itertools;
use std::{collections::VecDeque, ops::Sub, str::FromStr, vec::Vec};

static DIRECTIONS: [Point; 6] = [
    Point { x: 1, y: 0, z: 0 },
    Point { x: -1, y: 0, z: 0 },
    Point { x: 0, y: 1, z: 0 },
    Point { x: 0, y: -1, z: 0 },
    Point { x: 0, y: 0, z: 1 },
    Point { x: 0, y: 0, z: -1 },
];

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}
impl Point {
    const ONES: Self = Self { x: 1, y: 1, z: 1 };

    fn min(&self, other: &Point) -> Point {
        Point {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
            z: self.z.min(other.z),
        }
    }
    fn max(&self, other: &Point) -> Point {
        Point {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            z: self.z.max(other.z),
        }
    }
}

impl<'a, 'b> Sub<&'a Point> for &'b Point {
    type Output = Point;
    fn sub(self, other: &'a Point) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
impl FromStr for Point {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((x, y, z)) = s.split(',').map(|x| x.parse().unwrap()).collect_tuple() {
            Ok(Point { x, y, z })
        } else {
            Err("Failed to parse".to_string())
        }
    }
}

#[derive(Clone, Debug, Default)]
struct Cell {
    visited: bool,
    is_lava: bool,
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<Vec<Cell>>>,
    offset: Point,
}
impl Map {
    // In fact 1-indexed to have 1 row/col from every side and avoid boundary checks
    fn at(&mut self, index: &Point) -> &mut Cell {
        let index = index - &self.offset;
        &mut self.map[(index.z + 1) as usize][(index.y + 1) as usize][(index.x + 1) as usize]
    }
    fn checked_at(&mut self, index: &Point) -> Option<&mut Cell> {
        let index = index - &self.offset;
        self.map
            .get_mut((index.z + 1) as usize)?
            .get_mut((index.y + 1) as usize)?
            .get_mut((index.x + 1) as usize)
    }
    fn to_fit(trues: &[Point]) -> Self {
        let max_coo = trues
            .iter()
            .fold(trues[0].clone(), |acc, item| acc.max(item));
        let min_coo = trues
            .iter()
            .fold(trues[0].clone(), |acc, item| acc.min(item));
        let size = &max_coo - &min_coo;
        Self {
            map: vec![
                vec![vec![Cell::default(); size.x as usize + 3]; size.y as usize + 3];
                size.z as usize + 3
            ],
            offset: min_coo,
        }
    }
}

fn parse_input(inp: &str) -> Vec<Point> {
    inp.split('\n').flat_map(Point::from_str).collect()
}

pub fn prob1(inp: &str) -> usize {
    let cubes = parse_input(inp);
    let mut map = Map::to_fit(&cubes);
    6 * cubes.len()
        - cubes
            .into_iter()
            .map(|cube| {
                map.at(&cube).is_lava = true;
                DIRECTIONS
                    .iter()
                    .filter(|dir| map.at(&(&cube - dir)).is_lava)
                    .count()
            })
            .sum::<usize>()
            * 2
}

pub fn prob2(inp: &str) -> usize {
    let cubes = parse_input(inp);
    let mut map = Map::to_fit(&cubes);
    cubes
        .into_iter()
        .for_each(|cube| map.at(&cube).is_lava = true);

    let start = &map.offset - &Point::ONES;
    let mut queue = VecDeque::from([start]);

    let mut area = 0;
    while let Some(current) = queue.pop_front() {
        for dir in &DIRECTIONS {
            let neighbour = &current - dir;
            if let Some(el) = map.checked_at(&neighbour) {
                if el.is_lava {
                    area += 1
                } else if !el.visited {
                    el.visited = true;
                    queue.push_back(neighbour);
                }
            }
        }
    }
    area
}

#[cfg(test)]
mod tests {
    use crate::{prob1, prob2};
    use std::fs;

    #[test]
    fn part_1_example() {
        assert_eq!(
            prob1(
                &fs::read_to_string("inputs/task18/example.txt")
                    .unwrap()
                    .trim()
            ),
            64,
        );
    }

    #[test]
    fn part_2_example() {
        assert_eq!(
            prob2(
                &fs::read_to_string("inputs/task18/example.txt")
                    .unwrap()
                    .trim()
            ),
            58,
        );
    }
}
