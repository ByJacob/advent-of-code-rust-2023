advent_of_code::solution!(11);

struct Directions {
    left: bool,
    up: bool,
    right: bool,
    down: bool,
}

fn rotate_text(text: &str, direction: &str) -> String {
    let lines: Vec<&str> = text.lines().collect();
    let rows = lines.len();
    let cols = lines
        .iter()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0);

    let mut rotated_matrix = vec![vec![' '; rows]; cols];

    for (i, line) in lines.iter().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            match direction {
                "right" => rotated_matrix[j][rows - 1 - i] = ch,
                "left" => rotated_matrix[cols - 1 - j][i] = ch,
                _ => panic!("Invalid direction. Use 'right' or 'left'."),
            }
        }
    }

    let rotated_lines: Vec<String> = rotated_matrix
        .iter()
        .map(|row| row.iter().collect())
        .collect();
    rotated_lines.join("\n")
}

fn expand_galaxy(text: String, fill_ch: &str) -> String {
    let mut result = text;
    let rotates: Vec<&str> = vec!["right", "left"];
    for i in 0..2 {
        let mut new_text = "".to_string();
        for line in result.lines() {
            if line
                .chars()
                .into_iter()
                .all(|s| s == '.' || s == fill_ch.chars().into_iter().nth(0).unwrap())
            {
                let new_line = fill_ch.to_string().repeat(line.len());
                new_text = String::from(new_text + &new_line + "\r\n");
            } else {
                new_text = String::from(new_text + line + "\r\n");
            }
        }
        let rotated_text = rotate_text(&new_text, rotates.get(i).unwrap().trim());
        result = rotated_text;
    }
    result
}

fn find_chars_in_multiline_string(
    multiline_string: &String,
    target_char: char,
) -> Option<Vec<(usize, usize)>> {
    let mut result: Vec<(usize, usize)> = vec![];
    for (line_idx, line) in multiline_string.lines().enumerate() {
        for (char_idx, ch) in line.chars().enumerate() {
            if ch == target_char {
                result.push((line_idx, char_idx))
            }
        }
    }
    Some(result)
}

