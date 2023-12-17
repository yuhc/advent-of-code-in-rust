use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use scanf::sscanf;
use std::option::Option;
use std::cmp::max;

fn solution(T: f64, D: f64) -> u64 {
    let mut tot = 1;

    // x^2 - Tx + D < 0
    // No solution.
    if T * T < 4.0 * D {
        return 0;
    }
    // One solution.
    else if T * T == 4.0 * D {
        if (T as u64) % 2 == 1 {
            return 0;
        }
    }
    // Two solutions.
    else {
        let mut x1 = ((T - (T*T - 4.0*D).sqrt()) / 2.0).ceil() as u64;
        let mut x2 = ((T + (T*T - 4.0*D).sqrt()) / 2.0).floor() as u64;
        if ((x1 * x1) as f64) - T * (x1 as f64) + D == 0.0 {
            x1 += 1;
        }
        if ((x2 * x2) as f64) - T * (x2 as f64) + D == 0.0 {
            x2 -= 1;
        }

        tot *= max(x2 - x1 + 1, 0);
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
    let time = _line.trim().split(' ')
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .join("").parse::<f64>().unwrap();

    // Read distance.
    line = "".to_string();
    _line = "".to_string();
    let _ = reader.read_line(&mut line);
    let _ = sscanf!(&line, "Distance: {}", _line);
    let distance = _line.trim().split(' ')
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .join("").parse::<f64>().unwrap();

    let tot = solution(time, distance);
    println!("{}", tot);
}
