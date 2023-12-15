advent_of_code::solution!(9);

trait SplitToVector {
    fn to_vector(self) -> Vec<i32>;
}

impl SplitToVector for String {
    fn to_vector(self) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        self.split_whitespace().for_each(|number| result.push(number.parse().unwrap()));
        result
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let first_line = String::from(line).to_vector();
        let mut process_lines: Vec<Vec<i32>> = vec![first_line];
        let mut all_zeros = false;

        while !all_zeros {
            let mut new_line:Vec<i32> = vec![];
            let last_line = process_lines.last().unwrap();
            for i in 0..(last_line.len()-1) {
                let next_num = last_line.get(i+1).unwrap();
                let cur_num = last_line.get(i).unwrap();;
                new_line.push(*next_num-*cur_num)
            }
            all_zeros = new_line.iter().all(|v| v == &0);
            process_lines.push(new_line);
        }

        let len_process_lines = process_lines.len();
        for idx in (0..len_process_lines).rev() {
            if idx == len_process_lines-1 {
                process_lines.get_mut(idx).unwrap().push(0);
            } else {
                let prev_line = process_lines.get_mut(idx+1).unwrap();
                let prev_line_num = *prev_line.last().unwrap();
                let curr_line = process_lines.get_mut(idx).unwrap();
                let new_number = prev_line_num + curr_line.last().unwrap();
                curr_line.push(new_number);
            }
        }
        sum += process_lines.first().unwrap().last().unwrap();

    }
    println!("Sum for part = {:?}", sum);
    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let first_line = String::from(line).to_vector();
        let mut process_lines: Vec<Vec<i32>> = vec![first_line];
        let mut all_zeros = false;

        while !all_zeros {
            let mut new_line:Vec<i32> = vec![];
            let last_line = process_lines.last().unwrap();
            for i in 0..(last_line.len()-1) {
                let next_num = last_line.get(i+1).unwrap();
                let cur_num = last_line.get(i).unwrap();;
                new_line.push(*next_num-*cur_num)
            }
            all_zeros = new_line.iter().all(|v| v == &0);
            process_lines.push(new_line);
        }

        let len_process_lines = process_lines.len();
        for idx in (0..len_process_lines).rev() {
            if idx == len_process_lines-1 {
                process_lines.get_mut(idx).unwrap().insert(0,0);
            } else {
                let prev_line = process_lines.get_mut(idx+1).unwrap();
                let prev_line_num = *prev_line.first().unwrap();
                let curr_line = process_lines.get_mut(idx).unwrap();
                let new_number = curr_line.first().unwrap() - prev_line_num;
                curr_line.insert(0, new_number);
            }
        }
        sum += process_lines.first().unwrap().first().unwrap();

    }
    println!("Sum for part = {:?}", sum);
    Some(sum as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
