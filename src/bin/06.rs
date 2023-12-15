use std::fmt::Debug;
use std::str::FromStr;
advent_of_code::solution!(6);

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

fn transform_line(line: Option<&String>) -> Vec<i32> {
    line.map(|l| l.split_once(":").unwrap())
        .map(|f| f.1)
        .map(|f| String::from(f))
        .unwrap().split_whitespace().into_iter()
        .map(|f| f.parse().unwrap()).collect()
}

fn transform_line2(line: Option<&String>) -> i64 {
    line.map(|l| l.split_once(":").unwrap())
        .map(|f| f.1)
        .map(|f| String::from(f))
        .unwrap().replace(" ","").parse().unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<String> = input.lines().into_iter().map(|f| String::from(f)).collect();
    let times: Vec<i32> = transform_line(lines.get(0));
    let distance: Vec<i32> = transform_line(lines.get(1));

    let mut sum = 1;
    for i in 0..times.len() {
        let curr_time = *times.get(i).unwrap();
        let curr_distance = *distance.get(i).unwrap();

        let mut win_case_count = 0;

        for hold_ms in 0..curr_time {
            let calc_distance = (curr_time-hold_ms)*hold_ms;
            if calc_distance>curr_distance {
                win_case_count += 1;
            }
        }
        sum *= win_case_count;
    }

    // println!("{:?}", times);
    // println!("{:?}", distance);
    println!("{:?}", sum);
    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<String> = input.lines().into_iter().map(|f| String::from(f)).collect();
    let time = transform_line2(lines.get(0));
    let distance = transform_line2(lines.get(1));

    let mut sum = 1;

    let mut win_case_count = 0;

    for hold_ms in 0..time {
        let calc_distance = (time-hold_ms)*hold_ms;
        if calc_distance>distance {
            win_case_count += 1;
        }
    }
    sum *= win_case_count;


    // println!("{:?}", time);
    // println!("{:?}", distance);
    println!("{:?}", sum);
    Some(sum as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
