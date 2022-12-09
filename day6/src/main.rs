use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;

fn main() {
    let contents = include_str!("input.txt");
    let chars = contents.chars().collect::<Vec<char>>();
    let mut sliding_char: HashMap<char, i32> = HashMap::new();

    let mut i = 0;
    while i < chars.len() as i32 {
        let c = chars[i as usize];
        let len = sliding_char.len();
        match sliding_char.entry(c) {
            Occupied(m) => {
                i = m.get() + 1;
                sliding_char.clear();
            }
            Vacant(m) => {
                m.insert(i);
                if len + 1 == 14 {
                    println!("{:?}", sliding_char);
                    break;
                }
                i += 1;
            }
        }
    }

    println!("{}", i + 1);
}
