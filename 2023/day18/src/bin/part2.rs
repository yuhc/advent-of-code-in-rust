use scanf::sscanf;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

#[derive(Debug)]
struct Dig {
    dir: Direction,
    meters: u32,
}

#[derive(Debug, Clone, Copy)]
struct Segment {
    x: i32,
    y_min: i32,
    y_max: i32,
}

#[derive(Ord, Eq, PartialOrd, PartialEq, Debug, Clone, Copy)]
enum PointType {
    Start,  // Left endpoint
    End,    // Right endpoint
    Middle, // Middle points
}

enum PointState {
    Start,
    End,
    Inner,
    Outer,
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
    point_type: PointType,
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.y.partial_cmp(&self.y)
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

// It can be easily resolved by Shoe-lace formula, but here is a more instinct way.
fn solution(digs: Vec<Dig>) {
    // Get all horizontal segments.
    let mut segments: Vec<Segment> = Vec::new();
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for dig in digs.iter() {
        match dig.dir {
            Direction::Up => x -= dig.meters as i32,
            Direction::Right => {
                segments.push(Segment {
                    x: x,
                    y_min: y,
                    y_max: y + dig.meters as i32,
                });
                y += dig.meters as i32;
            }
            Direction::Down => x += dig.meters as i32,
            Direction::Left => {
                segments.push(Segment {
                    x: x,
                    y_min: y - dig.meters as i32,
                    y_max: y,
                });
                y -= dig.meters as i32;
            }
        }
    }
    if x != 0 || y != 0 {
        panic!("Did not return to origin");
    }
    println!("Segments: {:?}", segments);

    // Push all endpoints into a heap.
    // These are changing points of the lagoon height.
    let mut heap: BinaryHeap<Point> = BinaryHeap::new();
    for seg in segments.iter() {
        heap.push(Point {
            x: seg.x,
            y: seg.y_min,
            point_type: PointType::Start,
        });
        heap.push(Point {
            x: seg.x,
            y: seg.y_max,
            point_type: PointType::End,
        });
    }

    // Sweep from left to right.
    let mut count: u64 = 0;
    let mut y: i32 = i32::MIN / 2;
    let mut cur_h_segments: Vec<i32> = Vec::new(); // Stores all horizontal segments that we have passed Start but not yet End.

    while let Some(&p) = heap.peek() {
        let next_y = p.y;

        /******* Calculate the passed area *******/

        cur_h_segments.sort();
        let width = (next_y - y - 1) as u64;
        let height = cur_h_segments
            .chunks(2)
            .map(|chunk| (chunk[1] - chunk[0] + 1) as u64)
            .sum::<u64>();
        count += width * height;

        /******* Calculate the height at next_y *******/

        let mut h_segments_at_y: Vec<(i32, PointType)> = Vec::new();
        let mut new_x_at_y: Vec<i32> = Vec::new();

        // Pop all points with the same next_y.
        while heap.peek().map_or(false, |&p| p.y == next_y) {
            let p = heap.pop().unwrap();
            h_segments_at_y.push((p.x, p.point_type));
            match p.point_type {
                PointType::Start => {
                    new_x_at_y.push(p.x);
                }
                PointType::End => {
                    // Close the horizontal segment.
                    let s = cur_h_segments.iter().position(|&t| t == p.x).unwrap();
                    cur_h_segments.remove(s);
                }
                _ => panic!("Invalid endpoint type"),
            }
        }

        // We are crossing the middle of all remaining horizontal segments.
        h_segments_at_y.extend(cur_h_segments.iter().map(|&x| (x, PointType::Middle)));
        // Can use a heap here.
        h_segments_at_y.sort();
        println!("h_segments_at_y: {:?}", h_segments_at_y);
        cur_h_segments.extend(new_x_at_y.iter());

        let mut accounted_x_at_y: Vec<i32> = Vec::new();
        let mut state = PointState::Outer;
        for &(x, point_type) in h_segments_at_y.iter() {
            match (state, point_type) {
                (PointState::Outer, PointType::Start) => {
                    state = PointState::Start;
                    accounted_x_at_y.push(x);
                }
                (PointState::Outer, PointType::Middle) => {
                    state = PointState::Inner;
                    accounted_x_at_y.push(x);
                }
                (PointState::Outer, PointType::End) => {
                    state = PointState::End;
                    accounted_x_at_y.push(x);
                }
                (PointState::Inner, PointType::Start) => {
                    state = PointState::End;
                }
                (PointState::Inner, PointType::Middle) => {
                    state = PointState::Outer;
                    accounted_x_at_y.push(x);
                }
                (PointState::Inner, PointType::End) => {
                    state = PointState::Start;
                }
                (PointState::Start, PointType::Start) => {
                    state = PointState::Outer;
                    accounted_x_at_y.push(x);
                }
                (PointState::Start, PointType::Middle) => {
                    panic!("Invalid state");
                }
                (PointState::Start, PointType::End) => {
                    state = PointState::Inner;
                }
                (PointState::End, PointType::Start) => {
                    state = PointState::Inner;
                }
                (PointState::End, PointType::Middle) => {
                    panic!("Invalid state");
                }
                (PointState::End, PointType::End) => {
                    state = PointState::Outer;
                    accounted_x_at_y.push(x);
                }
            }
        }
        println!("accounted_x_at_y: {:?}", accounted_x_at_y);
        count += accounted_x_at_y
            .chunks(2)
            .map(|chunk| (chunk[1] - chunk[0] + 1) as u64)
            .sum::<u64>();
        y = next_y;
    }

    println!("Count: {}", count);
}

fn main() {
    let input_file = File::open("input.txt").expect("File not found");
    let reader = BufReader::new(input_file);

    let mut digs: Vec<Dig> = Vec::new();
    for line in reader.lines() {
        let mut dir_s: String = String::new();
        let mut meters: u32 = 0;
        let mut color: String = String::new();
        let _ = sscanf!(&line.unwrap(), "{} {} (#{})", dir_s, meters, color);
        let color_int = u32::from_str_radix(color.as_str(), 16).unwrap();
        digs.push(Dig {
            dir: match color_int % 16 {
                3 => Direction::Up,
                0 => Direction::Right,
                1 => Direction::Down,
                2 => Direction::Left,
                _ => panic!("Invalid direction"),
            },
            meters: color_int / 16,
        });
    }

    solution(digs);
}
