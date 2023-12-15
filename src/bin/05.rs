use std::collections::HashMap;
use std::fmt::Debug;
use std::str::FromStr;
advent_of_code::solution!(5);

trait SplitToVector<T: FromStr> {
    fn to_vector(self) -> Vec<T> where <T as FromStr>::Err: Debug;
}

impl<T: FromStr> SplitToVector<T> for String {
    fn to_vector(self) -> Vec<T> where <T as FromStr>::Err: Debug {
        let mut result: Vec<T> = Vec::new();
        self.split_whitespace().for_each(|number| result.push(number.parse().unwrap()));
        result
    }
}

pub fn find_min_hashmap_by_value<T: Ord + Copy>(
    vec_of_hashmaps: &Vec<HashMap<String, T>>,
    key_to_find: &String
) -> Option<HashMap<String, T>> {
    // Ensure the vector is not empty
    if vec_of_hashmaps.is_empty() {
        return None;
    }

    // Initialize with the first HashMap in the vector
    let mut min_hashmap = vec_of_hashmaps[0].clone();

    // Iterate through the rest of the vector
    for hashmap in vec_of_hashmaps.iter().skip(1) {
        // Check if the current HashMap has a smaller value
        if hashmap.get(key_to_find) < min_hashmap.get(key_to_find) {
            min_hashmap = hashmap.clone();
        }
    }

    Some(min_hashmap)
}
pub fn part_one(input: &str) -> Option<u32> {
    let mut maps: HashMap<String, HashMap<(i64, i64), (i64, i64)>> = HashMap::new();
    let mut maps_order: Vec<String> = Vec::new();
    let mut seeds: Vec<i64> = Vec::new();
    let mut curr_key = String::from("");
    for (idx, line) in input.lines().enumerate() {
        if idx <= 0 {
            let (_, part2) = line.split_once(" ").unwrap();
            seeds = String::from(part2).to_vector();
        } else if line.len() > 0 {
            if line.contains(":") {
                curr_key = line.split_once(" ")
                    .map(|(s1, s2)| String::from(s1))
                    .unwrap();
                maps.entry(curr_key.clone()).or_insert(HashMap::new());
                maps_order.push(curr_key.clone());
            } else {
                let num = String::from(line).to_vector();
                let right_start: &i64 = num.get(0).unwrap();
                let left_start: &i64 = num.get(1).unwrap();
                let range = *num.get(2).unwrap() - 1;
                let mut curr_map: &mut HashMap<(i64, i64), (i64, i64)> = maps.get_mut(&curr_key).unwrap();
                curr_map.entry((*left_start, *left_start + range))
                    .or_insert((*right_start, *right_start + range));
            }
        }
    }
    let mut result: Vec<HashMap<String, i64>> = Vec::new();
    for seed in seeds {
        let mut seed_map: HashMap<String, i64> = HashMap::new();
        seed_map.insert(String::from("seed"), seed);
        let mut curr_num = seed;
        for map in &maps_order {
            let mut curr_value: Option<i64> = None;
            for (from, to) in maps.get(map).unwrap().iter() {
                if curr_num >= from.0 && curr_num <= from.1 {
                    let curr_num_idx = curr_num - from.0;
                    curr_value = Some(to.0 + curr_num_idx);
                }
            }
            if let Some(value) = curr_value {} else {
                curr_value = Some(curr_num);
            }
            seed_map.insert(String::from(map), curr_value.unwrap());
            curr_num = curr_value.unwrap();
        }
        result.push(seed_map);
    }
    let min_seed = find_min_hashmap_by_value(
        &result,
        &String::from("humidity-to-location"),
    ).unwrap();
    // println!("{:?}", maps);
    // println!("{:?}", result);

    print!("Min location: {}", min_seed.get(&String::from("humidity-to-location")).unwrap());
    Some(*min_seed.get(&String::from("humidity-to-location")).unwrap() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut maps: HashMap<String, HashMap<(i64, i64), (i64, i64)>> = HashMap::new();
    let mut maps_order: Vec<String> = Vec::new();
    let mut seeds: Vec<(i64, i64)> = Vec::new();
    let mut curr_key = String::from("");
    for (idx, line) in input.lines().enumerate() {
        if idx <= 0 {
            let (_, part2) = line.split_once(" ").unwrap();
            let tmp: Vec<i64> = String::from(part2).to_vector();
            for i in (0..tmp.len()).step_by(2) {
                seeds.push((*tmp.get(i).unwrap(), *tmp.get(i + 1).unwrap()));
            }
        } else if line.len() > 0 {
            if line.contains(":") {
                curr_key = line.split_once(" ")
                    .map(|(s1, s2)| String::from(s1))
                    .unwrap();
                maps.entry(curr_key.clone()).or_insert(HashMap::new());
                maps_order.push(curr_key.clone());
            } else {
                let num = String::from(line).to_vector();
                let right_start: &i64 = num.get(0).unwrap();
                let left_start: &i64 = num.get(1).unwrap();
                let range = *num.get(2).unwrap() - 1;
                let mut curr_map: &mut HashMap<(i64, i64), (i64, i64)> = maps.get_mut(&curr_key).unwrap();
                curr_map.entry((*left_start, *left_start + range))
                    .or_insert((*right_start, *right_start + range));
            }
        }
    }
    let mut groups: HashMap<String, Vec<(i64, i64)>> = HashMap::new();

    let mut new_group_start = 0;
    let mut new_group_end = 0;
    let mut curr_groups: Vec<(i64, i64)> = Vec::new();
    for (seed_start, range) in seeds {
        let seed_end = seed_start + range;
        curr_groups.push((seed_start, seed_end));
    }
    groups.insert(String::from("seed"), curr_groups.clone());
    for map in &maps_order {
        let mut new_groups: Vec<(i64, i64)> = Vec::new();
        let mut founded_groups: Vec<(i64, i64)> = Vec::new();
        for group in curr_groups.iter() {
            let curr_map = maps.get(map).unwrap().clone();
            for (from, to) in curr_map.iter() {
                let idx_fix_start = group.0 - from.0;
                let idx_fix_end = group.1 - from.0;
                if group.0 >= from.0 && group.1 <= from.1 {
                    new_group_start = to.0 + idx_fix_start;
                    new_group_end = to.0 + idx_fix_end;
                    new_groups.push((new_group_start, new_group_end));
                    founded_groups.push((group.0, group.1))
                } else if group.0 >= from.0 && group.0 <= from.1 {
                    new_group_start = to.0 + idx_fix_start;
                    new_group_end = to.1;
                    if new_group_start >= 0 {
                        new_groups.push((new_group_start, new_group_end));
                        founded_groups.push((group.0, from.1))
                    }
                } else if group.1 <= from.1 && group.1 >= from.0 {
                    new_group_start = to.0;
                    new_group_end = to.0 + idx_fix_end;
                    if new_group_end >= 0 {
                        new_groups.push((new_group_start, new_group_end));
                        founded_groups.push((from.0, group.1))
                    }
                }
                // println!("{:?}", curr_map);
            }
        }
        for group in curr_groups.iter() {
            let mut not_found_group = group.clone();
            for founded_group in founded_groups.iter() {
                if not_found_group.0 >= founded_group.0 && not_found_group.1 <= founded_group.1 {
                    not_found_group = (0,0);
                } else if not_found_group.0 >= founded_group.0 && not_found_group.0 <= founded_group.1 {
                    not_found_group = (founded_group.1+1,not_found_group.1);
                } else if not_found_group.1 <= founded_group.1 && not_found_group.1 >= founded_group.0 {
                    not_found_group = (not_found_group.0,founded_group.0-1);
                }
            }
            if not_found_group.0 < not_found_group.1 {
                new_groups.push(not_found_group.clone());
                founded_groups.push(not_found_group.clone());
            }
        }
        // println!("{:?}", founded_groups);
        // println!("{:?}", founded_groups);
        if new_groups.len() <= 0 {
            new_groups = curr_groups.clone();
        }
        // println!("{:?}", groups);
        // println!("{:?}", founded_groups);
        groups.insert(String::from(map), new_groups.clone());
        curr_groups = new_groups.clone();
    }
    let location = groups.get("humidity-to-location").unwrap()
        .iter().map(|v| v.0).min().unwrap();
    // println!("{:?}", maps);
    // println!("{:?}", groups);
    print!("Min location: {}", location);
    Some(location as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
