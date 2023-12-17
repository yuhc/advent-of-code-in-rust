use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use scanf::sscanf;
use std::option::Option;
use std::cmp::max;

fn solution(time: Vec<f32>, distances: Vec<f32>) -> u32 {
    let mut tot = 1;
    let len = distances.len();

    for i in 0..len {
        let T = time[i];
        let D = distances[i];

        // x^2 - Tx + D < 0
        // No solution.
        if T * T < 4.0 * D {
            return 0;
        }
        // One solution.
        else if T * T == 4.0 * D {
            if (T as u32) % 2 == 1 {
                return 0;
            }
        }
        // Two solutions.
        else {
            let mut x1 = ((T - (T*T - 4.0*D).sqrt()) / 2.0).ceil() as u32;
            let mut x2 = ((T + (T*T - 4.0*D).sqrt()) / 2.0).floor() as u32;
            if ((x1 * x1) as f32) - T * (x1 as f32) + D == 0.0 {
                x1 += 1;
            }
            if ((x2 * x2) as f32) - T * (x2 as f32) + D == 0.0 {
                x2 -= 1;
            }

            tot *= max(x2 - x1 + 1, 0);
            println!("{}", x2 - x1 + 1);
        }
    }

    tot
}

fn main() {
    let input_file = File::open("input.txt").expect("File not found");
    let mut reader = BufReader::new(input_file);
    let mut line = String::new();
    let mut _line = String::new();

    // Read time.
    let _ = reader.read_line(&mut line);
    let _ = sscanf!(&line, "Time: {}", _line);
    let time: Vec<f32> = _line.trim().split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<f32>().unwrap()).collect();

    // Read distance.
    line = "".to_string();
    _line = "".to_string();
    let _ = reader.read_line(&mut line);
    let _ = sscanf!(&line, "Distance: {}", _line);
    let distances: Vec<f32> = _line.trim().split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<f32>().unwrap()).collect();

    let tot = solution(time, distances);
    println!("{}", tot);
}
