use std::fs;

const FILE_PATH: &str = "./src/input.txt";

fn main() {
    let contents = fs::read_to_string(FILE_PATH).unwrap();
    let lines = contents.split("\r\n").map(|line| {
        let chars: Vec<char> = line.chars().collect();
        let player1 = chars[0].to_digit(36).unwrap() - 10;
        let outcome = chars[2].to_digit(36).unwrap() - 33;

        match outcome {
            1 => 4 + player1,
            2 => 7 + ((player1 + 1) % 3),
            _ => 1 + ((player1 + 2) % 3),
        }
    });

    println!("{}", lines.sum::<u32>());
}
