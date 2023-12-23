use scanf::sscanf;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Eq, PartialEq)]
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

type Ranges = HashSet<(u32, u32)>;

fn score(variables: &HashMap<char, Ranges>) -> u64 {
    let mut tot: u64 = 1;
    for (_, v) in variables {
        for (l, r) in v {
            tot *= (r - l + 1) as u64;
        }
    }
    println!("Score: {:?} => {}", variables, tot);
    tot
}

fn dfs(
    workflows: &HashMap<String, Vec<Condition>>,
    mut variables: HashMap<char, Ranges>,
    rule: String,
) -> u64 {
    let mut tot: u64 = 0;
    let conds = workflows.get(&rule).unwrap();

    for cond in conds {
        let target = &cond.target;

        if cond.op == Op::Always {
            if target == "A" {
                tot += score(&variables);
            } else if target != "R" {
                tot += dfs(workflows, variables.clone(), target.to_string());
            }
            break;
        }

        // If the condition is met, we can continue to that workflow.
        if cond.op == Op::LessThan &&
            variables.get(&cond.variable).unwrap().iter().any(|(l, _)| l < &cond.value) {
            // Meet the condition.
            if target != "R" {
                let mut v = variables.clone();
                let s = v.get_mut(&cond.variable).unwrap();
                for r in s.clone().iter() {
                    if r.0 >= cond.value {
                        s.remove(&r);
                    }
                    else if r.1 >= cond.value {
                        s.insert((r.0, cond.value - 1));
                        s.remove(&r);
                    }
                }
                if target == "A" {
                    tot += score(&v);
                }
                else {
                    tot += dfs(workflows, v, target.to_string());
                }
            }

            // Do not meet the condition. Continue to the next condition.
            let s = variables.get_mut(&cond.variable).unwrap();
            for r in s.clone().iter() {
                if r.1 < cond.value {
                    s.remove(&r);
                }
                else if r.0 < cond.value {
                    s.insert((cond.value, r.1));
                    s.remove(&r);
                }
            }
        }
        else if cond.op == Op::GreaterThan {
            if variables.get(&cond.variable).unwrap().iter().any(|(_, r)| r > &cond.value) {
                // Meet the condition.
                if target != "R" {
                    let mut v = variables.clone();
                    let s = v.get_mut(&cond.variable).unwrap();
                    for r in s.clone().iter() {
                        if r.1 <= cond.value {
                            s.remove(&r);
                        }
                        else if r.0 <= cond.value {
                            s.insert((cond.value + 1, r.1));
                            s.remove(&r);
                        }
                    }
                    if target == "A" {
                        tot += score(&v);
                    }
                    else {
                        tot += dfs(workflows, v, target.to_string());
                    }
                }

                // Do not meet the condition. Continue to the next condition.
                let s = variables.get_mut(&cond.variable).unwrap();
                for r in s.clone().iter() {
                    if r.0 > cond.value {
                        s.remove(&r);
                    }
                    else if r.1 > cond.value {
                        s.insert((r.0, cond.value));
                        s.remove(&r);
                    }
                }
            }
        }
    }

    tot
}

fn solution(workflows: &HashMap<String, Vec<Condition>>) -> u64 {
    let mut variables: HashMap<char, Ranges> = HashMap::new();
    variables.insert('x', vec![(1, 4000)].into_iter().collect());
    variables.insert('m', vec![(1, 4000)].into_iter().collect());
    variables.insert('a', vec![(1, 4000)].into_iter().collect());
    variables.insert('s', vec![(1, 4000)].into_iter().collect());

    dfs(workflows, variables, "in".to_string())
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

            let variable: char = left.chars().nth(0).unwrap();
            let op = match left.chars().nth(1).unwrap() {
                '<' => Op::LessThan,
                '>' => Op::GreaterThan,
                _ => panic!("Invalid op"),
            };
            let value: u32 = left[2..].parse::<u32>().unwrap();

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

    let tot = solution(&workflows);
    println!("Total: {}", tot);
}
