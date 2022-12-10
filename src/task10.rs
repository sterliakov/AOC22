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

trait Cycle {
    fn tick(&mut self);
    fn next(&mut self, inc: i32);

    fn solve(&mut self) {
        loop {
            let inp: String;
            scan!("{}\n", inp);
            if inp.is_empty() {
                break;
            }

            self.tick();
            self.next(0);

            match Command::parse(&inp) {
                Command::Nop => {}
                Command::Addx(inc) => {
                    self.tick();
                    self.next(inc);
                }
            }
        }
    }
}

#[derive(Debug)]
struct Cycle1 {
    cycle: i32,
    x: i32,
    total: i32,
}
#[derive(Debug)]
struct Cycle2 {
    cycle: i32,
    x: i32,
}

impl Cycle1 {
    const TARGETS: [i32; 6] = [20, 60, 100, 140, 180, 220];
}

impl Cycle for Cycle1 {
    fn tick(&mut self) {
        if Self::TARGETS.contains(&self.cycle) {
            self.total += self.cycle * self.x;
        }
    }
    fn next(&mut self, inc: i32) {
        self.cycle += 1;
        self.x += inc;
    }
}
impl Cycle for Cycle2 {
    fn tick(&mut self) {
        print!(
            "{}",
            if (self.x - self.cycle % 40 + 1).abs() <= 1 {
                '#'
            } else {
                '.'
            }
        );
        if self.cycle % 40 == 0 {
            println!();
        }
    }
    fn next(&mut self, inc: i32) {
        self.cycle += 1;
        self.x += inc;
    }
}

#[allow(dead_code)]
pub fn prob1() {
    let mut cycle = Cycle1 {
        cycle: 1,
        x: 1,
        total: 0,
    };
    cycle.solve();
    println!("{}", cycle.total);
}

#[allow(dead_code)]
pub fn prob2() {
    let mut cycle = Cycle2 { cycle: 1, x: 1 };
    cycle.solve();
}
