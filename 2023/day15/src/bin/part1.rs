use std::fs::File;
use std::io::{BufRead, BufReader};

fn solution(sequence: Vec<&str>) {
    let mut tot = 0;

    for seq in sequence {
        let mut hash = 0;
        for step in seq.chars() {
            hash += step as u32;
            hash *= 17;
            hash %= 256;
        }
        tot += hash;
        println!("{}", hash);
    }

    println!("{}", tot);
}

fn main() {
    let input_file = File::open("input.txt").expect("File not found");
    let reader = BufReader::new(input_file);
    let line: &str = &reader.lines().next().unwrap().unwrap();

    let sequence: Vec<&str> = line.split(',').collect();
    solution(sequence);
}
