use std::fs;

const FILE_PATH: &str = "./src/input.txt";

type Bag = Vec<char>;
fn main() {
    let contents = fs::read_to_string(FILE_PATH).unwrap();
    let lines: Vec<Bag> = contents
        .split("\r\n")
        .map(|line| line.chars().collect())
        .collect();

    let mut prio = 0;
    let mut i = 0;
    while i < lines.len() {
        prio += priority(&find_badge(&lines[i], &lines[i + 1], &lines[i + 2]));
        i += 3;
    }
    println!("{}", prio);
}

fn find_badge(bag1: &Bag, bag2: &Bag, bag3: &Bag) -> char {
    for c in bag1 {
        if bag2.contains(c) && bag3.contains(c) {
            println!("Found: {}", c);
            return c.clone();
        }
    }
    'n'
}

fn priority(c: &char) -> i32 {
    match *c as i32 {
        65..=90 => *c as i32 - 38,
        97..=122 => *c as i32 - 96,
        _ => 0,
    }
}
