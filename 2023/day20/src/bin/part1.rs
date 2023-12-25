use scanf::sscanf;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
enum ModuleType {
    FlipFlop,    // %
    Conjunction, // &
    Broadcast,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
enum Pulse {
    High,
    Low,
}

type ModuleState = HashMap<String, Pulse>; // Input module name -> pulse state
type State = HashMap<String, ModuleState>; // Module name -> module state

fn solution(connections: &HashMap<String, Vec<String>>, types: &HashMap<String, ModuleType>) {
    let mut states: Vec<State> = Vec::new();
    let mut high_pulses: Vec<u64> = Vec::new();
    let mut low_pulses: Vec<u64> = Vec::new();

    let mut state: State = State::new();
    for (module, type_) in types {
        state.insert(module.clone(), ModuleState::new());
        if type_ == &ModuleType::FlipFlop {
            state
                .get_mut(module)
                .unwrap()
                .insert(module.clone(), Pulse::Low);
        } else if type_ == &ModuleType::Conjunction {
            for (input_mod, conns) in connections {
                if conns.contains(module) {
                    state
                        .get_mut(module)
                        .unwrap()
                        .insert(input_mod.clone(), Pulse::Low);
                }
            }
        }
    }
    println!("State: {:?}", state);
    states.push(state.clone());

    let mut loop_start = 0;
    let mut loop_end = 0;
    for elves_press in 1..=1000 {
        // Every broadcast initial pulse.
        let mut queue: Vec<(String, Pulse, String)> = vec![]; // (module, pulse, source)
        let broadcast_targets = connections.get("broadcaster").unwrap();
        for target in broadcast_targets {
            queue.push((target.clone(), Pulse::Low, "broadcaster".to_string()));
        }

        let mut high_pulse = 0;
        let mut low_pulse = 1;
        let mut idx = 0;
        while idx < queue.len() {
            let (name, pulse, source) = queue[idx].clone();
            println!("Processing: {} -{:?}-> {}", source, pulse, name);
            idx += 1;

            if pulse == Pulse::Low {
                low_pulse += 1;
            } else {
                high_pulse += 1;
            }

            let ty = types.get(&name);
            if ty.is_none() {
                continue;
            }
            let ty = ty.unwrap();

            if ty == &ModuleType::FlipFlop {
                if pulse == Pulse::High {
                    continue;
                }

                let ff_state = state.get_mut(&name).unwrap().get_mut(&name).unwrap();
                let output = if ff_state == &Pulse::Low {
                    Pulse::High
                } else {
                    Pulse::Low
                };
                *ff_state = output;

                // Send to its connections.
                let conns = connections.get(&name).unwrap();
                for conn in conns {
                    queue.push((conn.clone(), output, name.clone()));
                }
            } else if ty == &ModuleType::Conjunction {
                let conf_state = state.get_mut(&name).unwrap();
                conf_state.insert(source.clone(), pulse.clone());
                let mut output = Pulse::High;
                if conf_state.values().all(|p| p == &Pulse::High) {
                    output = Pulse::Low;
                }

                // Send to its connections.
                let conns = connections.get(&name).unwrap();
                for conn in conns {
                    queue.push((conn.clone(), output, name.clone()));
                }
            }
        }

        high_pulses.push(high_pulse);
        low_pulses.push(low_pulse);

        match states.iter().position(|s| s == &state) {
            Some(pos) => {
                // println!("Loop detected!");
                // loop_start = pos;
                // loop_end = elves_press;
                // break;
                states.push(state.clone());
            }
            None => {
                states.push(state.clone());
            }
        }
    }

    println!("Loop start: {}", loop_start);
    println!("Loop end: {}", loop_end);
    // println!("States: {:?}", states);
    // println!("High pulses: {:?}", high_pulses);
    // println!("Low pulses: {:?}", low_pulses);

    let mut tot = high_pulses.iter().sum::<u64>() * low_pulses.iter().sum::<u64>();
    println!("Total: {}", tot);
}

fn main() {
    let input_file = File::open("input.txt").expect("File not found");
    let reader = BufReader::new(input_file);
    let lines: Vec<_> = reader.lines().collect();
    let mut iter = lines.iter();

    let mut connections: HashMap<String, Vec<String>> = HashMap::new();
    let mut types: HashMap<String, ModuleType> = HashMap::new();
    while let Some(Ok(line)) = iter.next() {
        let mut module: String = String::new();
        let mut dest_str: String = String::new();
        let _ = sscanf!(&line, "{} -> {}", module, dest_str);

        let mut module_type: ModuleType = ModuleType::Broadcast;
        let mut name: String = "broadcaster".to_string();
        if module.starts_with("%") {
            name = module[1..].to_string();
            module_type = ModuleType::FlipFlop;
        } else if module.starts_with("&") {
            name = module[1..].to_string();
            module_type = ModuleType::Conjunction;
        }
        types.insert(name.clone(), module_type);

        connections.insert(name.clone(), vec![]);
        dest_str.split(",").for_each(|dest| {
            let dest = dest.trim().to_string();
            connections.get_mut(&name).unwrap().push(dest);
        });
    }

    println!("{:?}", types);
    println!("{:?}", connections);

    solution(&connections, &types);
}
