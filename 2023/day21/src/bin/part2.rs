use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn bfs(map: &Vec<Vec<char>>, start_x: usize, start_y: usize, steps: usize) -> usize {
    let n = map.len();
    let m = map[0].len();
    let mut plots: HashSet<(usize, usize)> = HashSet::new();
    plots.insert((start_x, start_y));
    let dir = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

    for _ in 0..steps {
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

    plots.len()
}

fn solution(map: Vec<Vec<char>>, start_x: usize, start_y: usize) {
    // Map size: 131x131, starting point: (65, 65)
    let n = map.len();
    let m = map[0].len();
    assert_eq!(n, m);
    assert_eq!((start_x, start_y), (65, 65));

    // The map has a very nice property:
    // 1. The map is a square.
    // 2. Row 65 and column 65 are all dots.
    // 3. All edges are dots.
    for i in 0..n {
        assert_eq!(map[start_x][i], '.');
        assert_eq!(map[i][start_y], '.');

        assert_eq!(map[0][i], '.');
        assert_eq!(map[n - 1][i], '.');
        assert_eq!(map[i][0], '.');
        assert_eq!(map[i][m - 1], '.');
    }

    // This means the shortest path for the ELF to reach an expanded layout is
    // to first reach the edge point on (1) row 65, (2) column 65, or (3) one
    // of the four corners.

    let tot_steps = 26501365 as usize;
    // Walking horizontally or vertically, the farthest plot that the ELF can
    // reach happens to be on the edge.
    // 26501365 = 202300 * 131 + 65
    assert_eq!(tot_steps % n, 65);
    // The farthest points that the ELF can reach look like
    //                         ..x..
    //                         x   x
    //                     ..x........x..
    //                     x   .   .    x
    //                 ..x................x..
    //                 x   .   .   .    .   x
    //             ..x........................x..
    //             x   .   .   . S .    .   .   x
    //             ..x........................x..
    //                 x   .   .   .    .   x
    //                 ..x................x..
    //                     x   .   .    x
    //                     ..x........x..
    //                         x   x
    //                         ..x..

    // The inner part contains
    let repeats: usize = tot_steps / n;
    let num_plots_per_map_odd = bfs(&map, 65, 65, 201); // A large enough odd number.
    let num_plots_per_map_even = bfs(&map, 65, 65, 200); // A large enough even number.
    println!(
        "Number of reachable plots per map: even {}, odd {}",
        num_plots_per_map_even, num_plots_per_map_odd
    );
    let num_inner_plots = num_plots_per_map_even * (repeats - 1) * repeats
        + num_plots_per_map_odd * (repeats - 1) * (repeats - 2)
        + num_plots_per_map_odd * (repeats - 1)
        + num_plots_per_map_even * repeats;
    println!("Number of inner plots: {}", num_inner_plots);

    // Most of the outer parts can be paired. But we don't have to do the math
    // here. We can just brute force it.
    let mut num_outer_plots = 0;

    // 4 corners.
    num_outer_plots += bfs(&map, start_x, m - 1, 130);
    num_outer_plots += bfs(&map, start_x, 0, 130);
    num_outer_plots += bfs(&map, n - 1, start_y, 130);
    num_outer_plots += bfs(&map, 0, start_y, 130);

    // Triangles on the edges.
    num_outer_plots += bfs(&map, n - 1, 0, 64) * repeats;
    num_outer_plots += bfs(&map, n - 1, m - 1, 64) * repeats;
    num_outer_plots += bfs(&map, 0, 0, 64) * repeats;
    num_outer_plots += bfs(&map, 0, m - 1, 64) * repeats;

    // The rest on the edges.
    num_outer_plots += bfs(&map, n - 1, 0, 131 + 64) * (repeats - 1);
    num_outer_plots += bfs(&map, n - 1, m - 1, 131 + 64) * (repeats - 1);
    num_outer_plots += bfs(&map, 0, 0, 131 + 64) * (repeats - 1);
    num_outer_plots += bfs(&map, 0, m - 1, 131 + 64) * (repeats - 1);

    println!("Number of outer plots: {}", num_outer_plots);
    println!(
        "Total number of plots: {}",
        num_inner_plots + num_outer_plots
    );
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
