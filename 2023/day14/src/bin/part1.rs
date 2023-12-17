use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::option::Option;
use std::vec;
use core::cmp::{max, min};

fn solution(graph: Vec<Vec<char>>) {
    let n = graph.len();
    let m = graph[0].len();
    let mut tot = 0;

    // Scan by columns
    for j in 0..m as i32 {
        let mut cube_row: i32 = -1;
        let mut stacked_rounds = 0;
        let mut col_tot = 0;

        for i in 0..n as i32 {
            if graph[i as usize][j as usize] == '#' {                
                cube_row = i;
                stacked_rounds = 0;
            }
            else if graph[i as usize][j as usize] == 'O' {
                let round_row = cube_row + 1 + stacked_rounds;
                col_tot += n - round_row as usize;
                stacked_rounds += 1;
            }
        }

        tot += col_tot;
    }

    println!("Total: {}", tot);
}

fn main() {
    let input_file = File::open("input.txt").expect("File not found");
    let reader = BufReader::new(input_file);

    let mut graph: Vec<Vec<char>> = vec![];
    for line in reader.lines() {
        graph.push(line.unwrap().chars().collect());
    }

    solution(graph);
}
