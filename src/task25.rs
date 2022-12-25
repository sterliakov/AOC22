use std::{
    fmt::{self, Display},
    str::FromStr,
};

const BASE: i64 = 5;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
struct Snafu {
    values: Vec<i64>,
}

impl FromStr for Snafu {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut values = Vec::with_capacity(s.len());
        for digit in s.bytes().rev() {
            let num = match digit {
                b'2' => 2,
                b'1' => 1,
                b'0' => 0,
                b'-' => -1,
                b'=' => -2,
                _ => return Err("Unknown digit".to_string()),
            };
            values.push(num);
        }
        Ok(Self { values })
    }
}
impl Display for Snafu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut repr = String::new();
        // Not .map().join() to return Err instead of panic
        for x in self.values.iter().rev() {
            let digit = match x {
                2 => '2',
                1 => '1',
                0 => '0',
                -1 => '-',
                -2 => '=',
                _ => return fmt::Result::Err(fmt::Error {}),
            };
            repr.push(digit);
        }

        write!(f, "{}", repr)
    }
}

impl From<Snafu> for i64 {
    fn from(value: Snafu) -> Self {
        let mut inc = 1;
        value.values.iter().fold(0i64, |acc, digit| {
            let res = acc + inc * digit;
            inc *= BASE;
            res
        })
    }
}
impl From<i64> for Snafu {
    fn from(mut value: i64) -> Self {
        let mut res = Snafu::default();
        while value > 0 {
            let (div, rem) = (value / BASE, value % BASE);
            if rem > 2 {
                value = div + 1;
                res.values.push(rem - 5);
            } else {
                value = div;
                res.values.push(rem);
            }
        }
        res
    }
}

pub fn prob1(inp: &str) -> String {
    Snafu::from(
        inp.lines()
            .map(Snafu::from_str)
            .map(Result::unwrap)
            .map(i64::from)
            .sum::<i64>(),
    )
    .to_string()
}

pub fn prob2(_inp: &str) -> &str {
    panic!("No part 2 today:(")
}

#[cfg(test)]
mod tests {
    use super::prob1;
    use std::fs;

    #[test]
    fn part_1_example() {
        let inp = &fs::read_to_string("inputs/task25/example.txt").unwrap();
        let inp = inp.strip_suffix('\n').unwrap_or(&inp);
        assert_eq!(prob1(inp), "2=-1=0");
    }
}
