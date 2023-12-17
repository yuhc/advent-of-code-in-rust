use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::cmp;
use std::collections::BinaryHeap;

#[allow(dead_code)]
fn part1() {
    let input_file = File::open("input.txt").expect("File not found");
    let reader = BufReader::new(input_file);

    let mut max_calories: i64 = 0;
    let mut cur_calories: i64 = 0;

    for line in reader.lines() {
        match line.unwrap().parse::<i64>() {
            Ok(n) => {
                cur_calories += n;
            },
            Err(_) => {
                max_calories = cmp::max(cur_calories, max_calories);
                cur_calories = 0;
            },
        }
    }
    max_calories = cmp::max(cur_calories, max_calories);
    println!("{}", max_calories);
}

fn part2() {
    let input_file = File::open("input.txt").expect("File not found");
    let reader = BufReader::new(input_file);

    let mut calories = vec![];
    let mut cur_calories: i64 = 0;

    for line in reader.lines() {
        match line.unwrap().parse::<i64>() {
            Ok(n) => {
                cur_calories += n;
            },
            Err(_) => {
                calories.push(cur_calories);
                cur_calories = 0;
            },
        }
    }
    calories.push(cur_calories);

    let mut heap = calories.iter().copied().collect::<BinaryHeap<_>>();
    let mut top3_calories = 0;
    for _ in 0..3 {
        top3_calories += heap.pop().unwrap();
    }
    println!("{}", top3_calories);
}

fn main() {
    part2();
}
