use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use scanf::sscanf;
use std::collections::HashMap;
use num::integer::lcm;

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

impl Node {
    fn new(left: String, right: String) -> Self {
        Self {left, right}
    }
}

fn solution(instruction: String, nodes: HashMap<String, Node>) {
    // Get all nodes that start with "A".
    let mut currents: Vec<String> = nodes.keys()
        .filter(|k| k.ends_with("A"))
        .map(|k| k.clone())
        .collect();
    println!("{:?}", currents);

    let mut tots: Vec<Vec<u64>> = vec![vec![]];
    let mut i = 0;
    for current in &mut currents {
        let mut tot = 0;
        tots.push(vec![]);
        
        'outer: loop {
            for d in instruction.chars() {                
                let next = nodes.get(current).unwrap();
                match d {
                    'L' => *current = next.left.clone(),
                    'R' => *current = next.right.clone(),
                    _ => panic!("Invalid instruction \"{}\"", d),
                }
                tot += 1;

                if current.ends_with('Z') {
                    tots[i].push(tot);
                    if tots[i].len() == 5 {
                        break 'outer;
                    }
                }
            }
        }

        i += 1;
    }

    println!("Solution: {:?}", tots);

    /**
     * The loops are well designed:
     *      [11567, 23134, 34701, 46268, 57835],
     *      [19637, 39274, 58911, 78548, 98185],
     *      [21251, 42502, 63753, 85004, 106255],
     *      [14257, 28514, 42771, 57028, 71285],
     *      [16409, 32818, 49227, 65636, 82045],
     *      [18023, 36046, 54069, 72092, 90115]
     **/
    // LCM
    let mut ans = tots[0][0];
    for i in 1..tots.len() - 1 {
        ans = lcm(tots[i][0], ans);
    }
    println!("Solution: {}", ans);

}

fn main() {
    let input_file = File::open("input.txt").expect("File not found");
    let mut reader = BufReader::new(input_file);
    let mut instruction: String = String::new();
    let mut empty_line: String = String::new();
    let mut nodes = HashMap::<String, Node>::new();
    let mut name = String::new();
    let mut left: String = String::new();
    let mut right: String = String::new();

    let _ = reader.read_line(&mut instruction);
    let _ = reader.read_line(&mut empty_line);

    for line in reader.lines() {
        let _ = sscanf!(&line.unwrap(), "{} = ({}, {})", name, left, right);
        nodes.insert(name.clone(), Node::new(left.clone(), right.clone()));
    }

    solution(instruction.trim().to_string(), nodes);
}
