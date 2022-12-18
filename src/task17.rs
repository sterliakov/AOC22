use hashbrown::HashMap;
use std::vec::Vec;

const WIDTH: usize = 7;
const ROCKS: [&[(usize, usize)]; 5] = [
    &[(0, 3), (0, 0), (0, 1), (0, 2)],
    &[(1, 2), (0, 1), (1, 0), (2, 1)],
    &[(2, 2), (0, 0), (0, 1), (0, 2), (1, 2)],
    &[(0, 0), (1, 0), (2, 0), (3, 0)],
    &[(1, 1), (0, 0), (0, 1), (1, 0)],
];
const EMPTY: [u8; WIDTH] = [0, 0, 0, 0, 0, 0, 0];
const ROTATION: usize = 12;

fn get_height(map: &[[u8; WIDTH]]) -> usize {
    let len = map.len();
    len - map
        .iter()
        .rev()
        .position(|row| row != &EMPTY)
        .unwrap_or(len)
}

fn can_fit(map: &[[u8; WIDTH]], rock: &[(usize, usize)], h: usize, w: usize) -> bool {
    rock.iter()
        .all(|(dh, dw)| w + dw < WIDTH && map[h + dh][w + dw] != b'#')
}

// Vector of distances from last (most recent) # in column to max_height line
fn footrprint(map: &[[u8; WIDTH]], hmax: usize) -> Vec<usize> {
    (0..WIDTH)
        .map(|i| {
            map[..hmax]
                .iter()
                .rev()
                .position(|r| r[i] == b'#')
                .unwrap_or(hmax)
        })
        .collect::<Vec<_>>()
}

pub fn prob1(inp: &str) -> i64 {
    solve::<false>(inp, 2022)
}
pub fn prob2(inp: &str) -> i64 {
    solve::<true>(inp, 1_000_000_000_000)
}

pub fn solve<const T: bool>(inp: &str, steps: i64) -> i64 {
    let mut map = [[0u8; WIDTH]; 50];
    let mut jets = inp.as_bytes().iter().enumerate().cycle();
    let mut rocks = ROCKS.iter().enumerate().cycle();
    let (mut rot, mut inc_height) = (0i64, 0i64);

    let (mut cache, mut skip_cache) = if T {
        (HashMap::with_capacity(2048), false)
    } else {
        (HashMap::with_capacity(0), true)
    };

    let mut i = 0;
    while i < steps {
        let mut jet_index;
        let (rock_index, rock) = rocks.next().unwrap();
        let mut h = get_height(&map);
        let mut w = 2;
        if h > 42 {
            map[..ROTATION].iter_mut().for_each(|r| r.fill(0));
            map.rotate_left(ROTATION);
            rot += 1;
            h -= ROTATION;
        }
        h += 3;
        loop {
            let jet;
            (jet_index, jet) = jets.next().unwrap();
            w = match jet {
                b'>' if can_fit(&map, rock, h, w + 1) => w + 1,
                b'<' if w > 0 && can_fit(&map, rock, h, w - 1) => w - 1,
                b'<' | b'>' => w,
                _ => panic!("Unknown"),
            };
            if h == 0 || !can_fit(&map, rock, h - 1, w) {
                break;
            }
            h -= 1;
        }
        rock.iter().for_each(|(dh, dw)| map[h + dh][w + dw] = b'#');

        if T {
            let hmax = get_height(&map);
            let key = (rock_index, jet_index, footrprint(&map, hmax));
            let hmax = hmax as i64 + rot * ROTATION as i64;
            if skip_cache {
            } else if let Some(&(idx, height)) = cache.get(&key) {
                let repeats = (steps - idx - 1) / (i - idx) - 1;
                i += (i - idx) * repeats;
                inc_height = repeats * (hmax - height);
                skip_cache = true;
                // map.iter().for_each(|row| println!("{}", itertools::Itertools::join(&mut row.iter().map(|x|if*x==b'#'{'#'}else{'.'}), ""))); println!();
            } else {
                cache.insert(key, (i, hmax));
            }
        }
        i += 1;
    }
    get_height(&map) as i64 + inc_height + rot * ROTATION as i64
}

#[cfg(test)]
mod tests {
    use crate::{prob1, prob2};
    use std::fs;

    #[test]
    fn part_1_example() {
        assert_eq!(
            prob1(
                &fs::read_to_string("inputs/task17/example.txt")
                    .unwrap()
                    .strip_suffix('\n')
                    .unwrap()
            ),
            3068,
        );
    }

    #[test]
    fn part_2_example() {
        assert_eq!(
            prob2(
                &fs::read_to_string("inputs/task17/example.txt")
                    .unwrap()
                    .strip_suffix('\n')
                    .unwrap()
            ),
            1514285714288,
        );
    }
}
