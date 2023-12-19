use std::collections::HashMap;
advent_of_code::solution!(19);

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
enum ConditionChar {
    LeftLower,
    RightLower,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct Condition {
    left: String,
    char: ConditionChar,
    right: u32,
    dest_flow: String,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct Flow {
    name: String,
    conditions: Vec<Condition>,
    last_dest_flow: String,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut load_parts = false;
    let mut flows: HashMap<String, Flow> = HashMap::new();
    let mut parts: Vec<HashMap<String, u32>> = vec![];

    for line in input.lines() {
        if line.len() <= 0 {
            load_parts = !load_parts;
            continue;
        }
        if !load_parts {
            let (name, dirt_conditions) = line.split_once("{").unwrap();
            let conditions_str = dirt_conditions.trim_matches('}');
            let mut flow_conditions: Vec<Condition> = vec![];
            for condition in conditions_str.split(',') {
                if condition.contains("<") || condition.contains(">") {
                    let condition_char = if condition.contains("<") {
                        ConditionChar::LeftLower
                    } else {
                        ConditionChar::RightLower
                    };
                    let parts: Vec<&str> = condition
                        .split(|c| c == '<' || c == '>' || c == ':')
                        .collect();
                    flow_conditions.push(Condition {
                        left: String::from(parts[0]),
                        char: condition_char,
                        right: parts[1].parse().unwrap(),
                        dest_flow: String::from(parts[2]),
                    });
                } else {
                    flows.insert(
                        String::from(name),
                        Flow {
                            name: String::from(name),
                            conditions: flow_conditions.clone(),
                            last_dest_flow: String::from(condition),
                        },
                    );
                }
            }
        } else {
            let curr_parts: Vec<&str> = line
                .trim_matches('{')
                .trim_matches('}')
                .split(',')
                .collect();
            let mut new_part: HashMap<String, u32> = HashMap::new();
            for &part in &curr_parts {
                let (key, value) = part.split_once('=').unwrap();
                new_part.insert(String::from(key), value.parse().unwrap());
            }
            parts.push(new_part)
        }
    }
    let mut parts_destination: Vec<(HashMap<String, u32>, String)> = vec![];
    for part in parts {
        let mut curr_flow = String::from("in");
        while curr_flow != String::from("A") && curr_flow != String::from("R") {
            let flow = flows.get(&curr_flow).unwrap();
            let mut find_condition = false;
            for condition in &flow.conditions {
                let part_value = part.get(&condition.left).expect("Missing key").clone();
                match condition.char {
                    ConditionChar::LeftLower => {
                        if part_value < condition.right {
                            curr_flow = condition.dest_flow.clone();
                            find_condition = true;
                            break;
                        }
                    }
                    ConditionChar::RightLower => {
                        if part_value > condition.right {
                            curr_flow = condition.dest_flow.clone();
                            find_condition = true;
                            break;
                        }
                    }
                }
            }
            if !find_condition {
                curr_flow = String::from(&flow.last_dest_flow);
            }
        }
        parts_destination.push((part.clone(), curr_flow));
    }
    let mut accepted_parts: Vec<(HashMap<String, u32>, String)> = vec![];
    parts_destination
        .iter()
        .filter(|(_, v)| v == &String::from("A"))
        .for_each(|(p, v)| accepted_parts.push((p.clone(), String::from(v))));
    let sum: u32 = accepted_parts
        .iter()
        .map(|(p, _)| p.values().copied().sum::<u32>())
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u128> {
    let mut load_parts = false;
    let mut flows: HashMap<String, Flow> = HashMap::new();

    for line in input.lines() {
        if line.len() <= 0 {
            load_parts = !load_parts;
            continue;
        }
        if !load_parts {
            let (name, dirt_conditions) = line.split_once("{").unwrap();
            let conditions_str = dirt_conditions.trim_matches('}');
            let mut flow_conditions: Vec<Condition> = vec![];
            for condition in conditions_str.split(',') {
                if condition.contains("<") || condition.contains(">") {
                    let condition_char = if condition.contains("<") {
                        ConditionChar::LeftLower
                    } else {
                        ConditionChar::RightLower
                    };
                    let parts: Vec<&str> = condition
                        .split(|c| c == '<' || c == '>' || c == ':')
                        .collect();
                    flow_conditions.push(Condition {
                        left: String::from(parts[0]),
                        char: condition_char,
                        right: parts[1].parse().unwrap(),
                        dest_flow: String::from(parts[2]),
                    });
                } else {
                    flows.insert(
                        String::from(name),
                        Flow {
                            name: String::from(name),
                            conditions: flow_conditions.clone(),
                            last_dest_flow: String::from(condition),
                        },
                    );
                }
            }
        }
    }
    let mut permutation_counts: HashMap<
        ((u32, u32), (u32, u32), (u32, u32), (u32, u32), String),
        u128,
    > = HashMap::new();
    let mut permutations_list: Vec<(
        ((u32, u32), (u32, u32), (u32, u32), (u32, u32), String),
        u128,
    )> = vec![];
    let mut sum_accepted: u128 = 0;
    permutation_counts.insert(
        (
            (1, 4000),
            (1, 4000),
            (1, 4000),
            (1, 4000),
            String::from("in"),
        ),
        1,
    );
    while permutation_counts.len() > 0 {
        permutations_list = vec![];
        for (key, permutations) in permutation_counts.iter() {
            let (x, m, a, s, flow_name) = key;
            let mut x = x.clone();
            let mut m = m.clone();
            let mut a = a.clone();
            let mut s = s.clone();
            let flow = flows.get(flow_name).unwrap();
            let mut last_use = true;
            for condition in &flow.conditions {
                let mut part_value = match condition.left.as_str() {
                    "x" => x,
                    "m" => m,
                    "a" => a,
                    "s" => s,
                    _ => panic!("Unknown condition!"),
                };
                match condition.char {
                    ConditionChar::LeftLower => {
                        if part_value.0 < condition.right && part_value.1 > condition.right {
                            let new_key = match condition.left.as_str() {
                                "x" => {
                                    x = (condition.right, part_value.1);
                                    (
                                        (part_value.0, condition.right - 1),
                                        m,
                                        a,
                                        s,
                                        condition.dest_flow.to_string(),
                                    )
                                }
                                "m" => {
                                    m = (condition.right, part_value.1);
                                    (
                                        x,
                                        (part_value.0, condition.right - 1),
                                        a,
                                        s,
                                        condition.dest_flow.to_string(),
                                    )
                                }
                                "a" => {
                                    a = (condition.right, part_value.1);
                                    (
                                        x,
                                        m,
                                        (part_value.0, condition.right - 1),
                                        s,
                                        condition.dest_flow.to_string(),
                                    )
                                }
                                "s" => {
                                    s = (condition.right, part_value.1);
                                    (
                                        x,
                                        m,
                                        a,
                                        (part_value.0, condition.right - 1),
                                        condition.dest_flow.to_string(),
                                    )
                                }
                                _ => panic!("Unknown condition!"),
                            };
                            permutations_list.push((new_key, *permutations + 1));
                        } else if part_value.1 < condition.right {
                            let new_key = (x, m, a, s, condition.dest_flow.to_string());
                            permutations_list.push((new_key, *permutations + 1));
                            last_use = false;
                            break;
                        }
                    }
                    ConditionChar::RightLower => {
                        if part_value.0 < condition.right && part_value.1 > condition.right {
                            let new_key = match condition.left.as_str() {
                                "x" => {
                                    x = (part_value.0, condition.right);
                                    (
                                        (condition.right + 1, part_value.1),
                                        m,
                                        a,
                                        s,
                                        condition.dest_flow.to_string(),
                                    )
                                }
                                "m" => {
                                    m = (part_value.0, condition.right);
                                    (
                                        x,
                                        (condition.right + 1, part_value.1),
                                        a,
                                        s,
                                        condition.dest_flow.to_string(),
                                    )
                                }
                                "a" => {
                                    a = (part_value.0, condition.right);
                                    (
                                        x,
                                        m,
                                        (condition.right + 1, part_value.1),
                                        s,
                                        condition.dest_flow.to_string(),
                                    )
                                }
                                "s" => {
                                    s = (part_value.0, condition.right);
                                    (
                                        x,
                                        m,
                                        a,
                                        (condition.right + 1, part_value.1),
                                        condition.dest_flow.to_string(),
                                    )
                                }
                                _ => panic!("Unknown condition!"),
                            };
                            permutations_list.push((new_key, *permutations + 1));
                        } else if part_value.0 > condition.right {
                            let new_key = (x, m, a, s, condition.dest_flow.to_string());
                            permutations_list.push((new_key, *permutations + 1));
                            last_use = false;
                            break;
                        }
                    }
                }
            }
            if last_use {
                let new_key = (x, m, a, s, flow.last_dest_flow.to_string());
                permutations_list.push((new_key, *permutations + 1));
            }
        }
        permutations_list.retain(|(key, v)| {
            let (x, m, a, s, flow_name) = key;
            match flow_name.as_str() {
                "A" => {
                    sum_accepted += (x.1 - x.0 + 1) as u128
                        * (m.1 - m.0 + 1) as u128
                        * (a.1 - a.0 + 1) as u128
                        * (s.1 - s.0 + 1) as u128;
                    false
                }
                "R" => false,
                _ => true,
            }
        });
        permutation_counts.clear();
        for (key, permutations) in &permutations_list {
            let (x, m, a, s, flow_name) = key;
            *permutation_counts
                .entry((*x, *m, *a, *s, flow_name.to_string()))
                .or_insert(0) += *permutations;
        }
    }
    println!("{}", sum_accepted);
    Some(sum_accepted)
}

fn calc_new_permutation(
    new_key: &((u32, u32), (u32, u32), (u32, u32), (u32, u32), String),
) -> u128 {
    (new_key.0 .1 - new_key.0 .0 + 1) as u128
        * (new_key.1 .1 - new_key.1 .0 + 1) as u128
        * (new_key.2 .1 - new_key.2 .0 + 1) as u128
        * (new_key.3 .1 - new_key.3 .0 + 1) as u128
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167409079868000));
    }
}
