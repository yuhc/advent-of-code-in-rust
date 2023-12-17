use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

#[allow(dead_code)]
fn part1() {
    let input_file = File::open("input.txt").expect("File not found");
    let reader = BufReader::new(input_file);

    let mut tot = 0;
    for line in reader.lines() {
        let numbers = line.unwrap().chars()
            .filter(|c| c.is_numeric())
            .map(|c| c.to_digit(10))
            .collect::<Vec<_>>();

        if numbers.len() > 0 {
            tot += numbers[0].unwrap() * 10 + numbers[numbers.len() - 1].unwrap();
        }
        // println!("{:?}", numbers);
    }
    println!("{}", tot);
}

fn part2() {
    let input_file = File::open("input.txt").expect("File not found");
    let reader = BufReader::new(input_file);

    let mut tot = 0;
    for line in reader.lines() {
        let numbers = line.unwrap()
            .replace("one", "one1one")
            .replace("two", "two2two")
            .replace("three", "three3three")
            .replace("four", "four4four")
            .replace("five", "five5five")
            .replace("six", "six6six")
            .replace("seven", "seven7seven")
            .replace("eight", "eight8eight")
            .replace("nine", "nine9nine")
            .chars()
            .filter(|c| c.is_numeric())
            .map(|c| c.to_digit(10))
            .collect::<Vec<_>>();

        if numbers.len() > 0 {
            tot += numbers[0].unwrap() * 10 + numbers[numbers.len() - 1].unwrap();
        }
        // println!("{:?}", numbers);
    }
    println!("{}", tot);
}

fn main() {
    part2();
}
