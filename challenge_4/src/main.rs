use std::collections::HashSet;
use std::collections::LinkedList;
use std::env;
use std::fs;
static MATCH: &str = "XMAS";

/**
* Thoughts:
* Linear scanning matricies is good practice for rust.
* DFS could work too.
*
*/
fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let matrix = read_input(file_path);
    display_matrix(matrix.clone());
    // Linear scans
    println!("scan: {}", scan(&matrix));
    println!("mas: {}", mas_scan(&matrix));
}

fn read_input(filepath: &str) -> Vec<Vec<char>> {
    let contents = fs::read_to_string(filepath).expect("THERE'S NO FILE THERE DUMMY");
    let newline_split: Vec<&str> = contents.split("\n").collect();
    let mut lines: Vec<Vec<char>> = vec![];
    for line in newline_split {
        if !line.is_empty() {
            lines.push(line.chars().collect());
        }
    }
    lines
}

fn display_matrix(matrix: Vec<Vec<char>>) {
    for (row_num, row) in matrix.into_iter().enumerate() {
        print!("row number {row_num} ");
        for char in row {
            print!("{char}");
        }
        println!();
    }
}

fn scan(matrix: &Vec<Vec<char>>) -> i32 {
    // find starts and add them to queue
    // (x, y, MATCH idx)
    let mut starts: Vec<(usize, usize, usize)> = Vec::new();
    let directions: Vec<(i32, i32)> = vec![
        (1, 0),
        (0, 1),
        (-1, 0),
        (0, -1),
        (1, 1),
        (-1, 1),
        (1, -1),
        (-1, -1),
    ];

    for (x, row) in matrix.iter().enumerate() {
        for (y, char) in row.iter().enumerate() {
            if *char == 'X' {
                starts.push((x, y, 0));
            }
        }
    }

    let mut total = 0;
    for (x, y, _) in starts {
        for (dx, dy) in &directions {
            let mut nx = x as i32 + *dx;
            let mut ny = y as i32 + *dy;
            let mut idx = 1;

            while nx >= 0 && nx < matrix.len() as i32 && ny >= 0 && ny < matrix[0].len() as i32 {
                if matrix[nx as usize][ny as usize] == MATCH.chars().nth(idx).unwrap() {
                    idx += 1;
                } else {
                    break;
                }

                if idx == 4 {
                    total += 1;
                    break;
                }

                nx += *dx;
                ny += *dy;
            }
        }
    }
    total
}

/**
* Thoughts:
* find the middle point, scan the points diagnally and check against the know configuation for MAS
* Lots of manual coding annoyance, but prob the most straightford
*/
fn mas_scan(matrix: &Vec<Vec<char>>) -> i32 {
    // find starts and add them to queue
    // (x, y, MATCH idx)
    let mut starts: Vec<(usize, usize, usize)> = Vec::new();
    let mas_config: Vec<Vec<(i32, i32, char)>> = vec![
        vec![(1, 1, 'M'), (-1, 1, 'M'), (1, -1, 'S'), (-1, -1, 'S')],
        vec![(1, 1, 'M'), (-1, 1, 'S'), (1, -1, 'M'), (-1, -1, 'S')],
        vec![(1, 1, 'S'), (-1, 1, 'M'), (1, -1, 'S'), (-1, -1, 'M')],
        vec![(1, 1, 'S'), (-1, 1, 'S'), (1, -1, 'M'), (-1, -1, 'M')],
    ];

    for (x, row) in matrix.iter().enumerate() {
        for (y, char) in row.iter().enumerate() {
            if *char == 'A' {
                starts.push((x, y, 0));
            }
        }
    }

    let mut total = 0;
    for (x, y, _) in starts {
        for directions in &mas_config {
            let mut valid = 0;
            for (dx, dy, c) in directions {
                let nx = x as i32 + *dx;
                let ny = y as i32 + *dy;
                if nx >= 0
                    && nx < matrix.len() as i32
                    && ny >= 0
                    && ny < matrix[0].len() as i32
                    && matrix[nx as usize][ny as usize] == *c
                {
                    valid += 1;
                } else {
                    break;
                }
            }
            if valid == 4 {
                total += 1;
            }
        }
    }
    total
}
