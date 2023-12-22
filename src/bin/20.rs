use log::log;
use std::collections::HashMap;
use std::process::id;
advent_of_code::solution!(20);

#[derive(Debug, Eq, PartialEq, Clone)]
enum ModulesType {
    FlipFlop,
    Conjunction,
    Broadcast,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Module {
    name: String,
    type_: ModulesType,
    dest: Vec<String>,
    state: bool,
    inputs: Vec<String>,
    input_states: HashMap<String, bool>,
}

impl Module {
    fn get_output(&mut self, input: bool) -> Option<bool> {
        let output = match self.type_ {
            ModulesType::FlipFlop => {
                if !input {
                    self.state = !self.state;
                    return Some(self.state);
                }
                None
            }
            ModulesType::Conjunction => {
                let result = if self.input_states.len() == self.inputs.len() {
                    !self.input_states.iter().all(|(_, s)| *s)
                } else {
                    true
                };
                Some(result)
            }
            ModulesType::Broadcast => {
                self.state = input;
                Some(self.state)
            }
        };
        // if let Some(x) = output {
        //     println!("output: {} -> {}", self.name, x);
        // }
        output
    }

    fn store_input(&mut self, source: String, input: bool) {
        match self.type_ {
            ModulesType::Conjunction => {
                self.input_states.insert(source, input);
            }
            _ => {}
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut modules: HashMap<String, Module> = HashMap::new();

    for line in input.lines() {
        let (name, dest) = line.split_once("->").unwrap();
        let dest: Vec<String> = dest.split(",").map(|p| String::from(p.trim())).collect();
        if name.contains("broadcaster") {
            modules.insert(
                String::from("broadcaster"),
                Module {
                    name: String::from("broadcaster"),
                    type_: ModulesType::Broadcast,
                    dest,
                    state: false,
                    inputs: vec![],
                    input_states: HashMap::new(),
                },
            );
        } else if name.contains("%") {
            let name = name[1..].trim();
            modules.insert(
                String::from(name),
                Module {
                    name: String::from(name),
                    type_: ModulesType::FlipFlop,
                    dest,
                    state: false,
                    inputs: vec![],
                    input_states: HashMap::new(),
                },
            );
        } else {
            let name = name[1..].trim();
            modules.insert(
                String::from(name),
                Module {
                    name: String::from(name),
                    type_: ModulesType::Conjunction,
                    dest,
                    state: false,
                    inputs: vec![],
                    input_states: HashMap::new(),
                },
            );
        }
    }

    for (module_name, module) in modules.clone().iter_mut() {
        for dest in &module.dest {
            if let Some(dest_module) = modules.get_mut(dest) {
                dest_module.inputs.push(module_name.clone());
            }
        }
    }

    let mut signals_count: HashMap<bool, u32> = HashMap::new();
    signals_count.insert(true, 0);
    signals_count.insert(false, 0);

    for _ in 0..1000 {
        let mut signals = vec![(String::from("broadcaster"), false)];
        *signals_count.get_mut(&false).unwrap() += 1;
        loop {
            if let Some((module_name, signal)) = signals.pop() {
                let mut curr_module = modules
                    .get_mut(&module_name)
                    .expect("Missing module")
                    .clone();
                if let Some(output) = curr_module.get_output(signal) {
                    for dest in &curr_module.dest {
                        *signals_count.get_mut(&output).unwrap() += 1;
                        // println!("input: {} -> {} -> {}", curr_module.name.to_string(), output, dest);
                        if let Some(dest_module) = modules.get_mut(dest) {
                            dest_module.store_input(curr_module.name.to_string(), output);
                            signals.insert(0, (dest.clone(), output));
                        }
                    }
                }
                modules.insert(curr_module.name.clone(), curr_module.clone());
            } else {
                break;
            }
        }
    }

    let result: u32 = signals_count.values().into_iter().product();
    // println!("{:?}", modules);
    // println!("{:?}", signals_count);

    Some(result)
}

// Function to calculate the greatest common divisor (GCD) of two numbers
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

// Function to calculate the least common multiple (LCM) of two numbers
fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        a * b / gcd(a, b)
    }
}

// Function to calculate the LCM for a vector of integers
fn calculate_lcm(numbers: Vec<u64>) -> u64 {
    // If the vector is empty, LCM is undefined
    if numbers.is_empty() {
        return 0;
    }

    // Initialize LCM with the first element of the vector
    let mut result = numbers[0];

    // Iterate over the remaining elements and update LCM
    for &num in numbers.iter().skip(1) {
        result = lcm(result, num);
    }

    result
}
pub fn part_two(input: &str) -> Option<u64> {
    let mut modules: HashMap<String, Module> = HashMap::new();

    for line in input.lines() {
        let (name, dest) = line.split_once("->").unwrap();
        let dest: Vec<String> = dest.split(",").map(|p| String::from(p.trim())).collect();
        if name.contains("broadcaster") {
            modules.insert(
                String::from("broadcaster"),
                Module {
                    name: String::from("broadcaster"),
                    type_: ModulesType::Broadcast,
                    dest,
                    state: false,
                    inputs: vec![],
                    input_states: HashMap::new(),
                },
            );
        } else if name.contains("%") {
            let name = name[1..].trim();
            modules.insert(
                String::from(name),
                Module {
                    name: String::from(name),
                    type_: ModulesType::FlipFlop,
                    dest,
                    state: false,
                    inputs: vec![],
                    input_states: HashMap::new(),
                },
            );
        } else {
            let name = name[1..].trim();
            modules.insert(
                String::from(name),
                Module {
                    name: String::from(name),
                    type_: ModulesType::Conjunction,
                    dest,
                    state: false,
                    inputs: vec![],
                    input_states: HashMap::new(),
                },
            );
        }
    }

    for (module_name, module) in modules.clone().iter_mut() {
        for dest in &module.dest {
            if let Some(dest_module) = modules.get_mut(dest) {
                dest_module.inputs.push(module_name.clone());
            }
        }
    }

    //vec![String::from("rx")];
    let mut find_parts: Vec<String> = modules
        .iter()
        .filter(|(_, &ref m)| m.dest.contains(&String::from("rx")))
        .map(|(_, &ref m)| String::from(&m.name))
        .collect();

    let mut result_signal = false;

    let mut should_continue = true;
    loop {
        let mut new_find_modules = vec![];
        for fp in &find_parts {
            if let Some(curr_module) = modules.get(fp) {
                for inp in &curr_module.inputs {
                    if let Some(prev_module) = modules.get(inp) {
                        new_find_modules.push(prev_module.clone());
                    } else {
                        should_continue = false;
                    }
                }
            } else {
                should_continue = false;
                break;
            }
        }
        if !should_continue {
            break;
        }
        if new_find_modules
            .iter()
            .all(|m| m.type_ == ModulesType::Conjunction)
        {
            find_parts = new_find_modules
                .iter()
                .map(|m| String::from(&m.name))
                .collect();
            result_signal = !result_signal;
        } else {
            break;
        }
    }

    let mut signals_count: HashMap<bool, u32> = HashMap::new();
    signals_count.insert(true, 0);
    signals_count.insert(false, 0);

    let mut find_rx = false;
    let mut idx = 0;
    let mut result_count: HashMap<String, u32> = HashMap::new();
    loop {
        idx += 1;
        let mut signals = vec![(String::from("broadcaster"), false)];
        *signals_count.get_mut(&false).unwrap() += 1;
        loop {
            if let Some((module_name, signal)) = signals.pop() {
                let mut curr_module = modules
                    .get_mut(&module_name)
                    .expect("Missing module")
                    .clone();
                if let Some(output) = curr_module.get_output(signal) {
                    if find_parts.contains(&curr_module.name) && output == result_signal {
                        println!("{}", curr_module.name.to_string());
                        result_count
                            .entry(curr_module.name.to_string())
                            .or_insert(idx);
                    }
                    if find_parts.len() == result_count.len() {
                        find_rx = true;
                    }
                    for dest in &curr_module.dest {
                        *signals_count.get_mut(&output).unwrap() += 1;
                        // println!("input: {} -> {} -> {}", curr_module.name.to_string(), output, dest);
                        if let Some(dest_module) = modules.get_mut(dest) {
                            dest_module.store_input(curr_module.name.to_string(), output);
                            signals.insert(0, (dest.clone(), output));
                        }
                    }
                }
                modules.insert(curr_module.name.clone(), curr_module.clone());
            } else {
                break;
            }
        }
        if find_rx {
            break;
        }
    }
    let values: Vec<u64> = result_count
        .values()
        .into_iter()
        .map(|v| *v as u64)
        .collect();
    let result = calculate_lcm(values);
    println!("{}", result);
    // println!("{:?}", signals_count);

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_a() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(32000000));
    }

    #[test]
    fn test_part_one_b() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(11687500));
    }

    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
}
