use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use scanf::sscanf;
use std::collections::HashSet;

fn solution(reader: BufReader<File>) {
    let mut tot = 0;

    for line in reader.lines() {
        let mut card_id = 0;
        let mut winning_str: String = String::new();
        let mut number_str: String = String::new();
        sscanf!(line.unwrap().as_str(), "Card {}: {} | {}",
            card_id, winning_str, number_str);

        // Winning numbers.
        let winning_set: HashSet<u32> = winning_str.as_str().split(' ')
            .filter(|&x| x.len() > 0)
            .map(|x| x.trim().parse::<u32>().unwrap()).collect();
        // My numbers.
        let winning_numbers: Vec<&str> = number_str.as_str().split(' ')
            .filter(|&x| x.len() > 0)
            .filter(|&x| winning_set.contains(&x.trim().parse::<u32>().unwrap())).collect();

        // println!("Winning set: {:?}", winning_numbers);
        if winning_numbers.len() > 0 {
            let base: u32 = 2;
            tot += base.pow((winning_numbers.len() - 1).try_into().unwrap());
        }
    }

    println!("{}", tot);
}

fn main() {
    let input_file = File::open("input.txt").expect("File not found");
    let reader = BufReader::new(input_file);
    solution(reader);
}
