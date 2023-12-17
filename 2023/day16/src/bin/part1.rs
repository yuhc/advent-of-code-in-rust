use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::vec;

#[derive(PartialEq, Copy, Clone)]
enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

fn move_forward(
    queue: &mut Vec<(usize, usize, Direction)>,
    visited: &mut Vec<Vec<Vec<bool>>>,
    delta: &[(i32, i32); 4],
    x: usize, y: usize, next_dir: Direction) {
    let n = visited.len();
    let m = visited[0].len();

    let next_x = x as i32 + delta[next_dir as usize].0;
    let next_y = y as i32 + delta[next_dir as usize].1;
    
    // Boundary check.
    if next_x < 0 || next_x >= n as i32 || next_y < 0 || next_y >= m as i32 {
        return;
    }

    // Visited check.
    if !visited[next_x as usize][next_y as usize][next_dir as usize] {
        queue.push((next_x as usize, next_y as usize, next_dir));
        visited[next_x as usize][next_y as usize][next_dir as usize] = true;
    }
}

fn solution(graph: Vec<Vec<char>>) {
    let n = graph.len();
    let m = graph[0].len();
    let mut visited = vec![vec![vec![false; 4]; m]; n];

    let delta: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    let mut queue: Vec<(usize, usize, Direction)> = Vec::new();
    queue.push((0, 0, Direction::East));
    visited[0][0][Direction::East as usize] = true;
    let mut i = 0;

    while i < queue.len() {
        let (x, y, dir) = queue[i];
        i += 1;

        // Get new direction.
        match graph[x][y] {
            '.' => {
                // Keep going.
                move_forward(&mut queue, &mut visited, &delta, x, y, dir);
            },
            '|' => {
                if dir == Direction::North || dir == Direction::South {
                    // Keep going.
                    move_forward(&mut queue, &mut visited, &delta, x, y, dir)
                }
                else {
                    // Split direction.
                    move_forward(&mut queue, &mut visited, &delta, x, y, Direction::North);
                    move_forward(&mut queue, &mut visited, &delta, x, y, Direction::South);
                }
            },
            '-' => {
                // Keep going.
                if dir == Direction::East || dir == Direction::West {
                    move_forward(&mut queue, &mut visited, &delta, x, y, dir)
                }
                else {
                    // Split direction.
                    move_forward(&mut queue, &mut visited, &delta, x, y, Direction::East);
                    move_forward(&mut queue, &mut visited, &delta, x, y, Direction::West);
                }
            },
            '/' => {
                // Change direction.
                let next_dir = match dir {
                    Direction::North => Direction::East,
                    Direction::East => Direction::North,
                    Direction::South => Direction::West,
                    Direction::West => Direction::South,
                };
                move_forward(&mut queue, &mut visited, &delta, x, y, next_dir);
            },
            '\\' => {
                // Change direction.
                let next_dir = match dir {
                    Direction::North => Direction::West,
                    Direction::East => Direction::South,
                    Direction::South => Direction::East,
                    Direction::West => Direction::North,
                };
                move_forward(&mut queue, &mut visited, &delta, x, y, next_dir);
            },
            _ => panic!("Invalid input."),
        }
    }

    let mut result = vec![vec!['.'; m]; n];
    let mut tot = 0;
    for i in 0..n {
        for j in 0..m {
            if visited[i][j].iter().any(|x| *x) {
                result[i][j] = '#';
                tot += 1;
            }
        }
    }
    println!("{:?}", result);
    println!("{}", tot);
}

fn main() {
    let input_file = File::open("input.txt").expect("File not found");
    let reader = BufReader::new(input_file);
    
    let mut graph: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        graph.push(line.unwrap().chars().collect());
    }

    solution(graph);
}
