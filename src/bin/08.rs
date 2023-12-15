use std::collections::HashMap;
advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let mut moves: Vec<i32> = Vec::new();
    let mut roads: HashMap<String, Vec<String>> = HashMap::new();

    for (idx, line) in input.lines().enumerate() {
        match idx {
            0 => {
                moves = String::from(line)
                    .chars()
                    .map(|c| if c == 'L' { 0 } else { 1 })
                    .collect()
            }
            1 => {}
            _ => {
                if let Some((road, possible_values)) = line.split_once("=") {
                    let cleaned_string: String = possible_values
                        .chars()
                        .filter(|&c| c != '(' && c != ')')
                        .collect();

                    // Split the string by comma and trim whitespace
                    let parts: Vec<String> = cleaned_string
                        .split(',')
                        .map(|s| String::from(s.trim()))
                        .collect();

                    roads.insert(String::from(road.trim()), parts);
                }
            }
        }
    }

    let mut is_finish = false;
    let mut count_moves = 0;
    let mut curr_stage = String::from("AAA");

    while !is_finish {
        for m in &moves {
            count_moves += 1;
            let road = roads.get(&curr_stage.clone()).unwrap();
            curr_stage = road.get(*m as usize).unwrap().to_string();
            if curr_stage == String::from("ZZZ") {
                is_finish = true;
                break;
            }
        }
    }

    // println!("{:?}", roads);
    println!("{:?}", curr_stage);
    println!("{:?}", count_moves);
    Some(count_moves as u32)
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}

fn vec_lcm(numbers: &Vec<i32>) -> i64 {
    numbers
        .iter()
        .copied()
        .fold(1, |acc, num| lcm(acc, num as i64))
}
pub fn part_two(input: &str) -> Option<u64> {
    let mut moves: Vec<i32> = Vec::new();
    let mut roads: HashMap<String, Vec<String>> = HashMap::new();

    for (idx, line) in input.lines().enumerate() {
        match idx {
            0 => {
                moves = String::from(line)
                    .chars()
                    .map(|c| if c == 'L' { 0 } else { 1 })
                    .collect()
            }
            1 => {}
            _ => {
                if let Some((road, possible_values)) = line.split_once("=") {
                    let cleaned_string: String = possible_values
                        .chars()
                        .filter(|&c| c != '(' && c != ')')
                        .collect();

                    // Split the string by comma and trim whitespace
                    let parts: Vec<String> = cleaned_string
                        .split(',')
                        .map(|s| String::from(s.trim()))
                        .collect();

                    roads.insert(String::from(road.trim()), parts);
                }
            }
        }
    }

    let mut count_moves = 0;
    let keys_ending_with_a: Vec<&String> = roads.keys().filter(|&key| key.ends_with('A')).collect();
    let mut curr_stages: Vec<&String> = keys_ending_with_a.clone();

    let mut count_while: i64 = 0;

    let mut z_position: HashMap<String, i32> = HashMap::new();

    while z_position.keys().len() < keys_ending_with_a.len() {
        count_while += 1;
        if count_while % 1000 == 0 {
            println!("loop {}", count_while);
        }
        for m in &moves {
            let mut new_stages: Vec<&String> = vec![];
            count_moves += 1;
            curr_stages
                .iter()
                .enumerate()
                .for_each(|(idx, &curr_stage)| {
                    let road = roads.get(&curr_stage.clone()).unwrap();
                    let found_rouad = road.get(*m as usize).unwrap();
                    new_stages.push(found_rouad);
                    if found_rouad.ends_with("Z") {
                        let key = keys_ending_with_a.get(idx).unwrap().to_string();
                        z_position.entry(key.clone()).or_insert(count_moves);
                    }
                });
            curr_stages = new_stages;
        }
    }

    let v: Vec<i32> = z_position.values().into_iter().map(|v| *v).collect();
    let result = vec_lcm(&v);

    // println!("{:?}", result);
    // println!("{:?}", curr_stages);
    // println!("{:?}", count_moves);
    Some(result as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
