use std::{clone, collections::HashMap, iter::Map};

#[derive(Debug)]
enum Tile {
    Start(u32),
    End(u32),
    Between(u32),
}

impl Tile {
    fn new(c: char) -> Self {
        if c == 'S' {
            return Tile::Start(1);
        }
        if c == 'E' {
            return Tile::End(9);
        }
        Tile::Between(c as u32 - 97)
    }
}

struct Coord {
    x: usize,
    y: usize,
}

impl Clone for Coord {
    fn clone(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
        }
    }
}

impl Coord {
    fn new(x: usize, y: usize) -> Self {
        Coord { x, y }
    }

    fn distance(&self, other: &Self) -> f32 {
        let x = (self.x as f32 - other.x as f32).powf(2.0);
        let y = (self.y as f32 - other.y as f32).powf(2.0);
        (x + y).sqrt()
    }
}

fn main() {}

fn p1() {
    let grid = parse();
    let (start, end) = find_start_end(&grid);
    let grid_memory = HashMap::<Coord, (Coord, u32)>::new();
}

fn p1_recursion(
    curr_pos: Coord,
    dest_pos: &Coord,
    grid: &Vec<Vec<Tile>>,
    movement: u32,
    grid_memory: HashMap<Coord, (Coord, u32)>
) -> Option<u32> {
   let current_height = get
}

fn get_pos(grid: &Vec<Vec<Tile>>, dest_pos: &Coord) -> Option<u32> {
    if dest_pos.x < 0 || dest_pos.x >= grid.len() || dest_pos.y < 0 || dest_pos.y >= grid[0].len() {
        return None;
    }

    Some(grid[dest_pos.x][dest_pos.y])
}

fn parse() -> Vec<Vec<Tile>> {
    let contents = include_str!("input.txt");
    contents
        .lines()
        .map(|l| l.chars().map(|c| Tile::new(c)).collect())
        .collect()
}

fn find_start_end(grid: &Vec<Vec<Tile>>) -> (Coord, Coord) {
    let mut start = Coord::new(0, 0);
    let mut end = Coord::new(0, 0);
    for (x, tiles) in grid.iter().enumerate() {
        for (y, tile) in tiles.iter().enumerate() {
            match tile {
                Tile::Start(_) => start = Coord::new(x, y),
                Tile::End(_) => end = Coord::new(x, y),
                Tile::Between(_) => {}
            }
        }
    }
    (start, end)
}
