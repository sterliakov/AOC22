use range_union_find::IntRangeUnionFind;
use sscanf::sscanf;
use std::vec::Vec;

type Coord = (isize, isize);

#[derive(Debug)]
struct Info {
    sensor: Coord,
    beacon: Coord,
    distance: usize,
}

#[inline]
fn dist(a: &Coord, b: &Coord) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

fn parse_input(inp: &str) -> (isize, Vec<Info>) {
    let mut it = inp.trim().split('\n');
    (
        it.next().unwrap().parse().unwrap(),
        it.map(|row| {
            let (xs, ys, xb, yb) = sscanf!(
                row,
                "Sensor at x={isize}, y={isize}: closest beacon is at x={isize}, y={isize}"
            )
            .expect("Cannot parse");
            Info {
                sensor: (xs, ys),
                beacon: (xb, yb),
                distance: dist(&(xs, ys), &(xb, yb)),
            }
        })
        .collect(),
    )
}

pub fn prob1(inp: &str) -> isize {
    let (y, records) = parse_input(inp);
    let mut acc = IntRangeUnionFind::new();
    for r in records.iter() {
        let dx = r.sensor.1.abs_diff(y) as isize - r.distance as isize;
        if dx < 0 {
            acc.insert_range(&(r.sensor.0 + dx..=r.sensor.0 - dx))
                .unwrap();
        }
    }
    for r in records {
        if r.beacon.1 == y {
            acc.remove_range_pair(&r.beacon.0, &r.beacon.0).unwrap();
        }
    }

    acc.into_collection::<Vec<_>>()
        .iter()
        .map(|r| r.end() - r.start() + 1)
        .sum()
}

pub fn prob2(inp: &str) -> isize {
    const SIGNS: [(isize, isize); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
    let (mut y0, records) = parse_input(inp);
    y0 *= 2;
    for r in records.iter() {
        for dx in 0..=r.distance + 1 {
            let dy = (r.distance + 1 - dx) as isize;
            for (sgnx, sgny) in SIGNS {
                let p = (r.sensor.0 + sgnx * dx as isize, r.sensor.1 + sgny * dy);
                if 0 <= p.0
                    && p.0 <= y0
                    && 0 <= p.1
                    && p.1 <= y0
                    && !records.iter().any(|r2| dist(&r2.sensor, &p) <= r2.distance)
                {
                    return p.1 + p.0 * 4_000_000;
                }
            }
        }
    }
    panic!("Failed to find");
}

#[cfg(test)]
mod tests {
    use crate::{prob1, prob2};
    use std::fs;

    #[test]
    fn part_1_example() {
        assert_eq!(
            prob1(&fs::read_to_string("inputs/task15/example.txt").unwrap()),
            26,
        );
    }

    #[test]
    fn part_2_example() {
        assert_eq!(
            prob2(&fs::read_to_string("inputs/task15/example.txt").unwrap()),
            56000011,
        );
    }
}
