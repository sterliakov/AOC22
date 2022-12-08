use std::vec::Vec;
use text_io::scan;

fn parse_input() -> Vec<Vec<u8>> {
    let mut matrix: Vec<Vec<u8>> = Vec::new();
    loop {
        let inp: String;
        scan!("{}\n", inp);
        if !inp.is_empty() {
            let row = inp.chars().map(|c| c.to_digit(10).unwrap() as u8).collect();
            matrix.push(row);
        } else {
            break matrix;
        }
    }
}

#[allow(dead_code)]
pub fn prob1() {
    let matrix = parse_input();
    let h = matrix.len();
    let w = matrix[0].len();

    let mut visible = vec![vec![0u32; matrix[0].len()]; matrix.len()];

    let mut solve_horizontal = |it: &mut dyn Iterator<Item = (usize, &u8)>, i: usize| {
        let (_, mut curr_max) = it.next().expect("Should be at least 1 character");
        visible[i][0] = 1;
        visible[i][w - 1] = 1;
        for (j, elem) in it {
            if curr_max < elem {
                curr_max = elem;
                visible[i][j] = 1;
            }
        }
    };
    for (i, row) in matrix.iter().enumerate() {
        solve_horizontal(&mut row.iter().enumerate(), i);
        solve_horizontal(&mut row.iter().enumerate().rev(), i);
    }

    let mut iters: Vec<_> = matrix.into_iter().map(|n| n.into_iter()).collect();
    let transposed = (0..w).map(|_| {
        iters
            .iter_mut()
            .map(|n| n.next().unwrap())
            .collect::<Vec<u8>>()
    });

    let mut solve_vertical = |it: &mut dyn Iterator<Item = (usize, &u8)>, i: usize| {
        let (_, mut curr_max) = it.next().expect("Should be at least 1 character");
        visible[0][i] = 1;
        visible[h - 1][i] = 1;
        for (j, elem) in it {
            if curr_max < elem {
                curr_max = elem;
                visible[j][i] = 1;
            }
        }
    };

    for (i, row) in transposed.enumerate() {
        solve_vertical(&mut row.iter().enumerate(), i);
        solve_vertical(&mut row.iter().enumerate().rev(), i);
    }

    let ans = visible.iter().fold(0, |acc, x| acc + x.iter().sum::<u32>());
    println!("{ans}");
}

fn score(matrix: &[Vec<u8>], idx: (usize, usize)) -> usize {
    let h = matrix.len();
    let w = matrix[0].len();
    let (i0, j0) = idx;
    let curr = matrix[i0][j0];

    let to_top = i0 - (0..i0).rev().find(|&i| matrix[i][j0] >= curr).unwrap_or(0);
    let to_bottom = (i0 + 1..h)
        .find(|&i| matrix[i][j0] >= curr)
        .unwrap_or(h - 1)
        - i0;
    let to_left = j0 - (0..j0).rev().find(|&j| matrix[i0][j] >= curr).unwrap_or(0);
    let to_right = (j0 + 1..w)
        .find(|&j| matrix[i0][j] >= curr)
        .unwrap_or(w - 1)
        - j0;
    to_left * to_right * to_top * to_bottom
}

#[allow(dead_code)]
pub fn prob2() {
    let matrix = parse_input();
    let h = matrix.len();
    let w = matrix[0].len();

    let ans = (0..h)
        .map(|i| {
            (0..w)
                .map(|j| score(&matrix, (i, j)))
                .max()
                .expect("Non-empty")
        })
        .max()
        .expect("Non-empty");
    println!("{ans}");
}
