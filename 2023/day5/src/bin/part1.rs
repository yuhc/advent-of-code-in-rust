use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use scanf::sscanf;
use std::option::Option;

fn solution(reader: &mut BufReader<File>, from_values: Vec<u64>) -> Vec<u64> {
    let mut to_values: Vec<Option<u64>> = vec![None; from_values.len()];

    // Skip the first line.
    let mut line = String::new();
    let _ = reader.read_line(&mut line);

    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }

        let mut source_start = 0;
        let mut dest_start = 0;
        let mut range = 0;
        sscanf!(line.as_str(), "{} {} {}", dest_start, source_start, range);

        // Translate all from_values.
        for i in 0..(from_values.len()) {
            if to_values[i].is_some() {
                continue;
            }

            let from_v = from_values[i];
            if from_v >= source_start && from_v < source_start + range {
                to_values[i] = Some(from_v - source_start + dest_start);
            }
        }
    }

    for i in 0..(from_values.len()) {
        if to_values[i].is_none() {
            to_values[i] = Some(from_values[i]);
        }
    }

    // println!("{:?}", to_values);
    to_values.into_iter().map(|x| x.unwrap()).collect()
}

fn main() {
    let input_file = File::open("input.txt").expect("File not found");
    let mut reader = BufReader::new(input_file);

    // Read seeds.
    let mut line = String::new();
    let mut seeds_string = String::new();
    let _ = reader.read_line(&mut line);
    sscanf!(line.as_str(), "seeds: {}", seeds_string);
    let seeds: Vec<u64> = seeds_string.as_str().split(' ')
        .map(|s| s.trim().parse::<u64>().unwrap()).collect();
    let _ = reader.read_line(&mut line);

    let soils = solution(&mut reader, seeds);
    let fertilizer = solution(&mut reader, soils);
    let water = solution(&mut reader, fertilizer);
    let light = solution(&mut reader, water);
    let temperature = solution(&mut reader, light);
    let humidity = solution(&mut reader, temperature);
    let location = solution(&mut reader, humidity);

    println!("{:?}", location);
    println!("{}", location.iter().min().unwrap());
}
