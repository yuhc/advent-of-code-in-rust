use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use scanf::sscanf;
use std::option::Option;

#[derive(Debug, Clone, Copy)]
struct Segment(u64, u64);

fn solution(reader: &mut BufReader<File>, from_values: Vec::<Option<Segment>>) -> Vec::<Option<Segment>> {
    let mut from_segments = from_values.clone();
    let mut to_segments = Vec::<Option<Segment>>::new();

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
        let len = from_segments.len();
        for i in 0..len {
            if from_segments[i].is_none() {
                continue;
            }

            let from_segment = from_segments[i].clone().unwrap();
            // from_segment covers range.
            if from_segment.0 <= source_start && from_segment.0 + from_segment.1 >= source_start + range {
                // First part unchanged.
                if from_segment.0 < source_start {
                    from_segments.push(Some(Segment(from_segment.0, source_start - from_segment.0)));
                }
                // Last part unchanged.
                if from_segment.0 + from_segment.1 > source_start + range {
                    from_segments.push(Some(Segment(source_start + range, from_segment.0 + from_segment.1 - source_start - range)));
                }
                // Mid part remapped.
                to_segments.push(Some(Segment(dest_start, range)));
                from_segments[i] = None;
            } else

            // range covers from_segment.
            if source_start <= from_segment.0 && source_start + range >= from_segment.0 + from_segment.1 {
                // Whole segment changed.
                to_segments.push(Some(Segment(from_segment.0 - source_start + dest_start, from_segment.1)));
                from_segments[i] = None;
            } else

            // from_segments and range intersect, while from_segment is in the front.
            if from_segment.0 < source_start && from_segment.0 + from_segment.1 < source_start + range
                && source_start < from_segment.0 + from_segment.1 {
                // First part unchanged.
                from_segments.push(Some(Segment(from_segment.0, source_start - from_segment.0)));
                // Last part remapped.
                to_segments.push(Some(Segment(dest_start, from_segment.0 + from_segment.1 - source_start)));
                from_segments[i] = None;
            } else

            // from_segments and range intersect, while from_segment is in the back.
            if source_start < from_segment.0 && source_start + range < from_segment.0 + from_segment.1
                && source_start + range > from_segment.0 {
                // First part remapped.
                to_segments.push(Some(Segment(dest_start + from_segment.0 - source_start, source_start + range - from_segment.0)));
                // Last part unchanged.
                from_segments.push(Some(Segment(source_start + range, from_segment.0 + from_segment.1 - source_start - range)));
                from_segments[i] = None;
            }
        }
    }

    for i in 0..from_segments.len() {
        if from_segments[i].is_some() {
            to_segments.push(from_segments[i]);
        }
    }

    to_segments.into_iter().filter(|x| x.is_some()).collect()
}

fn main() {
    let input_file = File::open("input.txt").expect("File not found");
    let mut reader = BufReader::new(input_file);

    // Read seeds.
    let mut line = String::new();
    let mut seeds_string = String::new();
    let _ = reader.read_line(&mut line);
    sscanf!(line.as_str(), "seeds: {}", seeds_string);
    let seeds: Vec::<Option<Segment>> = seeds_string.as_str().split(' ')
        .map(|s| s.trim().parse::<u64>().unwrap()).collect::<Vec<u64>>()
        .chunks(2).map(|t| Some(Segment(t[0], t[1]))).collect();
    let _ = reader.read_line(&mut line);

    let soils = solution(&mut reader, seeds);
    let fertilizer = solution(&mut reader, soils);
    let water = solution(&mut reader, fertilizer);
    let light = solution(&mut reader, water);
    let temperature = solution(&mut reader, light);
    let humidity = solution(&mut reader, temperature);
    let location = solution(&mut reader, humidity);

    println!("{:?}", location);
    let location_starts = location.iter().map(|x| x.unwrap().0).collect::<Vec<u64>>();
    // println!("{:?}", location_starts);
    println!("{}", location_starts.into_iter().min().unwrap());
}
