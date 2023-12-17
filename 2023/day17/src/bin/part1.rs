use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::{BinaryHeap, HashMap};

#[derive(PartialEq, Copy, Clone, Eq, Hash, Debug)]
enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl Direction {
    pub fn iter() -> impl Iterator<Item = Direction> {
        [Direction::Up, Direction::Right, Direction::Down, Direction::Left].iter().copied()
    }
}

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
struct Node {
    x: usize,
    y: usize,
    direction: Direction,
}

#[derive(PartialEq, Eq)]
struct State {
    node: Node,
    cost: u32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.cost.cmp(&self.cost))
    }
}

fn solution(graph: Vec<Vec<u32>>) {
    let n = graph.len();
    let m = graph[0].len();

    // Dijkstra's algorithm. Each node is a tuple of (x, y, direction).
    let mut distances: HashMap<Node, u32> = HashMap::new();
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    distances.insert(Node { x: 0, y: 0, direction: Direction::Down }, 0);
    distances.insert(Node { x: 0, y: 0, direction: Direction::Right }, 0);
    distances.insert(Node { x: 0, y: 0, direction: Direction::Up }, 0);
    distances.insert(Node { x: 0, y: 0, direction: Direction::Left }, 0);
    heap.push(State {node: Node { x: 0, y: 0, direction: Direction::Down }, cost: 0});
    heap.push(State {node: Node { x: 0, y: 0, direction: Direction::Right }, cost: 0});
    heap.push(State {node: Node { x: 0, y: 0, direction: Direction::Up }, cost: 0});
    heap.push(State {node: Node { x: 0, y: 0, direction: Direction::Left }, cost: 0});

    let delta: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    while let Some(state) = heap.pop() {
        let cost = state.cost;
        let node = state.node;
        // println!("Visiting {:?} with cost {}", node, cost);

        if node.x == n - 1 && node.y == m - 1 {
            println!("Shortest path to {:?}: {}", node, cost);
            break;
        }

        let d = distances.get(&node);
        if d.is_some_and(|d| *d < cost) {
            // Already had a better distance.
            continue;
        }

        // Move forward.
        for dir in Direction::iter() {
            if dir == node.direction || dir == (match node.direction {
                Direction::Up => Direction::Down,
                Direction::Down => Direction::Up,
                Direction::Left => Direction::Right,
                Direction::Right => Direction::Left,
            }) {
                continue;
            }

            let mut new_x = node.x as i32;
            let mut new_y = node.y as i32;
            let mut new_cost = cost;
            for _step in 1..=3 {
                new_x += delta[dir as usize].0;
                new_y += delta[dir as usize].1;
                // Boundary check.
                if new_x < 0 || new_x >= n as i32 || new_y < 0 || new_y >= m as i32 {
                    break;
                }
                new_cost += graph[new_x as usize][new_y as usize];

                // Distance check.
                let new_node = Node { x: new_x as usize, y: new_y as usize, direction: dir };
                let new_d = distances.get(&new_node);
                if new_d.is_some_and(|d| *d > new_cost) || new_d.is_none() {
                    distances.insert(new_node, new_cost);                    
                    heap.push(State {node: new_node, cost: new_cost});
                }
            }
        }
    }
}

fn main() {
    let input_file = File::open("input.txt").expect("File not found");
    let reader = BufReader::new(input_file);
    
    let mut graph: Vec<Vec<u32>> = Vec::new();
    for line in reader.lines() {
        let line_int: Vec<u32> = line.unwrap().chars()
            .map(|x| (x.to_string()).parse::<u32>().unwrap()).collect();
        graph.push(line_int);
    }

    println!("{:?}", graph);
    solution(graph);
}
