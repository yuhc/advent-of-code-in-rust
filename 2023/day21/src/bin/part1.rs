use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn solution(map: Vec<Vec<char>>, start_x: usize, start_y: usize) {
    let n = map.len();
    let m = map[0].len();

    let mut plots: HashSet<(usize, usize)> = HashSet::new();
    plots.insert((start_x, start_y));
    let dir = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

    for _ in 0..64 {
        let mut next_plots: HashSet<(usize, usize)> = HashSet::new();
        for (x, y) in plots {
            for (dx, dy) in &dir {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx < 0 || nx >= n as i32 || ny < 0 || ny >= m as i32 {
                    continue;
                }
                if map[nx as usize][ny as usize] == '#' {
                    continue;
                }
                next_plots.insert((nx as usize, ny as usize));
            }
        }
        plots = next_plots;
    }

    println!("{}", plots.len());
}

fn main() {
    let input_file = File::open("input.txt").expect("File not found");
    let reader = BufReader::new(input_file);
    let lines: Vec<_> = reader.lines().collect();
    let mut iter = lines.iter();

    let mut map: Vec<Vec<char>> = Vec::new();
    let mut start_x = 0;
    let mut start_y = 0;
    while let Some(Ok(line)) = iter.next() {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            if c == 'S' {
                start_x = map.len();
                start_y = row.len();
                row.push('.');
                continue;
            }
            row.push(c);
        }
        map.push(row);
    }

    solution(map, start_x, start_y);
}
