struct Screen {
    screen: Vec<String>,
    x: i32, //sprite position
    draw_cycle: i32,
}

impl Screen {
    fn new() -> Self {
        let mut c = vec!["".to_string(); 6];
        c[0] += "#";
        Self {
            screen: c,
            x: 0,
            draw_cycle: 0,
        }
    }
    fn print(self) {
        println!(" -------------ELF------COMS--------------");
        for l in self.screen {
            print!("|");
            print!("{l}");
            print!("|\n");
        }
        println!(" ----------------------------------------");
    }

    fn draw_tick(&mut self) {
        self.draw_cycle += 1;
        let c_pos: usize = (self.draw_cycle / 40) as usize;
        let rel_pos = self.draw_cycle % 40;
        if c_pos >= self.screen.len() {
            return;
        }
        if rel_pos >= self.x && rel_pos <= self.x + 2 {
            self.screen[c_pos] = self.screen[c_pos].to_string() + "#";
        } else {
            self.screen[c_pos] = self.screen[c_pos].to_string() + ".";
        }
    }
}

fn main() {
    let contents = include_str!("input.txt").lines().map(|line| {
        let mut s = line.split(" ");
        (s.next().unwrap().to_string(), s.next())
    });

    let mut cycle = 0;
    let mut last_value = 1;
    let mut x = 0;

    let signal_str = [20, 60, 100, 140, 180, 220];
    let mut sig_inc = 0;

    let mut answer = 0;

    let mut screen = Screen::new();

    for (command, value) in contents {
        x += last_value;
        if command.contains("noop") {
            screen.draw_tick();
            last_value = 0;
            cycle += 1;
        } else {
            screen.draw_tick();
            let value: i32 = value.unwrap().parse().unwrap();
            screen.x += value;
            screen.draw_tick();
            last_value = value;
            cycle += 2;
        }

        if sig_inc < signal_str.len() && cycle >= signal_str[sig_inc] {
            answer += x * signal_str[sig_inc];
            dbg!(x);
            sig_inc += 1;
        }
    }

    screen.print();

    dbg!(answer);
}
