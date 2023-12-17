use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use scanf::sscanf;
use std::option::Option;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug)]
struct Node {
    name: String,
    left: String,
    right: String,
}

impl Node {
    fn new(name: String, left: String, right: String) -> Self {
        Self { name, left, right }
    }
}

fn solution(instruction: String, nodes: HashMap<String, Node>) {
    let mut tot = 0;
    let mut current = "AAA".to_string();
    while current != "ZZZ" {
        for d in instruction.chars() {
            let next = nodes.get(&current).unwrap();
            match d {
                'L' => current = next.left.clone(),
                'R' => current = next.right.clone(),
                _ => panic!("Invalid instruction \"{}\"", d),
            }        
            tot += 1;

            if current == "ZZZ" {
                break;
            }
        }
    }

    println!("Solution: {}", tot);
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
        nodes.insert(name.clone(), Node::new(name.clone(), left.clone(), right.clone()));
    }

    println!("Instruction: {}", instruction);
    println!("Nodes: {:?}", nodes);
    solution(instruction.trim().to_string(), nodes);
}
