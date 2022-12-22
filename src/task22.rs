#[derive(Clone, Copy, Debug)]
enum Dir {
    R = 0,
    D = 1,
    L = 2,
    U = 3,
}
impl Dir {
    const ORDER: [Self; 4] = [R, D, L, U];
    const OFFSETS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    fn next(&self) -> Self {
        Self::ORDER[(*self as usize + 1) % 4]
    }
    fn prev(&self) -> Self {
        Self::ORDER[(*self as usize + 3) % 4]
    }
    fn offset(&self) -> (isize, isize) {
        Self::OFFSETS[*self as usize]
    }
}
use Dir::{D, L, R, U};

fn parse_input(inp: &str) -> (Vec<Vec<u8>>, &str) {
    let (map, moves) = inp.split_once("\n\n").unwrap();
    let map: Vec<_> = map.lines().map(|l| l.as_bytes().to_vec()).collect();
    (map, moves)
}

fn walk(
    map: &[Vec<u8>],
    moves: &str,
    wrap: impl Fn(&[Vec<u8>], isize, isize, Dir) -> (isize, isize, Dir),
) -> usize {
    let mut r = 0isize;
    let mut c = map[0].iter().position(|c| c == &b'.').unwrap() as isize;
    let mut dir = R;

    let mut chars = moves.chars().peekable();
    while let Some(chr) = chars.next() {
        match chr {
            'L' => dir = dir.prev(),
            'R' => dir = dir.next(),
            chr => {
                let mut steps = chr.to_digit(10).unwrap();
                while let Some(chr) = chars.peek() {
                    if !chr.is_ascii_digit() {
                        break;
                    }
                    steps = steps * 10 + chars.next().unwrap().to_digit(10).unwrap();
                }
                for _ in 0..steps {
                    let (dr, dc) = dir.offset();
                    match map
                        .get((r + dr) as usize)
                        .and_then(|row| row.get((c + dc) as usize))
                        .unwrap_or(&b' ')
                    {
                        b'.' => (r, c) = (r + dr, c + dc),
                        b'#' => break,
                        b' ' => {
                            let (new_r, new_c, new_dir) = wrap(map, r, c, dir);
                            if map[new_r as usize][new_c as usize] == b'#' {
                                break;
                            }
                            (r, c, dir) = (new_r, new_c, new_dir);
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }
    }

    (1000 * (r + 1) + 4 * (c + 1) + dir as isize) as usize
}

pub fn prob1(inp: &str) -> usize {
    fn wrap(map: &[Vec<u8>], mut r: isize, mut c: isize, dir: Dir) -> (isize, isize, Dir) {
        let (dr, dc) = dir.offset();
        let (mut new_r, mut new_c) = (r - dr, c - dc);
        while *map
            .get(new_r as usize)
            .and_then(|row| row.get(new_c as usize))
            .unwrap_or(&b' ')
            != b' '
        {
            (r, c) = (new_r, new_c);
            (new_r, new_c) = (r - dr, c - dc);
        }
        (r, c, dir)
    }
    let (map, moves) = parse_input(inp);
    walk(&map, moves, wrap)
}

fn _prob2<const CELL_WIDTH: isize>(inp: &str) -> usize {
    fn wrap<const CELL_WIDTH: isize>(
        _: &[Vec<u8>],
        r: isize,
        c: isize,
        dir: Dir,
    ) -> (isize, isize, Dir) {
        let cw_m1 = CELL_WIDTH - 1;
        let (big_row, big_col, new_dir) = match CELL_WIDTH {
            50 => match (r / CELL_WIDTH, c / CELL_WIDTH, dir) {
                (0, 1, U) => (3, 0, R),
                (0, 1, L) => (2, 0, R),
                (0, 2, U) => (3, 0, U),
                (0, 2, R) => (2, 1, L),
                (0, 2, D) => (1, 1, L),
                (1, 1, R) => (0, 2, U),
                (1, 1, L) => (2, 0, D),
                (2, 0, U) => (1, 1, R),
                (2, 0, L) => (0, 1, R),
                (2, 1, R) => (0, 2, L),
                (2, 1, D) => (3, 0, L),
                (3, 0, R) => (2, 1, U),
                (3, 0, D) => (0, 2, D),
                (3, 0, L) => (0, 1, D),
                _ => unreachable!(),
            },
            4 => match (r / 4, c / 4, dir) {
                (0, 2, U) => (1, 0, D),
                (0, 2, R) => (2, 3, L),
                (0, 2, L) => (1, 1, D),
                (1, 0, U) => (0, 2, D),
                (1, 0, D) => (2, 2, U),
                (1, 0, L) => (2, 3, U),
                (1, 1, U) => (0, 2, R),
                (1, 1, D) => (2, 2, R),
                (1, 2, R) => (2, 3, D),
                (2, 2, L) => (1, 1, U),
                (2, 2, D) => (1, 0, U),
                (2, 3, U) => (1, 2, L),
                (2, 3, R) => (0, 2, L),
                (2, 3, D) => (1, 0, R),
                _ => unreachable!(),
            },
            _ => todo!(),
        };
        let (offset_r, offset_c) = (r % CELL_WIDTH, c % CELL_WIDTH);
        let i = match dir {
            R => offset_r,
            D => cw_m1 - offset_c,
            L => cw_m1 - offset_r,
            U => offset_c,
        };
        let (new_r, new_c) = match new_dir {
            R => (i, 0),
            D => (0, cw_m1 - i),
            L => (cw_m1 - i, cw_m1),
            U => (cw_m1, i),
        };
        (
            big_row * CELL_WIDTH + new_r,
            big_col * CELL_WIDTH + new_c,
            new_dir,
        )
    }

    let (map, moves) = parse_input(inp);
    walk(&map, moves, wrap::<CELL_WIDTH>)
}
pub fn prob2(inp: &str) -> usize {
    _prob2::<50>(inp)
}

#[cfg(test)]
mod tests {
    use super::{_prob2, prob1};
    use std::fs;

    #[test]
    fn part_1_example() {
        let inp = &fs::read_to_string("inputs/task22/example.txt").unwrap();
        let inp = inp.strip_suffix('\n').unwrap_or(&inp);
        assert_eq!(prob1(inp), 6032,);
    }

    #[test]
    fn part_2_example() {
        let inp = &fs::read_to_string("inputs/task22/example.txt").unwrap();
        let inp = inp.strip_suffix('\n').unwrap_or(&inp);
        assert_eq!(_prob2::<4>(inp), 5031,);
    }
}
