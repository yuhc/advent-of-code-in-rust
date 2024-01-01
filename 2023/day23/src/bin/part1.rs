use std::fs::File;
use std::io::{BufRead, BufReader};

fn dfs(
    map: &Vec<Vec<char>>,
    dirs: &Vec<(isize, isize)>,
    non_blockers: &Vec<char>,
    end_x: usize,
    end_y: usize,
    start_x: usize,
    start_y: usize,
) -> usize {
    let n = map.len();
    let m = map[0].len();
    let mut best = 0;

    let mut visited: Vec<Vec<bool>> = vec![vec![false; m]; n];
    visited[start_x][start_y] = true;
    let mut stack: Vec<(usize, usize, usize, Vec<Vec<bool>>)> = Vec::new(); // x, y, cost, visited
    stack.push((start_x, start_y, 0, visited));

    while stack.len() > 0 {
        let (x, y, cost, visited) = stack.pop().unwrap();
        let mut visited = visited.clone();
        if (x, y) == (end_x, end_y) {
            if cost > best {
                best = cost;
            }
            continue;
        }

        for (i, (dx, dy)) in dirs.iter().enumerate() {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx < 0 || nx >= n as isize || ny < 0 || ny >= m as isize {
                continue;
            }
            let mut nx = nx as usize;
            let mut ny = ny as usize;
            let mut new_cost = cost + 1;

            if map[nx][ny] == non_blockers[i] {
                let mut nnx = nx as isize + dx;
                let mut nny = ny as isize + dy;
                while map[nnx as usize][nny as usize] == '.' {
                    new_cost += 1;
                    nx = nnx as usize;
                    ny = nny as usize;
                    nnx = nx as isize + dx;
                    nny = ny as isize + dy;
                }
            } else if map[nx][ny] != '.' {
                continue;
            }

            if !visited[nx][ny] {
                visited[x][y] = true;
                stack.push((nx, ny, new_cost, visited.clone()));
            }
        }
    }

    best
}

fn main() {
    let input_file = File::open("input.txt").expect("File not found");
    let reader = BufReader::new(input_file);

    let mut map: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        map.push(line.chars().collect::<Vec<char>>());
    }

    let n = map.len();
    let m = map[0].len();
    let start_x = 0;
    let start_y = map[0].iter().position(|&c| c == '.').unwrap();
    let end_x = n - 1;
    let end_y = map[n - 1].iter().position(|&c| c == '.').unwrap();

    let dirs: Vec<(isize, isize)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    let non_blockers: Vec<char> = vec!['>', 'v', '<', '^'];

    let best = dfs(&map, &dirs, &non_blockers, end_x, end_y, start_x, start_y);
    println!("{}", best);
}
