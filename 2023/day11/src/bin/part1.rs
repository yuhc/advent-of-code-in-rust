use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::option::Option;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::vec;
use core::cmp::{max, min};

fn solution(glaciers: Vec<(usize, usize)>, expended_rows: Vec<bool>, expended_cols: Vec<bool>) {
    let n = expended_rows.len();
    let m = expended_cols.len();
    // Pre-cache number of expanded rows to the left (or cols to the top).
    let mut num_expanded_left: Vec<usize> = vec![0; m];
    if expended_cols[0] {
        num_expanded_left[0] = 1;
    }
    for i in 1..m {
        num_expanded_left[i] = num_expanded_left[i - 1];
        if expended_cols[i] {
            num_expanded_left[i] += 1;
        }
    }
    let mut num_expanded_top: Vec<usize> = vec![0; n];
    if expended_rows[0] {
        num_expanded_top[0] = 1;
    }
    for i in 1..n {
        num_expanded_top[i] = num_expanded_top[i - 1];
        if expended_rows[i] {
            num_expanded_top[i] += 1;
        }
    }

    println!("Num expanded left: {:?}", num_expanded_left);
    println!("Num expanded top: {:?}", num_expanded_top);

    let mut total: usize = 0;
    let n = glaciers.len();
    for i in 0..n {
        for j in i+1..n {
            let (x1, y1) = glaciers[i];
            let (x2, y2) = glaciers[j];
            let num_expanded_rows = num_expanded_top[max(x1, x2)] - num_expanded_top[min(x1, x2)];
            let num_expanded_cols = num_expanded_left[max(y1, y2)] - num_expanded_left[min(y1, y2)];
            total += max(x1, x2) - min(x1, x2) + max(y1, y2) - min(y1, y2)
                + num_expanded_rows + num_expanded_cols;
        }
    }
    println!("Total: {}", total);
}

fn main() {
    let input_file = File::open("input.txt").expect("File not found");
    let reader = BufReader::new(input_file);

    let mut graph: Vec<Vec<char>> = vec![];
    let mut glaciers: Vec<(usize, usize)> = vec![];
    let mut i: usize = 0;
    for line in reader.lines() {
        let row: Vec<char> = line.unwrap().chars().collect();
        let g = row.iter().enumerate().filter(|(_, &c)| c == '#').map(|(j, _)| (i, j)).collect::<Vec<(usize, usize)>>();
        glaciers.extend(g.into_iter());
        graph.push(row);
        i += 1;
    }
    println!("Glaciers: {:?}", glaciers);

    let mut expended_rows: Vec<bool> = vec![true; graph.len()];
    let mut expended_cols: Vec<bool> = vec![true; graph[0].len()];
    for g in &glaciers {
        expended_rows[g.0] = false;
        expended_cols[g.1] = false;
    }

    solution(glaciers, expended_rows, expended_cols);
}
