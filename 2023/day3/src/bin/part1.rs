use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::cmp::min;

fn validate(matrix: &Vec<Vec<char>>,
        num_rows: usize, num_cols: usize,
        i: usize, start_col: usize, end_col: usize) -> bool {
    // Top
    if i > 0 {
        let start_j = if start_col == 0 { 0 } else { start_col - 1 };
        for j in start_j..=min(end_col+1, num_cols-1) {
            if matrix[i-1][j] != '.' {
                return true;
            }
        }
    }

    // Bottom
    if i < num_rows - 1 {
        let start_j = if start_col == 0 { 0 } else { start_col - 1 };
        for j in start_j..=min(end_col+1, num_cols-1) {
            if matrix[i+1][j] != '.' {
                return true;
            }
        }
    }

    // Left
    if start_col > 0 {
        if matrix[i][start_col-1] != '.' {
            return true;
        }
    }

    // Right
    if end_col < num_cols - 1 {
        if matrix[i][end_col+1] != '.' {
            return true;
        }
    }

    false
}

fn solution(matrix: &Vec<Vec<char>>) {
    let num_rows = matrix.len();
    let num_cols = matrix[0].len();
    let mut tot = 0;

    for i in 0..num_rows {
        let mut j = 0;
        while j < num_cols {
            if !matrix[i][j].is_numeric() {
                j += 1;
                continue;
            }

            let mut value = 0;
            let start_col = j;
            while j < num_cols && matrix[i][j].is_numeric() {
                value = value * 10 + matrix[i][j].to_digit(10).unwrap();
                j += 1;
            }
            let end_col = j - 1;

            if validate(matrix, num_rows, num_cols, i, start_col, end_col) {
                tot += value;
                println!("{} {} {} {}", i, start_col, end_col, value);
            }
        }
    }

    println!("{}", tot);
}

fn main() {
    let input_file = File::open("input.txt").expect("File not found");
    let reader = BufReader::new(input_file);
    let matrix: Vec<Vec<char>> = reader.lines().map(|line| line.unwrap().chars().collect()).collect();
    solution(&matrix);
}
