use std::collections::HashMap;
advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u32> {
    let parts: Vec<String> = input.split(",").map(|s| String::from(s)).collect();
    let mut sum = 0;
    for part in parts {
        let mut number: i64 = 0;
        for c in part.chars() {
            number += c as i64;
            number *= 17;
            number %= 256;
        }
        println!("Part number {}", number);
        sum += number;
    }

    println!("Sum {}", sum);
    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let parts: Vec<String> = input.split(",").map(|s| String::from(s)).collect();
    let mut boxes: HashMap<u16, Vec<String>> = HashMap::new();

    // Populate the HashMap with keys from 0 to 255 and empty Vec as values
    for i in 0..=255 {
        boxes.insert(i, Vec::new());
    }
    for part in parts {
        let mut number: i64 = 0;
        for c in part.chars() {
            if c != '=' && c != '-' {
                number += c as i64;
                number *= 17;
                number %= 256;
            } else if c == '=' {
                let box_obj = part.split_once("=").unwrap();
                let num = number as u16;
                boxes.entry(num).or_insert(vec![]);
                if let Some(boxx) = boxes.get_mut(&num) {
                    if let Some(position) = boxx.into_iter().position(|b| b.contains(box_obj.0)) {
                        boxx.remove(position);
                        boxx.insert(position, String::from(box_obj.0) + " " + &String::from(box_obj.1))
                    } else {
                        boxx.push(String::from(box_obj.0) + " " + &String::from(box_obj.1))
                    }
                } else {
                    panic!("CANT find box with key {}", num)
                }
            } else {
                let box_obj = part.split("-").into_iter().nth(0).unwrap();
                let num = number as u16;
                if let Some(boxx) = boxes.get_mut(&num) {
                    if let Some(position) = boxx.into_iter().position(|b| b.contains(box_obj)) {
                        boxx.remove(position);
                    }
                }
            }
        }
        // println!("Part number {}", number);
        // sum += number;
    }
    let mut sum = 0;
    for i in 0..=255 {
        if let Some(boxx) = boxes.get(&i) {
            for (idx, slot) in boxx.into_iter().enumerate() {
                let focal_len: u64 = slot.trim().split(" ").last().unwrap().parse().unwrap();
                sum += (i+1) as u64 * (idx+1) as u64 * focal_len
            }
        } else {
            panic!("Cannt find box {}", i);
        }
    }

    println!("Sum: {}", sum);
    Some(sum as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
