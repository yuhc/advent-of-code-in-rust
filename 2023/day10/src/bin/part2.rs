use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::option::Option;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::vec;

#[derive(Debug)]
struct Node {
    x: usize,
    y: usize,
    step: u32,
    direction: char,
}

impl Node {
    fn new(x: usize, y: usize, step: u32, direction: char) -> Self {
        Node {
            x,
            y,
            step,
            direction,
        }
    }
    
}

fn dfs(graph: &Vec<Vec<char>>, s_x: usize, s_y: usize, s_dir: char) -> u32 {
    let n = graph.len();
    let m = graph[0].len();
    let mut visited: Vec<Vec<bool>> = vec![vec![false; m]; n];
    let mut queue: Vec<Node> = vec![];
    queue.push(Node::new(s_x, s_y, 0, s_dir));
    visited[s_x][s_y] = true;
    let mut idx = 0;
    let mut tot_steps: u32 = 0;
    let mut last_dir: char = 'X';

    while idx < queue.len() {
        let node = &queue[idx];
        idx += 1;
        let direction = node.direction;

        let mut nx: i32;
        let mut ny: i32;
        let mut ndir: char;
        match direction {
            'N' => {
                nx = node.x as i32 - 1;
                ny = node.y as i32;
                if nx < 0 || ny < 0 || nx >= n as i32 || ny >= m as i32 {
                    continue;
                }
                if graph[nx as usize][ny as usize] == 'S' {
                    tot_steps = node.step + 1;
                    last_dir = node.direction;
                    break;
                }
                if visited[nx as usize][ny as usize] {
                    continue;
                }

                let npile = graph[nx as usize][ny as usize];
                match npile {
                    '|' => { ndir = 'N'; },
                    '7' => { ndir = 'W'; },
                    'F' => { ndir = 'E'; },
                    _ => continue,
                }
            },
            'S' => {
                nx = node.x as i32 + 1;
                ny = node.y as i32;
                if nx < 0 || ny < 0 || nx >= n as i32 || ny >= m as i32 {
                    continue;
                }
                if graph[nx as usize][ny as usize] == 'S' {
                    tot_steps = node.step + 1;
                    last_dir = node.direction;
                    break;
                }
                if visited[nx as usize][ny as usize] {
                    continue;
                }

                let npile = graph[nx as usize][ny as usize];
                match npile {
                    '|' => { ndir = 'S'; },
                    'L' => { ndir = 'E'; },
                    'J' => { ndir = 'W'; },
                    _ => continue,
                }
            },
            'W' => {
                nx = node.x as i32;
                ny = node.y as i32 - 1;
                if nx < 0 || ny < 0 || nx >= n as i32 || ny >= m as i32 {
                    continue;
                }
                if graph[nx as usize][ny as usize] == 'S' {
                    tot_steps = node.step + 1;
                    last_dir = node.direction;
                    break;
                }
                if visited[nx as usize][ny as usize] {
                    continue;
                }

                let npile = graph[nx as usize][ny as usize];
                match npile {
                    '-' => { ndir = 'W'; },
                    'F' => { ndir = 'S'; },
                    'L' => { ndir = 'N'; },
                    _ => continue,
                }
            },
            'E' => {
                nx = node.x as i32;
                ny = node.y as i32 + 1;
                if nx < 0 || ny < 0 || nx >= n as i32 || ny >= m as i32 {
                    continue;
                }
                if graph[nx as usize][ny as usize] == 'S' {
                    tot_steps = node.step + 1;
                    last_dir = node.direction;
                    break;
                }
                if visited[nx as usize][ny as usize] {
                    continue;
                }

                let npile = graph[nx as usize][ny as usize];
                match npile {
                    '-' => { ndir = 'E'; },
                    '7' => { ndir = 'S'; },
                    'J' => { ndir = 'N'; },
                    _ => continue,
                }
            },
            _ => panic!("Invalid direction"),
        }

        visited[nx as usize][ny as usize] = true;
        queue.push(Node::new(nx as usize, ny as usize, node.step + 1, ndir));
    }

    if tot_steps > 0 {
        // Get S symbol.
        let mut s_sym =
            match s_dir {
                'N' => {
                    match last_dir {
                        'N' => '|',
                        'W' => 'L',
                        'E' => 'J',
                        _ => panic!("Invalid direction"),
                    }
                },
                'S' => {
                    match last_dir {
                        'S' => '|',
                        'W' => 'F',
                        'E' => '7',
                        _ => panic!("Invalid direction"),
                    }
                },
                'W' => {
                    match last_dir {
                        'N' => '7',
                        'S' => 'J',
                        'W' => '-',
                        _ => panic!("Invalid direction"),
                    }
                },
                'E' => {
                    match last_dir {
                        'N' => 'F',
                        'S' => 'L',
                        'E' => '-',
                        _ => panic!("Invalid direction"),
                    }                    
                },
                _ => panic!("Invalid direction"),
            };

        let mut tot_inside = 0;
        let mut enclose: Vec<Vec<char>> = vec![vec!['.'; m]; n]; // Debug
        for i in 0..n {
            for j in 0..m {
                if !visited[i][j] {
                    // Check how many vertices are crossed from the right-hand side.
                    let mut count = 0;
                    for k in j+1..m {
                        if visited[i][k] &&
                            (graph[i][k] == '|' || graph[i][k] == 'F' || graph[i][k] == '7' ||
                                (graph[i][k] == 'S' && (s_sym == '|' || s_sym == 'F' || s_sym == '7'))) {
                            count += 1;
                        }
                    }

                    if count % 2 == 1 {
                        enclose[i][j] = 'I';
                        tot_inside += 1;
                    }
                }
            }
        }
        
        println!("Total insides: {}", tot_inside);
        // println!("Enclose: {:?}", enclose);
    }

    tot_steps
}

fn solution(graph: Vec<Vec<char>>, s_x: usize, s_y: usize) {
    println!("Starting point: ({}, {})", s_x, s_y);
    // Explore in 4 directions.
    let steps = dfs(&graph, s_x, s_y, 'N');
    println!("Steps: N => {}", steps / 2);
    let steps = dfs(&graph, s_x, s_y, 'S');
    println!("Steps: S => {}", steps / 2);
    let steps = dfs(&graph, s_x, s_y, 'W');
    println!("Steps: W => {}", steps / 2);
    let steps = dfs(&graph, s_x, s_y, 'E');
    println!("Steps: E => {}", steps / 2);

}

fn main() {
    let input_file = File::open("input.txt").expect("File not found");
    let reader = BufReader::new(input_file);

    let mut graph: Vec<Vec<char>> = vec![];
    let mut s_x: usize = 0;
    let mut s_y: usize = 0;
    let mut i: usize = 0;
    for line in reader.lines() {
        let row: Vec<char> = line.unwrap().chars().collect();
        // Find starting point.
        let lookup_s = row.iter().position(|&c| c == 'S');
        match lookup_s {
            Some(idx) => {
                s_x = i;
                s_y = idx;
            },
            None => ()
        };

        graph.push(row);
        i += 1;
    }

    solution(graph, s_x, s_y);
}
