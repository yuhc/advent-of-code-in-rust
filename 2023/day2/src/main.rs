use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use scanf::sscanf;

#[allow(dead_code)]
fn part1() {
    let input_file = File::open("input.txt").expect("File not found");
    let reader = BufReader::new(input_file);

    let mut tot = 0;
    'outer: for line in reader.lines() {
        let mut game_id = 0;
        let mut game_string: String = String::new();
        sscanf!(line.unwrap().as_str(), "Game {}: {}", game_id, game_string);

        // ["3 blue, 4 red", " 1 red, 2 green, 6 blue", " 2 green"]
        let cube_sets = game_string.trim().split(";").collect::<Vec<&str>>();
        for cube_set in cube_sets {
            // ["3 blue", " 4 red"]
            let cubes = cube_set.trim().split(",").collect::<Vec<&str>>();
            for cube in cubes {
                let pair = cube.trim().split(' ').collect::<Vec<&str>>();
                match pair[1] {
                    "red" => if pair[0].parse::<i32>().unwrap() > 12 { continue 'outer; },
                    "blue" => if pair[0].parse::<i32>().unwrap() > 14 { continue 'outer; },
                    "green" => if pair[0].parse::<i32>().unwrap() > 13 { continue 'outer; },
                    _ => panic!(),
                };
            }
        }

        tot += game_id;
    }
    println!("{}", tot);
}

fn part2() {
    let input_file = File::open("input.txt").expect("File not found");
    let reader = BufReader::new(input_file);

    let mut tot = 0;
    for line in reader.lines() {
        let mut game_id = 0;
        let mut game_string: String = String::new();
        sscanf!(line.unwrap().as_str(), "Game {}: {}", game_id, game_string);

        // ["3 blue, 4 red", " 1 red, 2 green, 6 blue", " 2 green"]
        let cube_sets = game_string.trim().split(";").collect::<Vec<&str>>();
        let mut max_red = 0;
        let mut max_blue = 0;
        let mut max_green = 0;
        for cube_set in cube_sets {
            // ["3 blue", " 4 red"]
            let cubes = cube_set.trim().split(",").collect::<Vec<&str>>();
            for cube in cubes {
                let pair = cube.trim().split(' ').collect::<Vec<&str>>();
                match pair[1] {
                    "red" => max_red = max_red.max(pair[0].parse::<i32>().unwrap()),
                    "blue" => max_blue = max_blue.max(pair[0].parse::<i32>().unwrap()),
                    "green" => max_green = max_green.max(pair[0].parse::<i32>().unwrap()),
                    _ => panic!(),
                };
            }
        }

        tot += max_red * max_blue * max_green;
    }
    println!("{}", tot);
}

fn main() {
    part2();
}
