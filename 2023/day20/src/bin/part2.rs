use gcd::Gcd;
use scanf::sscanf;
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
    // RX is connected to only one conjunction.
    let rx_inputs = connections.iter().filter_map(|(k, v)| {
        if v.contains(&"rx".to_string()) {
            Some(k)
        } else {
            None
        }
    });
    let rx_inputs: Vec<_> = rx_inputs.collect();
    assert_eq!(rx_inputs.len(), 1);
    let rx_conjunction = rx_inputs[0];
    assert_eq!(types.get(rx_conjunction).unwrap(), &ModuleType::Conjunction);

    // The directly connected conjunction must all send high pulses.
    let rx_conjunction_inputs = connections
        .iter()
        .filter_map(|(k, v)| {
            if v.contains(rx_conjunction) {
                Some(k)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let mut loop_lens: Vec<u32> = Vec::new();

    for rx_conjunction_input in rx_conjunction_inputs {
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

        let mut count = 0;
        'press_loop: for elves_press in 1..=20000 {
            // Every broadcast initial pulse.
            let mut queue: Vec<(String, Pulse, String)> = vec![]; // (module, pulse, source)
            let broadcast_targets = connections.get("broadcaster").unwrap();
            for target in broadcast_targets {
                queue.push((target.clone(), Pulse::Low, "broadcaster".to_string()));
            }

            let mut idx = 0;
            while idx < queue.len() {
                let (name, pulse, source) = queue[idx].clone();
                // println!("Processing: {} -{:?}-> {}", source, pulse, name);
                idx += 1;

                if &source == rx_conjunction_input && pulse == Pulse::High {
                    println!("Elves press: {} {} -High-> {}", elves_press, source, name);
                    count += 1;
                    if count == 1 {
                        loop_lens.push(elves_press);
                    }
                    // Verify that all conjunctions are keeping looping.
                    assert_eq!(elves_press % loop_lens.last().unwrap(), 0);
                    if count == 3 {
                        break 'press_loop;
                    }
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
        }
    }

    // Compute LCM for all loop lengths.
    let mut lcm = loop_lens[0] as u64;
    for i in 1..loop_lens.len() {
        lcm = lcm * loop_lens[i] as u64 / lcm.gcd(loop_lens[i].into());
    }
    println!("LCM: {}", lcm);
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
