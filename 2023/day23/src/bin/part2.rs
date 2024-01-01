use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn build_graph(
    map: &Vec<Vec<char>>,
    dirs: &Vec<(isize, isize)>,
) -> HashMap<usize, HashMap<usize, usize>> {
    let n = map.len();
    let m = map[0].len();

    // Node --> { Neighbour --> Cost }
    let mut edges: HashMap<usize, HashMap<usize, usize>> = HashMap::new();

    for i in 0..n {
        for j in 0..m {
            if map[i][j] == '#' {
                continue;
            }
            let id1 = i * m + j;

            for (dx, dy) in dirs {
                let nx = i as isize + dx;
                let ny = j as isize + dy;
                if nx < 0 || nx >= n as isize || ny < 0 || ny >= m as isize {
                    continue;
                }
                let nx = nx as usize;
                let ny = ny as usize;
                if map[nx][ny] == '#' {
                    continue;
                }

                let id2 = nx * m + ny;
                edges.entry(id1).or_insert(HashMap::new()).insert(id2, 1);
                edges.entry(id2).or_insert(HashMap::new()).insert(id1, 1);
            }
        }
    }

    println!("Original graph:\n{:?}", edges);
    edges
}

fn compress_graph(
    edges: &mut HashMap<usize, HashMap<usize, usize>>,
    start_id: usize,
    end_id: usize,
) {
    println!("Compressing graph... {} nodes", edges.len());

    let mut queue: Vec<usize> = Vec::new();
    for (&id, neighbours) in edges.iter() {
        // Start and end nodes only have one neighbour.
        if neighbours.len() <= 2 && id != start_id && id != end_id {
            queue.push(id);
        }
    }

    let mut tot_compressed = 0;

    while queue.len() > 0 {
        let id = queue.pop().unwrap();
        if id == start_id || id == end_id {
            continue;
        }
        if !edges.contains_key(&id) {
            continue;
        }
        tot_compressed += 1;

        // Dead ends should be removed.
        if edges.get(&id).unwrap().len() == 0 {
            // Do nothing.
        } else if edges.get(&id).unwrap().len() == 1 {
            let neighbour = edges.get(&id).unwrap().keys().next().unwrap().clone();
            edges.get_mut(&neighbour).unwrap().remove(&id);

            if edges.get(&neighbour).unwrap().len() <= 2 {
                queue.push(neighbour);
            }
        } else {
            assert!(edges.get(&id).unwrap().len() == 2);
            let neighbour1 = edges.get(&id).unwrap().keys().nth(0).unwrap().clone();
            let neighbour2 = edges.get(&id).unwrap().keys().nth(1).unwrap().clone();
            let new_weight = edges.get(&id).unwrap().get(&neighbour1).unwrap()
                + edges.get(&id).unwrap().get(&neighbour2).unwrap();
            edges.get_mut(&neighbour1).unwrap().remove(&id);
            edges.get_mut(&neighbour2).unwrap().remove(&id);
            edges
                .get_mut(&neighbour1)
                .unwrap()
                .insert(neighbour2, new_weight);
            edges
                .get_mut(&neighbour2)
                .unwrap()
                .insert(neighbour1, new_weight);
        }

        edges.remove(&id);
    }

    println!("Eliminated dead ends: {}", tot_compressed);
    println!("Compressed graph:\n{:?}", edges);
}

fn dfs(edges: &HashMap<usize, HashMap<usize, usize>>, start_id: usize, end_id: usize) -> usize {
    let mut best = 0;
    let mut visited: HashMap<usize, bool> = HashMap::new();
    for (&id, _) in edges.iter() {
        visited.insert(id, false);
    }
    visited.insert(start_id, true);

    let mut stack: Vec<(usize, usize, HashMap<usize, bool>)> = Vec::new(); // id, cost, visited
    stack.push((start_id, 0, visited));

    while stack.len() > 0 {
        let (id, cost, visited) = stack.pop().unwrap();
        let mut visited = visited.clone();
        if id == end_id {
            best = best.max(cost);
            continue;
        }

        for neighbour in edges.get(&id).unwrap().keys() {
            if !visited.get(neighbour).unwrap() {
                let new_cost = cost + edges.get(&id).unwrap().get(neighbour).unwrap();
                // println!(
                //     "{} --> {} (cost = {}, weight = {})",
                //     id,
                //     neighbour,
                //     new_cost,
                //     new_cost - cost
                // );
                visited.insert(*neighbour, true);
                stack.push((*neighbour, new_cost, visited.clone()));
                visited.insert(*neighbour, false);
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
        map.push(
            line.chars()
                .map(|c| if c == '.' || c == '#' { c } else { '.' })
                .collect::<Vec<char>>(),
        );
    }

    let n = map.len();
    let m = map[0].len();
    let start_x = 0;
    let start_y = map[0].iter().position(|&c| c == '.').unwrap();
    let end_x = n - 1;
    let end_y = map[n - 1].iter().position(|&c| c == '.').unwrap();
    let start_id = start_x * m + start_y;
    let end_id = end_x * m + end_y;
    println!("Start ID = {}", start_id);
    println!("End ID = {}", end_id);

    let dirs: Vec<(isize, isize)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut edges = build_graph(&map, &dirs);
    compress_graph(&mut edges, start_id, end_id);

    let best = dfs(&edges, start_id, end_id);
    println!("{}", best);
}
