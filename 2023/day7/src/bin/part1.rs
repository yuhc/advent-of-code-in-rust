use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use scanf::sscanf;
use std::option::Option;
use std::cmp::Ordering;
use once_cell::sync::Lazy;
use std::collections::HashMap;

static CARD_TO_RANK: Lazy<HashMap<char, u32>> = Lazy::new(|| {
    HashMap::<char, u32>::from([
        ('A', 0),
        ('K', 1),
        ('Q', 2),
        ('J', 3),
        ('T', 4),
        ('9', 5),
        ('8', 6),
        ('7', 7),
        ('6', 8),
        ('5', 9),
        ('4', 10),
        ('3', 11),
        ('2', 12),
    ])
});

#[derive(Debug)]
struct Hand {
    hand: String,
    unsorted_hand: String,
    bid: u32,
}

impl Hand {
    fn new(hand: String, bid: u32) -> Self {
        // Sort cards in a hand.
        // !!! No need to sort actually because it only compares the unsorted hands of the same rank.
        let mut sorted_chars: Vec<char> = hand.chars().collect();
        sorted_chars.sort_by(|a, b| {
            let rank_a = CARD_TO_RANK.get(&a).unwrap();
            let rank_b = CARD_TO_RANK.get(&b).unwrap();
            rank_a.cmp(&rank_b)
        });
        let sorted_hand = sorted_chars.into_iter().collect();
        Self {
            hand: sorted_hand,
            unsorted_hand: hand,
            bid: bid,
        }
    }
}

fn get_type(hand: &String) -> u32 {
    let occurances: HashMap<char, u32> = hand
        .chars()
        .fold(HashMap::<char, u32>::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });

    // Five of a kind.
    if occurances.len() == 1 {
        return 0;
    }
    if occurances.len() == 2 {
        // Four of a kind.
        if occurances.values().any(|v| v == &4) {
            return 1;
        }
        // Full house.
        if occurances.values().any(|v| v == &3) {
            return 2;
        }
    }
    if occurances.len() == 3 {
        // Three of a kind.
        if occurances.values().any(|v| v == &3) {
            return 3;
        }
        // Two pair.
        if occurances.values().any(|v| v == &1) {
            return 4;
        }
    }
    // One pair.
    if occurances.len() == 4 {
        return 5;
    }
    // High card.
    6
}

fn solution(mut hands: Vec<Hand>) {
    hands.sort_by(|a, b| {
        let type_a = get_type(&a.hand);
        let type_b = get_type(&b.hand);

        if type_a != type_b {
            return type_a.cmp(&type_b);
        }

        let mut order = Ordering::Equal;
        a.unsorted_hand.chars().zip(b.unsorted_hand.chars()).for_each(|(a, b)| {
            let rank_a = CARD_TO_RANK.get(&a).unwrap();
            let rank_b = CARD_TO_RANK.get(&b).unwrap();
            if rank_a != rank_b && order == Ordering::Equal {
                order = rank_a.cmp(&rank_b);
            }
        });
        order
    });

    let mut tot: u64 = 0;
    let max_rank = hands.len();
    for (i, hand) in hands.iter().enumerate() {
        tot += (max_rank - i) as u64 * hand.bid as u64;
    }
    println!("{}", tot);
}

fn main() {
    let input_file = File::open("input.txt").expect("File not found");
    let reader = BufReader::new(input_file);
    let mut hands = Vec::<Hand>::new();
    let mut hand = String::new();
    let mut bid = 0;

    for line in reader.lines() {
        let _ = sscanf!(&line.unwrap(), "{} {}", hand, bid);
        hands.push(Hand::new(hand.clone(), bid));
    }

    solution(hands);
}
