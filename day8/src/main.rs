fn main() {
    let contents = include_str!("input.txt");

    let mut rows_cols: Vec<Vec<i32>> = vec![];
    let lines = contents.lines();

    lines.for_each(|line| {
        rows_cols.push(
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect(),
        );
    });

    let mut total_trees = 0;
    let mut highest_score = 0;

    let (mut i, mut j) = (0, 0);

    let s = p2(1, 1, &rows_cols);
    println!("test: {}", s);

    while i < rows_cols.len() {
        while j < rows_cols[i].len() {
            if is_shown(i, j, &rows_cols) {
                total_trees += 1;
            }
            let view = p2(i, j, &rows_cols);
            if view > highest_score {
                highest_score = view;
                println!("HIGHEST FOUND {i}, {j}, height {}", rows_cols[i][j]);
            }
            j += 1;
        }
        i += 1;
        j = 0;
    }
    // println!("{:?}", rows_cols);
    println!("Total trees: {}", total_trees);
    println!("Highest view: {}", highest_score);
}

fn is_shown(col: usize, row: usize, rows_cols: &Vec<Vec<i32>>) -> bool {
    let mut i = 0;
    let mut j = 0;
    let mut highest = -1;
    let current_height = rows_cols[col][row];

    while i < rows_cols.len() {
        if i == col {
            if highest < current_height {
                return true;
            }
            highest = -1;
        } else {
            if rows_cols[i][row] > highest {
                highest = rows_cols[i][row];
            }
        }
        i += 1;
    }
    if highest < current_height {
        return true;
    }
    highest = -1;

    while j < rows_cols[col].len() {
        if j == row {
            if highest < current_height {
                return true;
            }
            highest = -1;
        } else {
            if rows_cols[col][j] > highest {
                highest = rows_cols[col][j];
            }
        }
        j += 1;
    }
    if highest < current_height {
        return true;
    }

    false
}

fn p2(col: usize, row: usize, rows_cols: &Vec<Vec<i32>>) -> i32 {
    let mut i = col as i32 + 1;
    let mut j = row as i32 + 1;
    let height = rows_cols[col][row];
    let mut view = 0;
    let mut score = 1;

    while i < rows_cols.len() as i32 && rows_cols[i as usize][row] < height {
        view += 1;
        i += 1;
    }
    if i < rows_cols.len() as i32 {
        view += 1;
    }
    score *= view;
    view = 0;
    i = col as i32 - 1;

    while i > 0 && rows_cols[i as usize][row] < height {
        view += 1;
        i -= 1;
    }
    if i > -1 {
        view += 1;
    }
    score *= view;
    view = 0;

    while j < rows_cols[col].len() as i32 && rows_cols[col][j as usize] < height {
        view += 1;
        j += 1;
    }
    if j < rows_cols[col].len() as i32 {
        view += 1;
    }
    j = row as i32 - 1;
    score *= view;
    view = 0;

    while j >= 0 && rows_cols[col][j as usize] < height {
        view += 1;
        j -= 1;
    }
    if j > -1 {
        view += 1;
    }
    score *= view;

    score
}
