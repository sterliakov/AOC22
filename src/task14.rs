use grid::Grid;
use itertools::Itertools;
use std::cmp::max;
use std::vec::Vec;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
enum CellType {
    #[default]
    Empty,
    Sand,
    Rock,
    Start,
}

const SOURCE: (usize, usize) = (500, 0);

fn parse_input(inp: &str) -> Grid<CellType> {
    let mk_range = |a, b| {
        if a > b {
            b..=a
        } else {
            a..=b
        }
    };

    let data: Vec<Vec<_>> = inp
        .trim()
        .split('\n')
        .map(|row| {
            row.split(" -> ")
                .map(|pair| pair.split_once(',').unwrap())
                .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
                .collect()
        })
        .collect();

    let (mut w, mut h) = (0, 0);
    data.iter().for_each(|row| {
        row.iter().for_each(|(x, y)| {
            w = max(w, *x);
            h = max(h, *y);
        })
    });
    let mut grid = Grid::new(h + 1, w + 1);
    data.iter().for_each(|row| {
        row.iter().tuple_windows().for_each(|(from, to)| {
            mk_range(from.1, to.1).for_each(|y| grid[y][from.0] = CellType::Rock);
            mk_range(from.0, to.0).for_each(|x| grid[from.1][x] = CellType::Rock);
        })
    });
    grid[SOURCE.1][SOURCE.0] = CellType::Start;
    grid
}

fn process(grid: &mut Grid<CellType>) -> usize {
    let (max_row, max_col) = grid.size();
    let mut steps = 0usize;
    loop {
        steps += 1;
        let (mut col, mut row) = SOURCE;
        let mut stepped = false;
        loop {
            row += 1;
            if row >= max_row || col >= max_col {
                return steps;
            } else if grid[row][col] == CellType::Empty {
                // continue;
            } else if grid[row].get(col - 1) == Some(&CellType::Empty) {
                col -= 1;
            } else if grid[row].get(col + 1) == Some(&CellType::Empty) {
                col += 1;
            } else {
                grid[row - 1][col] = CellType::Sand;
                break;
            }
            stepped = true;
        }
        if !stepped {
            break steps;
        }
    }
}

pub fn prob1(inp: &str) -> usize {
    let mut grid = parse_input(inp);
    process(&mut grid) - 1
}

pub fn prob2(inp: &str) -> usize {
    let mut grid = parse_input(inp);
    let max_row = grid.rows();
    (0..=max_row).for_each(|_| grid.push_col(vec![CellType::Empty; max_row]));
    let max_col = grid.cols();
    grid.push_row(vec![CellType::Empty; max_col]);
    grid.push_row(vec![CellType::Rock; max_col]);
    process(&mut grid)
}

#[cfg(test)]
mod tests {
    use crate::{prob1, prob2};
    use std::fs;

    #[test]
    fn part_1_example() {
        assert_eq!(
            prob1(&fs::read_to_string("inputs/task14/example.txt").unwrap()),
            24,
        );
    }

    #[test]
    fn part_2_example() {
        assert_eq!(
            prob2(&fs::read_to_string("inputs/task14/example.txt").unwrap()),
            93,
        );
    }
}
