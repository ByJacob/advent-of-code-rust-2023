advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u32> {
    let mut maps: Vec<Vec<String>> = vec![];
    let mut curr_map: Vec<String> = vec![];

    for line in input.lines() {
        if line.len() <= 0 {
            maps.push(curr_map.clone());
            curr_map = vec![];
        } else {
            curr_map.push(String::from(line));
        }
    }
    maps.push(curr_map.clone());
    let mut sum = 0;
    for map in &maps {
        let mut map = map.clone();
        for multiple in vec![100, 1] {
            let mut row_mirror_id = 0;
            for row_id in 0..map.len() {
                if row_id >= map.len() - 1 {
                    continue;
                }
                let mut curr_dif: usize = 0;
                let mut is_mirror = true;
                loop {
                    let elem1 = if row_id as i32 - curr_dif as i32 >= 0 {
                        map.get(row_id - curr_dif)
                    } else {
                        None
                    };
                    let elem2 = map.get(row_id + 1 + curr_dif);
                    match (elem1, elem2) {
                        (Some(elem_prev), Some(elem_next)) => {
                            for (char1, char2) in elem_prev.chars().zip(elem_next.chars()) {
                                if char1 != char2 {
                                    is_mirror = false;
                                    break;
                                }
                            }
                            curr_dif += 1
                        }
                        _ => {
                            break;
                        }
                    }
                }
                if is_mirror {
                    row_mirror_id = row_id + 1;
                    sum += row_mirror_id * multiple
                }
            }
            let mut new_map: Vec<String> = vec![];
            for col_id in 0..map.get(0).unwrap().len() {
                let mut new_line = String::from("");
                for row_id in 0..map.len() {
                    let curr_row = map.get(row_id).unwrap();
                    let curr_ch = curr_row.chars().into_iter().nth(col_id).unwrap();
                    new_line += &String::from(curr_ch);
                }
                new_map.push(new_line);
            }
            map = new_map.clone();
        }
    }
    println!("{:?}", sum);
    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut maps: Vec<Vec<String>> = vec![];
    let mut curr_map: Vec<String> = vec![];

    for line in input.lines() {
        if line.len() <= 0 {
            maps.push(curr_map.clone());
            curr_map = vec![];
        } else {
            curr_map.push(String::from(line));
        }
    }
    maps.push(curr_map.clone());
    let mut sum = 0;
    for map in &maps {
        let mut map = map.clone();
        for multiple in vec![100, 1] {
            let mut row_mirror_id = 0;
            for row_id in 0..map.len() {
                if row_id >= map.len() - 1 {
                    continue;
                }
                let mut curr_dif: usize = 0;
                let mut is_mirror = true;
                let mut broken_elements = 0;
                loop {
                    let elem1 = if row_id as i32 - curr_dif as i32 >= 0 {
                        map.get(row_id - curr_dif)
                    } else {
                        None
                    };
                    let elem2 = map.get(row_id + 1 + curr_dif);
                    match (elem1, elem2) {
                        (Some(elem_prev), Some(elem_next)) => {
                            for (char1, char2) in elem_prev.chars().zip(elem_next.chars()) {
                                if char1 != char2 {
                                    broken_elements += 1;
                                }
                            }
                            curr_dif += 1
                        }
                        _ => {
                            break;
                        }
                    }
                }
                if broken_elements == 1 {
                    row_mirror_id = row_id + 1;
                    sum += row_mirror_id * multiple
                }
            }
            let mut new_map: Vec<String> = vec![];
            for col_id in 0..map.get(0).unwrap().len() {
                let mut new_line = String::from("");
                for row_id in 0..map.len() {
                    let curr_row = map.get(row_id).unwrap();
                    let curr_ch = curr_row.chars().into_iter().nth(col_id).unwrap();
                    new_line += &String::from(curr_ch);
                }
                new_map.push(new_line);
            }
            map = new_map.clone();
        }
    }
    println!("{:?}", sum);
    Some(sum as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
