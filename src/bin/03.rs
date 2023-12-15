use std::collections::HashMap;
advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let number_pattern = regex::Regex::new(r"\d+").unwrap();

    let lines: Vec<String> = input.lines().map(|s| String::from(s)).collect();
    let mut sum = 0;
    for idx in 0..(lines.len()) {
        let cur_line = &lines[idx];
        let empty_str = &String::from("");
        let prev_line = if idx>0 {
            lines.get(idx-1).unwrap()
        } else {
            &empty_str
        };
        let next_line = if idx<lines.len()-1 {
            lines.get(idx+1).unwrap()
        } else {
            &empty_str
        };

        for m in number_pattern.find_iter(cur_line) {
            let num: i32 = m.as_str().parse().expect("Excpect found number");
            let start = m.start();
            let end = m.end();
            let mut have_char = false;
            let start_search = ((start as i32)-1).max(0) as usize;
            let end_search = (end+1).min(cur_line.len());
            for line in [prev_line, cur_line, next_line] {
                if line.len() > 0 {
                    for ch in line[start_search..end_search].chars() {
                        if !ch.is_numeric() && ch != '.' {
                            have_char = true;
                            break;
                        }
                    }
                }
                if have_char {
                    break;
                }
            }
            if have_char {
                sum += num;
            }

        }
    }
    print!("Sum gameID: {}", sum);
    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let number_pattern = regex::Regex::new(r"\d+").unwrap();

    let lines: Vec<String> = input.lines().map(|s| String::from(s)).collect();
    let mut stars: HashMap<(usize, usize), Vec<i32>> = HashMap::new();
    let mut sum = 0;
    for idx in 0..(lines.len()) {
        let cur_line = &lines[idx];
        let empty_str = &String::from("");
        let prev_line = if idx>0 {
            lines.get(idx-1).unwrap()
        } else {
            &empty_str
        };
        let next_line = if idx<lines.len()-1 {
            lines.get(idx+1).unwrap()
        } else {
            &empty_str
        };

        for m in number_pattern.find_iter(cur_line) {
            let num: i32 = m.as_str().parse().expect("Excpect found number");
            let start = m.start();
            let end = m.end();
            let start_search = ((start as i32)-1).max(0) as usize;
            let end_search = (end+1).min(cur_line.len());
            for (idx_fix, line) in [(-1 as i32, prev_line), (0, cur_line), (1, next_line)] {
                if line.len() > 0 {
                    for (idx_ch, ch) in line[start_search..end_search].chars().enumerate() {
                        if ch == '*' {
                            let hash_key = ((idx as i32 + idx_fix) as usize, start_search+idx_ch);
                            stars.entry(hash_key)
                                .or_insert(Vec::new()).push(num);
                        }
                    }
                }
            }

        }
    }
    let stars_2: HashMap<(usize, usize), Vec<i32>> = stars.iter()
        .filter(|(_, value)| value.len()==2)
        .map(|(k,v)| (k.clone(), v.clone()))
        .collect();
    stars_2.iter().for_each(|(k,v)| {
        sum += v.get(0).expect("Element1")*v.get(1).expect("Element2")
    });
    print!("Sum gameID: {}", sum);
    Some(sum as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
