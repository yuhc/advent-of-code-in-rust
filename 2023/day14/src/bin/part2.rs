use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::vec;

fn solution(graph: Vec<Vec<char>>) {
    let mut cache: HashMap<Vec<Vec<char>>, i32> = HashMap::new();

    let n = graph.len();
    let m = graph[0].len();

    let mut graph = graph.clone();
    cache.insert(graph.clone(), 0);

    for k in 0..1_000_000_000 {
        // North. Scan by columns
        for j in 0..m as i32 {
            let mut cube_row: i32 = -1;
            let mut stacked_rounds = 0;

            for i in 0..n as i32 {
                if graph[i as usize][j as usize] == '#' {
                    cube_row = i;
                    stacked_rounds = 0;
                } else if graph[i as usize][j as usize] == 'O' {
                    let round_row = cube_row + 1 + stacked_rounds;
                    graph[i as usize][j as usize] = '.';
                    graph[round_row as usize][j as usize] = 'O';
                    stacked_rounds += 1;
                }
            }
        }

        // West. Scan by rows
        for i in 0..n as i32 {
            let mut cube_col: i32 = -1;
            let mut stacked_rounds = 0;

            for j in 0..m as i32 {
                if graph[i as usize][j as usize] == '#' {
                    cube_col = j;
                    stacked_rounds = 0;
                } else if graph[i as usize][j as usize] == 'O' {
                    let round_col = cube_col + 1 + stacked_rounds;
                    graph[i as usize][j as usize] = '.';
                    graph[i as usize][round_col as usize] = 'O';
                    stacked_rounds += 1;
                }
            }
        }

        // South. Scan by columns
        for j in 0..m as i32 {
            let mut cube_row: i32 = n as i32;
            let mut stacked_rounds = 0;

            for i in (0..n as i32).rev() {
                if graph[i as usize][j as usize] == '#' {
                    cube_row = i;
                    stacked_rounds = 0;
                } else if graph[i as usize][j as usize] == 'O' {
                    let round_row = cube_row - 1 - stacked_rounds;
                    graph[i as usize][j as usize] = '.';
                    graph[round_row as usize][j as usize] = 'O';
                    stacked_rounds += 1;
                }
            }
        }

        // East. Scan by rows
        for i in 0..n as i32 {
            let mut cube_col: i32 = m as i32;
            let mut stacked_rounds = 0;

            for j in (0..m as i32).rev() {
                if graph[i as usize][j as usize] == '#' {
                    cube_col = j;
                    stacked_rounds = 0;
                } else if graph[i as usize][j as usize] == 'O' {
                    let round_col = cube_col - 1 - stacked_rounds;
                    graph[i as usize][j as usize] = '.';
                    graph[i as usize][round_col as usize] = 'O';
                    stacked_rounds += 1;
                }
            }
        }

        let find = cache.get(&graph);
        if find.is_some() {
            let start = *find.unwrap();
            let loop_len = k + 1 - start;
            println!("Loop detected after {} iterations => start {} loop {}", k + 1, start, loop_len);
            graph = cache.iter()
                .find_map(
                    |(key, &v)|
                        if v == (1_000_000_000 - start) % loop_len + start { Some(key) } else { None }
                )
                .unwrap().clone();
            break;
        } else {
            cache.insert(graph.clone(), k + 1);
        }
    }

    let mut tot = 0;
    for i in 0..n {
        for j in 0..m {
            if graph[i][j] == 'O' {
                tot += n - i;
            }
        }
    }
    println!("{:?}", graph);
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
