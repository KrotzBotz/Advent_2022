use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
fn main() {
    let s = include_str!("input.txt");
    let (stacks, instructions) = s.split_once("\r\n\r\n").unwrap();
    let stacks = stacks.lines();
    let mut stacks = stacks.rev();
    stacks.next();

    let mut sks = HashMap::<i32, Vec<char>>::new();

    for s in stacks {
        for i in 0..9 {
            let c = s.chars().nth((i * 4) + 1).unwrap();
            if c != ' ' {
                match sks.entry(i as i32 + 1) {
                    Occupied(mut s) => {
                        s.get_mut().push(c);
                    }
                    Vacant(s) => {
                        s.insert(vec![c]);
                    }
                }
            }
        }
    }

    let mut zkz = sks.clone();

    instructions.lines().for_each(|l| {
        let m = l.split(" ").collect::<Vec<&str>>();
        let amount = m[1].parse::<i32>().unwrap();
        let from = m[3].parse::<i32>().unwrap();
        let to = m[5].parse::<i32>().unwrap();
        move_item(from, to, amount, &mut sks);
        move_items(from, to, amount, &mut zkz);
    });

    let mut ans = vec!['n'; 9];
    for (i, n) in zkz {
        ans[(i - 1) as usize] = n.last().unwrap().clone();
    }
    println!("{:?}", ans);
}

fn move_item(v1: i32, v2: i32, mut amount: i32, sks: &mut HashMap<i32, Vec<char>>) {
    while let Some(x) = sks.get_mut(&v1).unwrap().pop() {
        sks.get_mut(&v2).unwrap().push(x);
        amount -= 1;
        if amount < 1 {
            break;
        }
    }
}

fn move_items(v1: i32, v2: i32, amount: i32, sks: &mut HashMap<i32, Vec<char>>) {
    let l1 = sks.get(&v1).unwrap().len();
    if amount < l1 as i32 {
        let mut l = sks
            .get_mut(&v1)
            .unwrap()
            .split_off(l1.saturating_sub(amount as usize));
        sks.get_mut(&v2).unwrap().append(&mut l);
    } else {
        let mut l = sks.get_mut(&v1).unwrap().split_off(0);
        sks.get_mut(&v2).unwrap().append(&mut l);
    }
}
