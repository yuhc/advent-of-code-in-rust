use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Brick {
    x1: usize,
    y1: usize,
    z1: usize,
    x2: usize,
    y2: usize,
    z2: usize,

    supporting: Vec<usize>,   // bricks that this brick is supporting
    supported_by: Vec<usize>, // bricks that are supporting this brick
}

impl Brick {
    fn new(points: &[usize]) -> Brick {
        Brick {
            x1: points[0],
            y1: points[1],
            z1: points[2],
            x2: points[3],
            y2: points[4],
            z2: points[5],
            supporting: Vec::new(),
            supported_by: Vec::new(),
        }
    }

    fn min_z(&self) -> usize {
        self.z1.min(self.z2)
    }

    fn max_z(&self) -> usize {
        self.z1.max(self.z2)
    }
}

fn solution(bricks: &mut Vec<Brick>, ground_x: usize, ground_y: usize) {
    let n = bricks.len();
    // Brick index that is on the top of the ground at (x, y).
    let mut ground: Vec<Vec<usize>> = vec![vec![usize::MAX; ground_y]; ground_x];

    for i in 0..n {
        let mut z = 0;
        let mut lower_bricks: HashSet<usize> = HashSet::new();

        for x in bricks[i].x1..=bricks[i].x2 {
            for y in bricks[i].y1..=bricks[i].y2 {
                if ground[x][y] != usize::MAX {
                    z = z.max(bricks[ground[x][y]].max_z());
                    lower_bricks.insert(ground[x][y]);
                }
                ground[x][y] = i;
            }
        }

        // Filter lower bricks that are really supporting the n-th brick.
        for b in lower_bricks {
            if bricks[b].max_z() == z {
                bricks[b].supporting.push(i);
                bricks[i].supported_by.push(b);
            }
        }

        // Update this brick's z.
        if bricks[i].z1 > bricks[i].z2 {
            let diff = bricks[i].z1 - bricks[i].z2;
            bricks[i].z1 = z + 1;
            bricks[i].z2 = z + 1 + diff;
        } else {
            let diff = bricks[i].z2 - bricks[i].z1;
            bricks[i].z2 = z + 1;
            bricks[i].z1 = z + 1 + diff;
        }
    }

    // Check which bricks can be safely removed.
    let mut count = 0;
    'outer: for i in 1..n {
        let supporting = &bricks[i].supporting;
        for s in supporting {
            if bricks[*s].supported_by.len() == 1 {
                continue 'outer;
            }
        }
        count += 1;
    }
    println!("Count: {}", count);
}

fn main() {
    let input_file = File::open("input.txt").expect("File not found");
    let reader = BufReader::new(input_file);

    let mut bricks: Vec<Brick> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let points = line
            .split(&[',', '~'])
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        bricks.push(Brick::new(&points));
    }

    // Sort by z and assign indices.
    bricks.sort_by(|a, b| a.min_z().cmp(&b.min_z()));
    println!("Bricks: {:?}", bricks);

    let ground_x = bricks.iter().map(|b| b.x1.max(b.x2)).max().unwrap() + 1;
    let ground_y = bricks.iter().map(|b| b.y1.max(b.y2)).max().unwrap() + 1;
    println!("Ground: {}x{}", ground_x, ground_y);

    solution(&mut bricks, ground_x, ground_y);
}
