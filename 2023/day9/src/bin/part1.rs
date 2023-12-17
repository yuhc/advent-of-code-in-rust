use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
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

fn solution(numbers: Vec<i32>) -> i32 {
    let mut sequences: Vec<Vec<i32>> = vec![];
    let mut sub: Vec<i32> = numbers;

    while !sub.is_empty() && sub.iter().any(|x| x != &0) {
        sequences.push(sub.clone());
        let len = sub.len();
        sub = sub[0..len - 1].into_iter().zip(sub[1..len].into_iter())
            .map(|(a, b)| b - a)
            .collect::<Vec<i32>>();
    }

    // println!("{:?}", sequences);
    let mut next = 0;
    for sub in sequences.iter() {
        next += *sub.last().unwrap();
    }

    next
}

fn main() {
    let input_file = File::open("input.txt").expect("File not found");
    let reader = BufReader::new(input_file);

    let mut tot = 0;
    for line in reader.lines() {
        let numbers: Vec<i32> = line.unwrap().split(' ').map(|c| c.parse::<i32>().unwrap()).collect();
        let next = solution(numbers);
        tot += next;
    }

    println!("{}", tot);
}
