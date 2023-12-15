use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::fmt::Debug;
use std::str::FromStr;
advent_of_code::solution!(12);

fn generate_strings(input: &str, groups: &Vec<i32>) -> Vec<String> {
    let hash_count = groups.iter().sum::<i32>() as usize;
    let hash_groups = groups.len();
    let mut result = Vec::new();
    let progress = ProgressBar::new(input.len() as u64);
    progress.set_style(
        ProgressStyle::default_bar()
            .template(
                "[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} ({percent}%) generate_strings",
            )
            .expect("REASON")
            .progress_chars("=> "),
    );
    generate_strings_recursive(
        input,
        0,
        String::new(),
        hash_count,
        hash_groups,
        groups,
        &mut result,
        &progress,
    );
    result
}

fn generate_strings_recursive(
    input: &str,
    index: usize,
    current: String,
    hash_count: usize,
    hash_groups: usize,
    groups: &Vec<i32>,
    result: &mut Vec<String>,
    progress: &ProgressBar,
) {
    if index == input.len() {
        if current.chars().filter(|c| c == &'#').count() != hash_count {
            return;
        }
        if current
            .split(".")
            .filter(|&s| s.contains("#"))
            .enumerate()
            .all(|(idx, p)| {
                let curr_group_count = groups.get(idx).unwrap();
                p.len() == *curr_group_count as usize
            })
        {
            result.push(current.clone());
        }
        return;
    }
    if current.chars().filter(|c| c == &'#').count() > hash_count {
        return;
    }
    let curr_hash_groups = current.split(".").filter(|&s| s.contains("#")).count();
    if curr_hash_groups > hash_groups {
        return;
    } else if curr_hash_groups == hash_groups {
        if current
            .split(".")
            .filter(|&s| s.contains("#"))
            .enumerate()
            .any(|(idx, p)| {
                let curr_group_count = groups.get(idx).unwrap();
                p.len() > *curr_group_count as usize
            })
        {
            return;
        }
    }
    let current_char = &input[index..index + 1];
    progress.set_position(index as u64 + 1);
    if current_char == "?" {
        let mut option1 = current.clone();
        option1.push('.');
        generate_strings_recursive(
            input,
            index + 1,
            option1,
            hash_count,
            hash_groups,
            groups,
            result,
            progress,
        );

        let mut option2 = current;
        option2.push('#');
        generate_strings_recursive(
            input,
            index + 1,
            option2,
            hash_count,
            hash_groups,
            groups,
            result,
            progress,
        );
    } else {
        let mut new_current = current;
        new_current.push_str(current_char);
        generate_strings_recursive(
            input,
            index + 1,
            new_current,
            hash_count,
            hash_groups,
            groups,
            result,
            progress,
        );
    }
}

trait SplitToVector<T: FromStr> {
    fn to_vector(self) -> Vec<T>
    where
        <T as FromStr>::Err: Debug;
}

impl<T: FromStr> SplitToVector<T> for String {
    fn to_vector(self) -> Vec<T>
    where
        <T as FromStr>::Err: Debug,
    {
        let mut result: Vec<T> = Vec::new();
        self.split(",")
            .for_each(|number| result.push(number.parse().unwrap()));
        result
    }
}

fn count_hashes(input: &str) -> Vec<i32> {
    let mut counts = Vec::new();
    let mut current_count = 0;

    for ch in input.chars() {
        if ch == '#' {
            current_count += 1;
        } else if current_count > 0 {
            counts.push(current_count);
            current_count = 0;
        }
    }

    if current_count > 0 {
        counts.push(current_count);
    }

    counts
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;

    let progress = ProgressBar::new(input.lines().into_iter().count() as u64);

    progress.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} ({percent}%)")
            .expect("REASON")
            .progress_chars("=> "),
    );

    for line in input.lines() {
        progress.inc(1);
        let (values, group) = line.split_once(" ").unwrap();
        let group: Vec<i32> = String::from(group).to_vector();
        let poss_values = generate_strings(values, &group);
        // let mut good_poss = 0;
        // for poss in &poss_values {
        //     let count = count_hashes(poss);
        //     if group == count {
        //         good_poss += 1;
        //     }
        // }
        sum += poss_values.len() as i32;
    }

    println!("Sum poss values {}", sum);
    Some(sum as u32)
}

fn replace_char_at_index(mut input_str: String, index: usize, new_char: char) -> String {
    input_str
        .chars()
        .enumerate()
        .map(|(i, c)| if i == index { new_char } else { c })
        .collect()
}

