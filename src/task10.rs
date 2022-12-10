use text_io::scan;

#[derive(Debug)]
enum Command {
    Addx(i32),
    Nop,
}
impl Command {
    fn parse(line: &str) -> Command {
        let mut words_iter = line.split_whitespace();
        match words_iter.next().unwrap() {
            "noop" => Command::Nop,
            "addx" => Command::Addx(words_iter.next().unwrap().parse().unwrap()),
            _ => panic!("Unknown command"),
        }
    }
}

#[derive(Debug)]
struct Cycle {
    cycle: i32,
    x: i32,
}
impl Cycle {
    fn next(&mut self, inc: i32) {
        self.cycle += 1;
        self.x += inc;
    }
}

const TARGETS: [i32; 6] = [20, 60, 100, 140, 180, 220];

fn solve(tick: &mut dyn FnMut(&mut Cycle)) {
    let mut cycle = Cycle { cycle: 1, x: 1 };
    loop {
        let inp: String;
        scan!("{}\n", inp);
        if inp.is_empty() {
            break;
        }

        tick(&mut cycle);
        cycle.next(0);

        match Command::parse(&inp) {
            Command::Nop => {}
            Command::Addx(inc) => {
                tick(&mut cycle);
                cycle.next(inc);
            }
        }
    }
}

#[allow(dead_code)]
pub fn prob1() {
    let mut total = 0i32;
    let mut tick = |cycle: &mut Cycle| {
        if TARGETS.contains(&cycle.cycle) {
            total += cycle.cycle * cycle.x;
        }
    };
    solve(&mut tick);
    println!("{}", total);
}

#[allow(dead_code)]
pub fn prob2() {
    let mut tick = |cycle: &mut Cycle| {
        if (cycle.x - cycle.cycle % 40 + 1).abs() <= 1 {
            print!("#")
        } else {
            print!(".")
        }

        if cycle.cycle % 40 == 0 {
            println!();
        }
    };
    solve(&mut tick);
}
