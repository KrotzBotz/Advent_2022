use std::fs;

fn main() {
    let file_path = "./src/input.txt";

    let contents = fs::read_to_string(file_path).expect("idc");

    let mut elves = vec![];

    contents.split("\r\n\r\n").for_each(|lines| {
        let mut total = 0;
        lines.split("\n").for_each(|line| {
            let line = line.trim();
            total += line.parse::<i32>().unwrap();
        });
        elves.push(total);
    });

    elves.sort_by(|a, b| b.cmp(a));

    let top_three = elves[0] + elves[1] + elves[2];

    println!("{:?}", top_three);
}
