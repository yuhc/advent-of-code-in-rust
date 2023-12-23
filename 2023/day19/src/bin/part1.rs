use scanf::sscanf;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Op {
    LessThan,
    GreaterThan,
    Always,
}

#[derive(Debug)]
struct Condition {
    variable: char,
    op: Op,
    value: u32,
    target: String,
}

fn solution(workflows: &HashMap<String, Vec<Condition>>, variables: &HashMap<char, u32>) -> bool {
    let mut target = "in".to_string();
    loop {
        let conds = workflows.get(&target).unwrap();
        for cond in conds {
            let value = variables.get(&cond.variable).unwrap();
            if match cond.op {
                Op::LessThan => value < &cond.value,
                Op::GreaterThan => value > &cond.value,
                Op::Always => true,
            } {
                target = cond.target.clone();
                if target == "A" {
                    return true;
                } else if target == "R" {
                    return false;
                }
                break;
            }
        }
    }
}

fn main() {
    let input_file = File::open("input.txt").expect("File not found");
    let reader = BufReader::new(input_file);
    let lines: Vec<_> = reader.lines().collect();
    let mut iter = lines.iter();

    let mut workflows: HashMap<String, Vec<Condition>> = HashMap::new();
    while let Some(Ok(line)) = iter.next() {
        if line.is_empty() {
            break;
        }
        let mut name: String = String::new();
        let mut conds: String = String::new();
        let _ = sscanf!(&line, "{}{{{}}}", name, conds);

        let conds = conds.split(",").collect::<Vec<_>>();
        let len = conds.len();
        let mut cond_vec: Vec<Condition> = Vec::new();
        for (i, cond) in conds.iter().enumerate() {
            if i == len - 1 {
                cond_vec.push(Condition {
                    variable: 'Z',
                    op: Op::Always,
                    value: 0,
                    target: conds[i].to_string(),
                });
                break;
            }

            let mut left: String = String::new();
            let mut target: String = String::new();
            let _ = sscanf!(&cond, "{}:{}", left, target);

            let mut variable: char = left.chars().nth(0).unwrap();
            let op = match left.chars().nth(1).unwrap() {
                '<' => Op::LessThan,
                '>' => Op::GreaterThan,
                _ => panic!("Invalid op"),
            };
            let mut value: u32 = left[2..].parse::<u32>().unwrap();

            cond_vec.push(Condition {
                variable,
                op,
                value,
                target,
            });
        }

        workflows.insert(name, cond_vec);
    }
    println!("{:?}", workflows);

    let mut tot = 0;
    while let Some(Ok(line)) = iter.next() {
        let mut variables: HashMap<char, u32> = HashMap::new();
        variables.insert('Z', 0);
        let len = line.len();
        let line = &line[1..len - 1];

        for v in line.split(",") {
            let mut name: char = v.chars().nth(0).unwrap();
            let mut value: u32 = v[2..].parse::<u32>().unwrap();
            variables.insert(name, value);
        }

        println!("{:?}", variables);
        let accepted = solution(&workflows, &variables);
        if accepted {
            tot += variables.iter().map(|(_, v)| v).sum::<u32>();
        }
    }
    println!("Total: {}", tot);
}
