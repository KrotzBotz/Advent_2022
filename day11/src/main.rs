use std::vec;

#[derive(Debug)]
enum DecidingOp {
    Old,
    Val(u64),
}

impl DecidingOp {
    fn new(s: &str) -> Self {
        if s.contains("old") {
            return Self::Old;
        }
        Self::Val(s.parse::<u64>().unwrap())
    }
}

#[derive(Debug)]
struct Monkey_Controller {
    monkeys: Vec<Monkey>,
}

impl Monkey_Controller {
    fn p1(&self) -> u64 {
        let mut insp = vec![];

        for m in &self.monkeys {
            insp.push(m.inspections);
        }

        insp.sort();
        insp.reverse();

        insp[0] * insp[1]
    }

    fn round(&mut self, div: u64) {
        let mut i = 0;
        while i < self.monkeys.len() {
            while !self.monkeys[i].items.is_empty() {
                let pass_to = {
                    let monkey = &mut self.monkeys[i];
                    monkey.inspections += 1;
                    let item = monkey.items[0];

                    let n1 = match monkey.worry_op.0 {
                        DecidingOp::Old => item,
                        DecidingOp::Val(x) => x,
                    };

                    let n2 = match monkey.worry_op.2 {
                        DecidingOp::Old => item,
                        DecidingOp::Val(x) => x,
                    };

                    let mut worry = match monkey.worry_op.1.as_str() {
                        "*" => n1 * n2,
                        "+" => n1 + n2,
                        _ => 0,
                    };

                    worry = worry % div;
                    if worry % monkey.divisible_by == 0 {
                        (monkey.true_monkey, worry)
                    } else {
                        (monkey.false_monkey, worry)
                    }
                };

                self.monkeys[i].items.remove(0);
                self.monkeys[pass_to.0 as usize].items.push(pass_to.1);
            }
            i += 1;
        }
    }
}

#[derive(Debug)]
struct Monkey {
    id: u64,
    inspections: u64,
    items: Vec<u64>,
    worry_op: (DecidingOp, String, DecidingOp),
    divisible_by: u64,
    true_monkey: u64,
    false_monkey: u64,
}

impl Monkey {
    fn print(&self) {
        print!("Monkey {}: ", self.id);
        for i in &self.items {
            print!("{}, ", *i);
        }
        print!("\n");
    }
    fn new(raw: &str) -> Self {
        let mut raw_lines = raw.lines();

        let id: Vec<char> = raw_lines.next().unwrap().chars().collect();
        let id: u64 = id[id.len() - 2].to_digit(16).unwrap().into();

        let (_, starting_items) = raw_lines.next().unwrap().split_once(":").unwrap();
        let items: Vec<u64> = starting_items
            .split(", ")
            .map(|mut s| {
                s = s.trim();
                s.parse::<u64>().unwrap()
            })
            .collect();

        let (_, worry_operation) = raw_lines.next().unwrap().split_once("= ").unwrap();
        let mut worry_operation = worry_operation.split(" ");
        let first = DecidingOp::new(worry_operation.next().unwrap());
        let op = worry_operation.next().unwrap().to_string();
        let second = DecidingOp::new(worry_operation.next().unwrap());

        let divisible_by = raw_lines
            .next()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse::<u64>()
            .unwrap();
        let true_monkey = raw_lines
            .next()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse::<u64>()
            .unwrap();

        let false_monkey = raw_lines
            .next()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse::<u64>()
            .unwrap();

        Monkey {
            id,
            inspections: 0,
            items,
            worry_op: (first, op, second),
            divisible_by,
            true_monkey,
            false_monkey,
        }
    }
}

fn main() {
    let contents = include_str!("input.txt");
    let raw_monkeys = contents.split("\r\n\r\n");
    let mut m = Monkey_Controller { monkeys: vec![] };

    for r in raw_monkeys {
        m.monkeys.push(Monkey::new(r));
    }

    let v: Vec<u64> = m.monkeys.iter().map(|m| m.divisible_by).collect();
    let lcm = lcm(&v);

    // for i in 0..20 {
    //     println!("\n---------------");
    //     println!("Round: {}\n", i + 1);
    //     m.round(3);
    //     for mon in &m.monkeys {
    //         mon.print();
    //     }
    // }

    for _ in 0..10000 {
        m.round(lcm);
    }

    for mon in &m.monkeys {
        println!("Monkey: {}, Inspections: {}", mon.id, mon.inspections);
    }

    dbg!(m.p1());
}

fn lcm(v: &Vec<u64>) -> u64 {
    let mut v_clone = v.clone();
    v_clone.sort();
    v_clone.reverse();
    let max = v_clone[0];
    let second = v_clone[1];

    let mut total = max;

    let mut t = true;

    while t {
        t = false;
        for n in v {
            if total % *n != 0 {
                t = true;
            }
        }
        if t {
            total += max;
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use crate::lcm;

    #[test]
    fn test_lcm() {
        let v = vec![2, 6, 20];
        println!("LCM: {}", lcm(&v));
    }
}
