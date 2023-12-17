use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;

fn compute_hash(seq: &str) -> u32 {
    let mut hash = 0;
    for step in seq.chars() {
        hash += step as u32;
        hash *= 17;
        hash %= 256;
    }
    return hash;
}

#[derive(Debug)]
struct Lens(u32, u32); // focal, index

fn solution(sequence: Vec<&str>) {
    let mut boxes: Vec<HashMap<&str, Lens>> = vec![];
    for _i in 0..256 {
        boxes.push(HashMap::new());
    }
    let mut box_capacities: Vec<u32> = vec![1; 256];
    let mut tot = 0;

    for seq in sequence {
        let label: &str;

        if seq.chars().last().unwrap() == '-' {
            label = &seq[0..seq.len()-1];
            let box_id = compute_hash(label);

            let lens_set = &mut boxes[box_id as usize];
            lens_set.remove(label);
        }
        else {
            let sp = seq.split_once('=').unwrap();
            label = sp.0;
            let focal = sp.1.parse::<u32>().unwrap();
            let box_id = compute_hash(label);

            let lens_set = &mut boxes[box_id as usize];
            match lens_set.get_mut(label) {
                Some(lens) => {
                    let id = lens.1;
                    *lens = Lens(focal, id);
                },
                None => {
                    lens_set.insert(label, Lens(focal, box_capacities[box_id as usize]));
                    box_capacities[box_id as usize] += 1;
                }
            }
        }
    }

    // println!("{:?}", boxes);

    for (box_id, lens_set) in boxes.iter().enumerate() {
        let mut lens_list = lens_set.values().collect::<Vec<&Lens>>();
        lens_list.sort_by(|a, b| a.1.cmp(&b.1));
        let mut lens_slot = 0;
        let mut lens_power = 0;
        for lens in lens_list.iter() {
            lens_slot += 1;
            lens_power += (1 + box_id) * lens_slot * lens.0 as usize;
        }
        tot += lens_power;
        println!("{} {}", box_id, lens_power);
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
