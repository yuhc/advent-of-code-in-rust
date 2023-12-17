use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::cmp::min;
use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq)]
struct StarPoint(usize, usize);

fn check_star(matrix: &Vec<Vec<char>>,
        num_rows: usize, num_cols: usize,
        i: usize, start_col: usize, end_col: usize,
        value: u32,
        star_to_nums: &mut HashMap<StarPoint, Vec<u32>>
    ) {
    // Top
    if i > 0 {
        let start_j = if start_col == 0 { 0 } else { start_col - 1 };
        for j in start_j..=min(end_col+1, num_cols-1) {
            if matrix[i-1][j] == '*' {
                star_to_nums.get_mut(&StarPoint(i-1, j)).unwrap().push(value);
            }
        }
    }

    // Bottom
    if i < num_rows - 1 {
        let start_j = if start_col == 0 { 0 } else { start_col - 1 };
        for j in start_j..=min(end_col+1, num_cols-1) {
            if matrix[i+1][j] == '*' {
                star_to_nums.get_mut(&StarPoint(i+1, j)).unwrap().push(value);
            }
        }
    }

    // Left
    if start_col > 0 {
        if matrix[i][start_col-1] == '*' {
            star_to_nums.get_mut(&StarPoint(i, start_col-1)).unwrap().push(value);
        }
    }

    // Right
    if end_col < num_cols - 1 {
        if matrix[i][end_col+1] == '*' {
            star_to_nums.get_mut(&StarPoint(i, end_col+1)).unwrap().push(value);
        }
    }
}

fn solution(matrix: &Vec<Vec<char>>) {
    let num_rows = matrix.len();
    let num_cols = matrix[0].len();
    let mut star_to_nums: HashMap<StarPoint, Vec<u32>> = HashMap::new();
    let mut tot = 0;

    // Init star map.
    for i in 0..num_rows {
        for j in 0..num_cols {
            if matrix[i][j] == '*' {
                star_to_nums.insert(StarPoint(i, j), vec![]);
            }
        }
    }

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

            check_star(matrix, num_rows, num_cols, i, start_col, end_col, value, &mut star_to_nums);
        }
    }

    // Check star map.
    for (_, v) in star_to_nums.iter() {
        if v.len() == 2 {
            println!("{:?}", v);
            tot += v[0] * v[1];
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
