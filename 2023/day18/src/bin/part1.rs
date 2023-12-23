use scanf::sscanf;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

#[derive(Debug)]
struct Dig {
    dir: Direction,
    meters: u32,
    color: String,
    color_int: u32,
}

fn bfs(ground: &mut Vec<Vec<u32>>, i: i32, j: i32, n: i32, m: i32) {
    let mut heap: BinaryHeap<(i32, i32)> = BinaryHeap::new();
    heap.push((i, j));
    while let Some((i, j)) = heap.pop() {
        if i < 0 || i >= n || j < 0 || j >= m {
            continue;
        }
        if ground[i as usize][j as usize] != u32::MAX {
            continue;
        }
        ground[i as usize][j as usize] = 0;
        heap.push((i - 1, j));
        heap.push((i + 1, j));
        heap.push((i, j - 1));
        heap.push((i, j + 1));
    }
}

fn solution(digs: Vec<Dig>) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    // Get the max coordinates.
    let mut x_min: i32 = 0;
    let mut y_min: i32 = 0;
    let mut x_max: i32 = 0;
    let mut y_max: i32 = 0;
    for dig in digs.iter() {
        match dig.dir {
            Direction::Up => x -= dig.meters as i32,
            Direction::Right => y += dig.meters as i32,
            Direction::Down => x += dig.meters as i32,
            Direction::Left => y -= dig.meters as i32,
        }
        x_min = x_min.min(x);
        y_min = y_min.min(y);
        x_max = x_max.max(x);
        y_max = y_max.max(y);
    }
    if x != 0 || y != 0 {
        panic!("Did not return to origin");
    }

    let n: i32 = x_max - x_min + 1;
    let m: i32 = y_max - y_min + 1;
    x = -x_min;
    y = -y_min;
    println!("{}x{}, origin {} {}", n, m, x, y);

    // Paint all lagoon edges.
    // We don't have to paint a matrix (instead we can use a HashMap),
    // but this helps with debugging.
    let mut ground: Vec<Vec<u32>> = vec![vec![u32::MAX; m as usize]; n as usize];
    for dig in digs {
        match dig.dir {
            Direction::Up => {
                for _ in 0..dig.meters {
                    x -= 1;
                    ground[x as usize][y as usize] = dig.color_int;
                }
            }
            Direction::Right => {
                for _ in 0..dig.meters {
                    y += 1;
                    ground[x as usize][y as usize] = dig.color_int;
                }
            }
            Direction::Down => {
                for _ in 0..dig.meters {
                    x += 1;
                    ground[x as usize][y as usize] = dig.color_int;
                }
            }
            Direction::Left => {
                for _ in 0..dig.meters {
                    y -= 1;
                    ground[x as usize][y as usize] = dig.color_int;
                }
            }
        }
    }

    // Paint all inner cells. Search from ground edges.
    for i in 0..n {
        if ground[i as usize][0] == u32::MAX {
            bfs(&mut ground, i, 0, n, m);
        }
        if ground[i as usize][(m - 1) as usize] == u32::MAX {
            bfs(&mut ground, i, m - 1, n, m);
        }
    }
    for j in 0..m {
        if ground[0][j as usize] == u32::MAX {
            bfs(&mut ground, 0, j, n, m);
        }
        if ground[(n - 1) as usize][j as usize] == u32::MAX {
            bfs(&mut ground, n - 1, j, n, m);
        }
    }

    let count = ground.iter().flatten().filter(|&&x| x > 0).count();
    println!("Count: {}", count);
}

fn main() {
    let input_file = File::open("input.txt").expect("File not found");
    let reader = BufReader::new(input_file);

    let mut digs: Vec<Dig> = Vec::new();
    for line in reader.lines() {
        let mut dir_s: String = String::new();
        let mut meters: u32 = 0;
        let mut color: String = String::new();
        let _ = sscanf!(&line.unwrap(), "{} {} (#{})", dir_s, meters, color);
        digs.push(Dig {
            dir: match dir_s.as_str() {
                "U" => Direction::Up,
                "R" => Direction::Right,
                "D" => Direction::Down,
                "L" => Direction::Left,
                _ => panic!("Invalid direction"),
            },
            meters: meters,
            color: color.clone(),
            color_int: u32::from_str_radix(color.as_str(), 16).unwrap(),
        });
    }

    solution(digs);
}
