use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Cord {
    x: i32,
    y: i32,
}

impl Cord {
    fn copy(&self) -> Self {
        Cord {
            x: self.x,
            y: self.y,
        }
    }
}

fn main() {
    let contents = include_str!("input.txt");
    let commands = contents.lines().map(|line| {
        let mut l = line.split(" ");
        let command = match l.next().unwrap() {
            "U" => (0, 1),
            "D" => (0, -1),
            "L" => (-1, 0),
            "R" => (1, 0),
            _ => (0, 0),
        };
        let amount: i32 = l.next().unwrap().parse().unwrap();
        (command, amount)
    });

    let mut snake: Vec<Cord> = vec![];
    for _ in 0..10 {
        snake.push(Cord { x: 0, y: 0 });
    }
    let mut unique: HashSet<Cord> = HashSet::new();

    for ((x, y), amount) in commands {
        for _ in 0..amount {
            snake[0].x += x;
            snake[0].y += y;
            let mut i = 1;
            while i < snake.len() {
                catchup(&snake[i - 1].copy(), &mut snake[i]);
                i += 1;
            }
            unique.insert(snake[snake.len() - 1].copy());
        }
    }

    dbg!(unique.len());
}

fn catchup(head: &Cord, tail: &mut Cord) -> Cord {
    let x_diff = (head.x - tail.x).abs();
    let y_diff = (head.y - tail.y).abs();
    if x_diff > 1 && y_diff > 1 {
        if head.x > tail.x {
            tail.x += 1;
        } else {
            tail.x -= 1;
        }
        if head.y > tail.y {
            tail.y += 1;
        } else {
            tail.y -= 1;
        }
        return tail.copy();
    }
    if x_diff > 1 {
        if y_diff != 0 {
            tail.y = head.y;
        }
        if head.x > tail.x {
            tail.x += 1;
        } else {
            tail.x -= 1;
        }
    }
    if y_diff > 1 {
        if x_diff != 0 {
            tail.x = head.x;
        }
        if head.y > tail.y {
            tail.y += 1;
        } else {
            tail.y -= 1;
        }
    }
    tail.copy()
}