fn get_char_at_position(grid: &String, x: usize, y: usize) -> Option<char> {
    let mut current_x = 0;

    for (line_idx, line) in grid.lines().enumerate() {
        if current_x == x {
            if let Some(char_at_position) = line.chars().nth(y) {
                return Some(char_at_position);
            }
        }

        current_x += 1;
    }

    None
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

impl Pos {
    fn successors(&self) -> Vec<Pos> {
        let &Pos(x, y) = self;
        vec![Pos(x + 1, y), Pos(x, y + 1), Pos(x - 1, y), Pos(x, y - 1)]
    }
}
pub fn part_one(input: &str) -> Option<u64> {
    let file_content2 = expand_galaxy(String::from(input).clone(), "X");
    // println!("{}", file_content2);
    let galaxy_positions = find_chars_in_multiline_string(&file_content2, '#').unwrap();
    let mut sum = 0;
    for i in 0..galaxy_positions.len() {
        for j in i + 1..galaxy_positions.len() {
            // println!("Compare {}-{} from {}", i, j, galaxy_positions.len());
            let start = Pos(
                galaxy_positions.get(i).unwrap().0 as i32,
                galaxy_positions.get(i).unwrap().1 as i32,
            );
            let end = Pos(
                galaxy_positions.get(j).unwrap().0 as i32,
                galaxy_positions.get(j).unwrap().1 as i32,
            );

            let mut paths1: Vec<Pos> = vec![];
            for k in end.0.min(start.0)..=end.0.max(start.0) {
                paths1.push(Pos(k, end.1.min(start.1)));
            }
            let curr_last_point = paths1.pop().unwrap();
            for k in end.1.min(start.1)..=end.1.max(start.1) {
                paths1.push(Pos(curr_last_point.0, k));
            }
            let mut paths2: Vec<Pos> = vec![];
            for k in end.1.min(start.1)..=end.1.max(start.1) {
                paths2.push(Pos(end.0.min(start.0), k));
            }
            let curr_last_point = paths2.pop().unwrap();
            for k in end.0.min(start.0)..=end.0.max(start.0) {
                paths2.push(Pos(k, curr_last_point.1));
            }

            // let result = bfs(&start, |p| p.successors(), |p| *p == end).unwrap();
            let result2 = (end.0 - start.0).abs() + (end.1 - start.1).abs();
            paths1.remove(0);
            paths2.remove(0);
            let paths = vec![paths1.clone(), paths2.clone()];
            let mut sum_paths: Vec<i32> = vec![];
            for path in paths {
                let mut sum_path = 0;
                for pos in path {
                    let curr_char =
                        get_char_at_position(&file_content2, pos.0 as usize, pos.1 as usize)
                            .unwrap();
                    if curr_char == 'X' {
                        sum_path += 2
                    } else {
                        sum_path += 1
                    }
                }
                sum_paths.push(sum_path);
            }
            let result3 = *sum_paths.iter().min().unwrap();
            // println!("{:?}", paths1);
            // println!("{:?}", paths2);
            // println!("{:?}", result2);
            // println!("{:?}", result3);
            sum += result3;
        }
    }

    println!("{:?}", sum);
    Some(sum as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    part_two_p(input, 1000000)
}
pub fn part_two_p(input: &str, multiple_times: i32) -> Option<u64> {
    let file_content2 = expand_galaxy(String::from(input).clone(), "X");
    // println!("{}", file_content2);
    let galaxy_positions = find_chars_in_multiline_string(&file_content2, '#').unwrap();
    let mut sum = 0;
    for i in 0..galaxy_positions.len() {
        for j in i + 1..galaxy_positions.len() {
            // println!("Compare {}-{} from {}", i, j, galaxy_positions.len());
            let start = Pos(
                galaxy_positions.get(i).unwrap().0 as i32,
                galaxy_positions.get(i).unwrap().1 as i32,
            );
            let end = Pos(
                galaxy_positions.get(j).unwrap().0 as i32,
                galaxy_positions.get(j).unwrap().1 as i32,
            );

            let mut paths1: Vec<Pos> = vec![];
            for k in end.0.min(start.0)..=end.0.max(start.0) {
                paths1.push(Pos(k, end.1.min(start.1)));
            }
            let curr_last_point = paths1.pop().unwrap();
            for k in end.1.min(start.1)..=end.1.max(start.1) {
                paths1.push(Pos(curr_last_point.0, k));
            }
            let mut paths2: Vec<Pos> = vec![];
            for k in end.1.min(start.1)..=end.1.max(start.1) {
                paths2.push(Pos(end.0.min(start.0), k));
            }
            let curr_last_point = paths2.pop().unwrap();
            for k in end.0.min(start.0)..=end.0.max(start.0) {
                paths2.push(Pos(k, curr_last_point.1));
            }

            // let result = bfs(&start, |p| p.successors(), |p| *p == end).unwrap();
            let result2 = (end.0 - start.0).abs() + (end.1 - start.1).abs();
            paths1.remove(0);
            paths2.remove(0);
            let paths = vec![paths1.clone(), paths2.clone()];
            let mut sum_paths: Vec<i64> = vec![];
            for path in paths {
                let mut sum_path = 0;
                for pos in path {
                    let curr_char =
                        get_char_at_position(&file_content2, pos.0 as usize, pos.1 as usize)
                            .unwrap();
                    if curr_char == 'X' {
                        sum_path += multiple_times
                    } else {
                        sum_path += 1
                    }
                }
                sum_paths.push(sum_path as i64);
            }
            let result3 = *sum_paths.iter().min().unwrap();
            // println!("{:?}", paths1);
            // println!("{:?}", paths2);
            // println!("{:?}", result2);
            // println!("{:?}", result3);
            sum += result3;
        }
    }

    println!("{:?}", sum);
    Some(sum as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two_p(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, Some(1030));
    }

    #[test]
    fn test_part_two_b() {
        let result = part_two_p(&advent_of_code::template::read_file("examples", DAY), 100);
        assert_eq!(result, Some(8410));
    }
}
