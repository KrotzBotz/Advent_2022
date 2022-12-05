use std::fs;

const FILE_PATH: &str = "./src/input.txt";
fn main() {
    let contents = fs::read_to_string(FILE_PATH).unwrap();
    solve(&contents);
}

fn solve(c: &str) {
    let lines = c.split("\r\n").map(|line| {
        let mut zones = line.split(",");
        let (zone1, zone2): (Vec<i32>, Vec<i32>) = (
            zones
                .next()
                .unwrap()
                .split("-")
                .map(|l| l.parse().unwrap())
                .collect(),
            zones
                .next()
                .unwrap()
                .split("-")
                .map(|l| l.parse().unwrap())
                .collect(),
        );
        (in_zone(&zone1, &zone2), over_lap(&zone1, &zone2))
    });

    let (mut p1, mut p2) = (0, 0);
    for (x, y) in lines {
        p1 += x;
        p2 += y;
    }

    println!("part1: {}, part2: {}", p1, p2);
}

fn in_zone(z1: &Vec<i32>, z2: &Vec<i32>) -> i32 {
    if z1[0] <= z2[0] && z1[1] >= z2[1] {
        return 1;
    }
    if z2[0] <= z1[0] && z2[1] >= z1[1] {
        return 1;
    }
    0
}

fn over_lap(z1: &Vec<i32>, z2: &Vec<i32>) -> i32 {
    if z1[0] > z2[0] {
        return over_lap(&z2, &z1);
    }
    if z1[1] >= z2[0] {
        return 1;
    }
    0
}