fn find_positions(input_string: &str, target_char: char) -> Vec<usize> {
    input_string
        .char_indices()
        .filter(|&(_, c)| c == target_char)
        .map(|(index, _)| index)
        .collect()
}

struct SpringPermutation {
    group: i32,
    amount: i32,
    permutations: i64,
}
struct SpringPermutationCount {
    group: i32,
    amount: i32,
}
fn find_groups(line: &str, groups: &Vec<i32>) -> i64 {
    let mut spring_permutations = vec![SpringPermutation {
        group: 0,
        amount: 0,
        permutations: 1,
    }];
    let mut spring_permutation_counts: HashMap<(i32, i32), i64> = HashMap::new();
    spring_permutation_counts.insert((0, 0), 1);
    let mut springs_checked = 0;
    for ch in line.chars() {
        if ch != '?' {
            spring_permutations = vec![];
            for (group_amount, permutations) in spring_permutation_counts.iter() {
                if ch == '#'
                    && group_amount.0 < groups.len() as i32
                    && group_amount.1 < *groups.get(group_amount.0 as usize).unwrap() as i32
                {
                    spring_permutations.push(SpringPermutation {
                        group: group_amount.0,
                        amount: group_amount.1 + 1,
                        permutations: *permutations,
                    });
                } else if ch == '.' && group_amount.1 == 0 {
                    spring_permutations.push(SpringPermutation {
                        group: group_amount.0,
                        amount: group_amount.1,
                        permutations: *permutations,
                    });
                } else if ch == '.'
                    && group_amount.0 < groups.len() as i32
                    && group_amount.1 == *groups.get(group_amount.0 as usize).unwrap() as i32
                {
                    spring_permutations.push(SpringPermutation {
                        group: group_amount.0 + 1,
                        amount: 0,
                        permutations: *permutations,
                    });
                }
            }
        } else {
            spring_permutations = vec![];
            for (group_amount, permutations) in spring_permutation_counts.iter() {
                if group_amount.0 < groups.len() as i32
                    && group_amount.1 < *groups.get(group_amount.0 as usize).unwrap() as i32
                {
                    spring_permutations.push(SpringPermutation {
                        group: group_amount.0,
                        amount: group_amount.1 + 1,
                        permutations: *permutations,
                    })
                }
                if group_amount.1 == 0 {
                    spring_permutations.push(SpringPermutation {
                        group: group_amount.0,
                        amount: group_amount.1,
                        permutations: *permutations,
                    })
                } else if group_amount.1 == *groups.get(group_amount.0 as usize).unwrap() as i32 {
                    spring_permutations.push(SpringPermutation {
                        group: group_amount.0 + 1,
                        amount: 0,
                        permutations: *permutations,
                    })
                }
            }
        }
        springs_checked += 1;
        let springs_left = line[springs_checked..].len() as i32;

        spring_permutations.retain(|element| {
            return if element.group > groups.len() as i32 {
                element.amount == 0
            } else {
                let sum: i32 = groups.iter().skip(element.group as usize).sum();
                springs_left + element.amount >= sum
            };
        });

        spring_permutation_counts.clear();

        for p in &spring_permutations {
            *spring_permutation_counts
                .entry((p.group, p.amount))
                .or_insert(0) += p.permutations;
        }
        // println!("{}", spring_permutations.iter().map(|p| p.permutations).sum::<i32>())
    }

    spring_permutations
        .iter()
        .map(|p| p.permutations)
        .sum::<i64>()
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum = 0;

    let progress = ProgressBar::new(input.lines().into_iter().count() as u64);

    progress.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} ({percent}%) ALl lines")
            .expect("REASON")
            .progress_chars("=> "),
    );

    for line in input.lines() {
        progress.inc(1);
        let (values, group) = line.split_once(" ").unwrap();
        let mut new_values = String::from(values);
        let mut new_group = String::from(group);
        for i in 0..4 {
            new_values += &String::from("?");
            new_values += &String::from(values);
            new_group += ",";
            new_group += &String::from(group);
        }
        let group2: Vec<i32> = String::from(new_group).to_vector();
        let mut current_vector: Vec<String> = Vec::new();
        let mut current_vector_len = 0;
        println!();
        let progress2 = ProgressBar::new(1000000000);
        progress2.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} ({percent}%) founded groups").expect("REASON")
                .progress_chars("=> "),
        );
        let current_vector_len = find_groups(&new_values, &group2);
        println!();
        // println!("{}", current_vector_len);
        // println!("{:?}", current_vector)
        // let poss_values = generate_strings(&new_values, &group);
        // sum += current_vector.len() as i32;
        sum += current_vector_len;
    }

    println!("Sum poss values {}", sum);
    Some(sum as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
