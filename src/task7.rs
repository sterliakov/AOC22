use std::collections::HashMap;
use text_io::scan;

#[derive(Debug)]
enum Command {
    LS,
    CD(String),
}

impl Command {
    fn parse(line: &str) -> Option<Command> {
        let mut words_iter = line.split_whitespace();
        if words_iter.next().unwrap() != "$" {
            return None;
        }
        match words_iter.next().unwrap() {
            "ls" => Some(Command::LS),
            "cd" => Some(Command::CD(words_iter.next().unwrap().to_string())),
            _ => panic!("Unknown command"),
        }
    }
}

#[derive(Debug)]
enum Out {
    File(u32),
    Dir,
}

impl Out {
    fn parse(line: &str) -> Out {
        let file_size = line.split_whitespace().next().unwrap();
        if file_size == "dir" {
            Out::Dir
        } else {
            Out::File(file_size.parse::<u32>().expect("File size not a number"))
        }
    }
}

fn relax(stack: &[String], dir_sizes: &mut HashMap<String, u32>, curr_size: u32) {
    let mut path = "".to_string();
    for fld in stack.iter() {
        if path.len() > 1 {
            // Not "" or "/"
            path.push('/');
        }
        path.push_str(fld);
        if !dir_sizes.contains_key(&path) {
            dir_sizes.insert(path.clone(), curr_size);
        } else {
            *dir_sizes.get_mut(&path).unwrap() += curr_size;
        }
    }
}

fn solve() -> HashMap<String, u32> {
    let mut dir_sizes: HashMap<String, u32> = HashMap::new();
    let mut stack: Vec<String> = Vec::new();
    let mut curr_size: u32 = 0;

    loop {
        let inp: String;
        scan!("{}\n", inp);

        if inp.is_empty() {
            break;
        }
        match Command::parse(&inp) {
            Some(Command::CD(dir)) => {
                relax(&stack, &mut dir_sizes, curr_size);
                curr_size = 0;
                if dir == ".." {
                    stack.pop();
                } else {
                    stack.push(dir);
                }
            }
            Some(Command::LS) => {}
            None => match Out::parse(&inp) {
                Out::Dir => {}
                Out::File(size) => {
                    curr_size += size;
                }
            },
        }
    }

    relax(&stack, &mut dir_sizes, curr_size);
    dir_sizes
}

#[allow(dead_code)]
pub fn prob1() {
    let dir_sizes = solve();
    let ans: u32 = dir_sizes.values().filter(|&&v| v < 100000).sum();
    println!("{dir_sizes:#?}");
    println!("{ans}");
}

#[allow(dead_code)]
pub fn prob2() {
    let dir_sizes = solve();
    let to_clean = 30_000_000 + dir_sizes["/"] - 70_000_000;
    let ans = *dir_sizes.values().filter(|&&v| v > to_clean).min().unwrap();
    println!("{ans}");
}
