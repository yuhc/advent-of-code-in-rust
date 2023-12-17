use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use scanf::sscanf;
use std::cmp::max;
use std::collections::{HashSet, HashMap};

fn solution(reader: BufReader<File>) {
    let mut tot = 0;
    let mut max_card_id = 1;
    let mut copies: HashMap<u32, i32> = HashMap::new();
    let mut current_copies: i32 = 0;
    copies.insert(1, 1);

    for line in reader.lines() {
        let mut card_id = 0;
        let mut winning_str: String = String::new();
        let mut number_str: String = String::new();
        sscanf!(line.unwrap().as_str(), "Card {}: {} | {}",
            card_id, winning_str, number_str);

        // Adjust number of copies.
        current_copies += copies.get(&card_id).unwrap_or(&0);
        tot += current_copies;

        // Winning numbers.
        let winning_set: HashSet<u32> = winning_str.as_str().split(' ')
            .filter(|&x| x.len() > 0)
            .map(|x| x.trim().parse::<u32>().unwrap()).collect();
        // My numbers.
        let winning_numbers: Vec<&str> = number_str.as_str().split(' ')
            .filter(|&x| x.len() > 0)
            .filter(|&x| winning_set.contains(&x.trim().parse::<u32>().unwrap())).collect();
        let wins = winning_numbers.len() as u32;

        if let Some(x) = copies.get_mut(&(card_id + 1)) {
            *x += current_copies;
        }
        else {
            copies.insert(card_id + 1, current_copies);
        }
        if let Some(x) = copies.get_mut(&(card_id + wins + 1)) {
            *x -= current_copies;
        }
        else {
            copies.insert(card_id + wins + 1, -current_copies);
        }
    }

    println!("{}", tot);
}

fn main() {
    let input_file = File::open("input.txt").expect("File not found");
    let reader = BufReader::new(input_file);
    solution(reader);
}
