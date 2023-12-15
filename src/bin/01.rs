use std::collections::HashMap;
advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let number_pattern = regex::Regex::new(r"\d").unwrap();

    let mut sum = 0;

    for line in input.lines() {
        let numbers: Vec<i32> = number_pattern
            .find_iter(line)
            .map(|m| m.as_str())
            .map(|e| e.parse().expect("Cannot parse str to number"))
            .collect();
        sum += numbers.first().expect("Number is required") * 10;
        sum += numbers.last().expect("Number is required");
    }
    println!("Result: {}", sum);
    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let word_to_number: HashMap<&str, i32> = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]
    .iter()
    .cloned()
    .collect();

    let mut sum = 0;

    for line in input.lines() {
        let mut numbers: Vec<(i32, i32)> = vec![];
        for (word, number) in word_to_number.iter() {
            let mut start_index: usize = 0;
            while let Some(index) = line[start_index..].find(word) {
                // Adjust the index to be relative to the original string
                let absolute_index = start_index + index;

                // Store the starting index
                numbers.push((absolute_index as i32, *number));

                // Move the start_index to the next position after the current match
                start_index = absolute_index + 1;
            }
        }
        numbers.sort_by(|a, b| a.0.cmp(&b.0));
        let first = numbers.first().expect("Element").1;
        let last = numbers.last().expect("Element").1;
        sum += first * 10;
        sum += last;
    }
    println!("Result: {}", sum);
    Some(sum as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
