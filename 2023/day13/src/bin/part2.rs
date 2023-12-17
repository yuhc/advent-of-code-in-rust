use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::option::Option;
use std::vec;
use core::cmp::{max, min};

fn solution(graph: &Vec<Vec<char>>, old_score: usize) -> usize {
    let n = graph.len();
    let m = graph[0].len();

    for j in 0..m-1 {
        if j + 1 == old_score {
            continue;
        }
        let mut symmetric = true;
        for i in 0..n {
            let len = min(j + 1, m - j - 1);
            if graph[i][j+1-len..=j].iter().zip(graph[i][j+1..=j+len].iter().rev())
                .any(|(a, b)| a != b) {
                symmetric = false;
                break;
            }
        }

        if symmetric {
            return j + 1;
        }
    }

    0
}

fn transpose<T>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>> where T: Clone {
    assert!(!matrix.is_empty(), "Matrix must be non-empty.");
    let n = matrix.len();
    let m = matrix[0].len();

    (0..m).map(|col| matrix.iter().map(|row| row[col].clone()).collect())
        .collect()
}

fn solution_helper(graph: &Vec<Vec<char>>, old_score: usize) -> usize {
    // Look for vertical reflection.
    let mut score = solution(&graph, old_score % 100);

    if score == 0 {
        // Look for horizontal reflection.
        let transposed = transpose(&graph);
        score = solution(&transposed, old_score / 100) * 100;
    }
    score
}

fn main() {
    let input_file = File::open("input.txt").expect("File not found");
    let reader = BufReader::new(input_file);

    let mut tot = 0;
    let mut graph: Vec<Vec<char>> = vec![];
    for line in reader.lines() {
        let string = line.unwrap();
        if string.is_empty() {
            println!("Graph: {:?}", graph);

            // Look for vertical reflection.
            let mut score = solution_helper(&graph, 0);
            println!("Result: {}", score);

            let n = graph.len();
            let m = graph[0].len();
            let mut new_score = 0;
            'outer: for i in 0..n {
                for j in 0..m {
                    if graph[i][j] == '.' {
                        graph[i][j] = '#';
                        new_score = solution_helper(&graph, score);
                        if new_score != 0 && new_score != score {
                            break 'outer;
                        }
                        graph[i][j] = '.';
                    }
                    else {
                        graph[i][j] = '.';
                        new_score = solution_helper(&graph, score);
                        if new_score != 0 && new_score != score {
                            break 'outer;
                        }
                        graph[i][j] = '#';
                    }
                }
            }
            println!("Result New: {}", new_score);
            tot += new_score;

            graph.clear();
        }
        else {
            graph.push(string.chars().collect());
        }
    }

    println!("{}", tot);
}
