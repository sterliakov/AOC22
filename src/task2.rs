use text_io::scan;

fn choice_score(x: char) -> Result<u32, &'static str> {
    match x {
        'A' | 'X' => Ok(1),
        'B' | 'Y' => Ok(2),
        'C' | 'Z' => Ok(3),
        _ => Err("Unknown!"),
    }
}

fn winning_score(first: char, second: char) -> Result<u32, &'static str> {
    match (first, second) {
        ('C', 'X') | ('A', 'Y') | ('B', 'Z') => Ok(6),
        ('A', 'X') | ('B', 'Y') | ('C', 'Z') => Ok(3),
        ('B', 'X') | ('C', 'Y') | ('A', 'Z') => Ok(0),
        _ => Err("Unknown"),
    }
}

fn get_choice(first: char, goal: char) -> Result<char, &'static str> {
    match (first, goal) {
        ('C', 'X') | ('B', 'Y') | ('A', 'Z') => Ok('Y'),
        ('A', 'X') | ('C', 'Y') | ('B', 'Z') => Ok('Z'),
        ('B', 'X') | ('A', 'Y') | ('C', 'Z') => Ok('X'),
        _ => Err("Unknown"),
    }
}

fn solve(mutate: &mut dyn FnMut(&mut char, &mut char)) -> u32 {
    let mut total: u32 = 0;
    loop {
        let inp: String;
        scan!("{}\n", inp);
        if !inp.is_empty() {
            let (mut t1, mut t2): (char, char);
            scan!(inp.bytes() => "{} {}", t1, t2);
            mutate(&mut t1, &mut t2);
            total += winning_score(t1, t2).expect("Unknown combination")
                + choice_score(t2).expect("Unknown letter");
        } else {
            break total;
        }
    }
}

#[allow(dead_code)]
pub fn prob1() {
    fn noop(_t1: &mut char, _t2: &mut char) {}
    let total = solve(&mut noop);
    println!("{total}")
}

#[allow(dead_code)]
pub fn prob2() {
    fn to_choice(t1: &mut char, t2: &mut char) {
        *t2 = get_choice(*t1, *t2).expect("Cannot detect winner");
    }
    let total = solve(&mut to_choice);
    println!("{total}")
}
