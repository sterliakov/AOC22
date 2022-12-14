use std::collections::HashSet;
use text_io::scan;

enum Direction {
    Left,
    Right,
    Up,
    Down,
}
struct Motion {
    direction: Direction,
    steps: i32,
}
impl Motion {
    fn parse(row: &str) -> Motion {
        let mut words_iter = row.split_whitespace();
        Motion {
            direction: match words_iter.next().unwrap() {
                "L" => Direction::Left,
                "R" => Direction::Right,
                "U" => Direction::Up,
                "D" => Direction::Down,
                _ => panic!("Unknown direction"),
            },
            steps: words_iter
                .next()
                .unwrap()
                .parse()
                .expect("Must be a number"),
        }
    }
}

fn relax(first: (i32, i32), second: (i32, i32)) -> (i32, i32) {
    let wdiff = first.0 - second.0;
    let hdiff = first.1 - second.1;
    if wdiff.abs() + hdiff.abs() > 2 {
        (
            first.0 - (wdiff.abs() - 1) * wdiff.signum(),
            first.1 - (hdiff.abs() - 1) * hdiff.signum(),
        )
    } else if wdiff.abs() > 1 {
        (first.0 - wdiff.signum(), second.1)
    } else if hdiff.abs() > 1 {
        (second.0, first.1 - hdiff.signum())
    } else {
        second
    }
}
fn solve(knots: usize) -> HashSet<(i32, i32)> {
    assert!(knots > 1, "Not empty");
    let mut visited = HashSet::<(i32, i32)>::new();
    let mut rope = vec![(0i32, 0i32); knots];
    visited.insert(*rope.last().unwrap());

    loop {
        let inp: String;
        scan!("{}\n", inp);
        if inp.is_empty() {
            break;
        };
        let motion = Motion::parse(&inp);

        for _ in 0..motion.steps {
            let head = &mut rope[0];
            *head = match motion.direction {
                Direction::Right => (head.0 + 1, head.1),
                Direction::Left => (head.0 - 1, head.1),
                Direction::Up => (head.0, head.1 + 1),
                Direction::Down => (head.0, head.1 - 1),
            };

            for i in 1..knots {
                rope[i] = relax(rope[i - 1], rope[i]);
            }
            visited.insert(*rope.last().unwrap());
        }
    }

    visited
}

#[allow(dead_code)]
pub fn prob1() {
    println!("{}", solve(2).len());
}

#[allow(dead_code)]
pub fn prob2() {
    println!("{}", solve(10).len());
}
