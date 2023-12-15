use std::collections::HashMap;
use indicatif::{ProgressBar, ProgressStyle};
advent_of_code::solution!(10);

struct Directions {
    left: bool,
    up: bool,
    right: bool,
    down: bool
}

fn define_pipes() -> HashMap<char, Directions>{
    let mut pipes_type: HashMap<char, Directions> = HashMap::new();
    pipes_type.insert('|', Directions {left: false, up: true, right: false, down: true});
    pipes_type.insert('-', Directions {left: true, up: false, right: true, down: false});
    pipes_type.insert('L', Directions {left: false, up: true, right: true, down: false});
    pipes_type.insert('J', Directions {left: true, up: true, right: false, down: false});
    pipes_type.insert('7', Directions {left: true, up: false, right: false, down: true});
    pipes_type.insert('F', Directions {left: false, up: false, right: true, down: true});
    pipes_type.insert('.', Directions {left: false, up: false, right: false, down: false});
    pipes_type.insert('S', Directions {left: true, up: true, right: true, down: true});
    pipes_type
}

fn find_char_in_multiline_string(multiline_string: &String, target_char: char) -> Option<(usize, usize)> {
    for (line_idx, line) in multiline_string.lines().enumerate() {
        if let Some(char_idx) = line.chars().position(|c| c == target_char) {
            return Some((line_idx, char_idx));
        }
    }
    None
}

fn find_chars_in_multiline_string(multiline_string: &String, target_char: char) -> Option<Vec<(usize, usize)>> {
    let mut result:Vec<(usize, usize)> = vec![];
    for (line_idx, line) in multiline_string.lines().enumerate() {
        for (char_idx, ch) in line.chars().enumerate() {
            if ch == target_char {
                result.push((line_idx, char_idx))
            }
        }
    }
    Some(result)
}

fn get_neighbors(grid: &String, pos: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    let x = pos.0;
    let y = pos.1;
    let rows = grid.lines().count();
    let cols = grid.lines().next().map_or(0, |line| line.len());

    for i in x.saturating_sub(1)..=(x + 1).min(rows - 1) {
        for j in y.saturating_sub(1)..=(y + 1).min(cols - 1) {
            if (i, j) != (x, y) && (i == x || j == y) {
                neighbors.push((i, j));
            }
        }
    }

    neighbors
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
pub fn part_one(input: &str) -> Option<u32> {
    let pipes = define_pipes();
    let file_content = String::from(input);
    let s_position = find_char_in_multiline_string(&file_content, 'S').unwrap();
    let mut founded_pipes: HashMap<(usize, usize), i32> = HashMap::new();
    let mut curr_distance = 0;
    founded_pipes.insert(s_position, 0);
    let mut is_next = true;
    let mut curr_objects = vec![s_position];

    while is_next {
        curr_distance += 1;
        let mut new_objects: Vec<(usize, usize)> = vec![];
        for curr_object in &curr_objects {
            let mut possible_positions: Vec<(usize, usize)> = get_neighbors(&file_content, curr_object);
            let curr_x = curr_object.0;
            let curr_y = curr_object.1;
            let curr_char = get_char_at_position(&file_content, curr_x, curr_y).unwrap();
            let curr_direction = pipes.get(&curr_char).unwrap();
            for (poss_x, poss_y) in possible_positions {
                if founded_pipes.contains_key(&(poss_x, poss_y)) {
                    continue;
                }
                let possible_char = get_char_at_position(&file_content, poss_x, poss_y).unwrap();
                let possible_direction = pipes.get(&possible_char).unwrap();
                if curr_x == poss_x && curr_y > poss_y {
                    if curr_direction.left && possible_direction.right {
                        new_objects.push((poss_x, poss_y));
                        founded_pipes.insert((poss_x, poss_y), curr_distance);
                    }
                }
                else if curr_x == poss_x && curr_y < poss_y {
                    if curr_direction.right && possible_direction.left {
                        new_objects.push((poss_x, poss_y));
                        founded_pipes.insert((poss_x, poss_y), curr_distance);
                    }
                }
                else if curr_y == poss_y && curr_x < poss_x {
                    if curr_direction.down && possible_direction.up {
                        new_objects.push((poss_x, poss_y));
                        founded_pipes.insert((poss_x, poss_y), curr_distance);
                    }
                }
                else if curr_y == poss_y && curr_x > poss_x {
                    if curr_direction.up && possible_direction.down {
                        new_objects.push((poss_x, poss_y));
                        founded_pipes.insert((poss_x, poss_y), curr_distance);
                    }
                }
            }

        }
        is_next = !new_objects.is_empty();
        curr_objects = new_objects;

    }
    let max_distance = founded_pipes.values().max().unwrap();
    let mut sorted_vec: Vec<_> = founded_pipes.clone().into_iter().collect();
    // Sort the vector by values (ascending order)
    sorted_vec.sort_by(|a, b| a.1.cmp(&b.1));
    // println!("{:?}", sorted_vec);
    println!("{:?}", max_distance);
    Some(*max_distance as u32)
}

fn replace_char_at_position(input_string: String, x: usize, y: usize, new_char: char) -> String {
    let line_len = input_string.lines().nth(x).unwrap().len()+2;
    let mut chars: Vec<char> = input_string.chars().collect();

    if let Some(index_to_replace) = input_string.char_indices().nth(x*line_len + y) {
        let index_to_replace = index_to_replace.0;
        chars[index_to_replace] = new_char;
    }

    chars.into_iter().collect()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut file_content = String::from(input);
    let pipes = define_pipes();

    let s_position = find_char_in_multiline_string(&file_content, 'S').unwrap();
    let mut founded_pipes: HashMap<(usize, usize), i32> = HashMap::new();
    let mut curr_distance = 0;
    founded_pipes.insert(s_position, 0);
    let mut is_next = true;
    let mut curr_objects = vec![s_position];

    while is_next {
        curr_distance += 1;
        let mut new_objects: Vec<(usize, usize)> = vec![];
        for curr_object in &curr_objects {
            let mut possible_positions: Vec<(usize, usize)> = get_neighbors(&file_content, curr_object);
            let curr_x = curr_object.0;
            let curr_y = curr_object.1;
            let curr_char = get_char_at_position(&file_content, curr_x, curr_y).unwrap();
            let curr_direction = pipes.get(&curr_char).unwrap();
            for (poss_x, poss_y) in possible_positions {
                if founded_pipes.contains_key(&(poss_x, poss_y)) {
                    continue;
                }
                let possible_char = get_char_at_position(&file_content, poss_x, poss_y).unwrap();
                let possible_direction = pipes.get(&possible_char).unwrap();
                if curr_x == poss_x && curr_y > poss_y {
                    if curr_direction.left && possible_direction.right {
                        new_objects.push((poss_x, poss_y));
                        founded_pipes.insert((poss_x, poss_y), curr_distance);
                    }
                }
                else if curr_x == poss_x && curr_y < poss_y {
                    if curr_direction.right && possible_direction.left {
                        new_objects.push((poss_x, poss_y));
                        founded_pipes.insert((poss_x, poss_y), curr_distance);
                    }
                }
                else if curr_y == poss_y && curr_x < poss_x {
                    if curr_direction.down && possible_direction.up {
                        new_objects.push((poss_x, poss_y));
                        founded_pipes.insert((poss_x, poss_y), curr_distance);
                    }
                }
                else if curr_y == poss_y && curr_x > poss_x {
                    if curr_direction.up && possible_direction.down {
                        new_objects.push((poss_x, poss_y));
                        founded_pipes.insert((poss_x, poss_y), curr_distance);
                    }
                }
            }

        }
        is_next = !new_objects.is_empty();
        curr_objects = new_objects;

    }

    let mut new_file_content: String = String::from("");
    let pb = ProgressBar::new(file_content.lines().count() as u64);
    pb.set_prefix("Increment data");
    // Customize the style of the progress bar
    pb.set_style(ProgressStyle::default_bar()
        .template("[{bar:40.cyan/blue}] {percent}% {pos}/{len} {prefix} {msg}").expect("REASON")
        .progress_chars("##-"));
    for x in 0..file_content.lines().count() {
        pb.inc(1);
        let mut new_lines = vec!["".to_string(), "".to_string(), "".to_string()];
        let curr_line_len = file_content.lines().nth(x).unwrap().len();
        for y in 0..curr_line_len {
            let mut new_ch: String = format!("...\r\n...\r\n...\r\n");
            let curr_ch = if !founded_pipes.contains_key(&(x,y)) {
                '.'
            } else {
                get_char_at_position(&file_content, x, y).unwrap()
            };
            let curr_pipe = pipes.get(&curr_ch).unwrap();
            new_ch = replace_char_at_position(new_ch, 1, 1, curr_ch);
            if curr_pipe.up {
                new_ch = replace_char_at_position(new_ch, 0, 1, '|');
            }
            if curr_pipe.down {
                new_ch = replace_char_at_position(new_ch, 2, 1, '|');
            }
            if curr_pipe.left {
                new_ch = replace_char_at_position(new_ch, 1, 0, '-');
            }
            if curr_pipe.right {
                new_ch = replace_char_at_position(new_ch, 1, 2, '-');
            }
            for i in 0..3 {
                if let Some(line) = new_lines.get_mut(i) {
                    *line += &String::from(new_ch.lines().nth(i).unwrap());
                }
            }
        }
        for n in new_lines {
            new_file_content += &n;
            new_file_content += "\r\n";
        }
    }
    let pb = ProgressBar::new((file_content.lines().count()*file_content.lines().nth(0).unwrap().len()) as u64);
    pb.set_prefix("Fill first outside dots");
    // Customize the style of the progress bar
    pb.set_style(ProgressStyle::default_bar()
        .template("[{bar:40.cyan/blue}] {percent}% {pos}/{len} {prefix} {msg}").expect("REASON")
        .progress_chars("##-"));
    let mut file_content = new_file_content;
    let mut points_to_expand: Vec<(usize, usize)> = vec![];
    for x in 0..file_content.lines().count() {
        let line_len = file_content.lines().nth(x).unwrap().len();
        for y in 0..line_len {
            pb.inc(1);
            if (x==0 || x==file_content.lines().count()-1) || (y==0 || y==line_len-1) {
                if let Some(ch) = get_char_at_position(&file_content, x, y) {
                    if ch == '.' {
                        file_content = replace_char_at_position(file_content, x, y, '0');
                        points_to_expand.push((x,y));
                    }
                }

            }
        }
    }
    let mut is_finish = false;

    let mut last_changes = find_chars_in_multiline_string(&file_content, '0').unwrap();

    while !is_finish {

        let mut dot_changes = 0;
        pb.set_prefix("Process dots");
        // Customize the style of the progress bar
        pb.set_style(ProgressStyle::default_bar()
            .template("[{bar:40.cyan/blue}] {percent}% {pos}/{len} {prefix} {msg}").expect("REASON")
            .progress_chars("##-"));

        let pb = ProgressBar::new(*(&last_changes.len()) as u64);
        let mut new_changes: Vec<(usize, usize)> = vec![];
        for f0 in last_changes {
            pb.inc(1);
            let possible_values = get_neighbors(&file_content, &f0);
            for (poss_x, poss_y) in possible_values {
                let poss_char = get_char_at_position(&file_content, poss_x, poss_y).unwrap();
                if poss_char == '.' {
                    dot_changes += 1;
                    file_content = replace_char_at_position(file_content, poss_x, poss_y, '0');
                    new_changes.push((poss_x, poss_y));
                }
            }
        }
        last_changes = new_changes;
        let curr_zeros = file_content.chars().filter(|c| c==&'0').count();
        println!("Changes dot:{}", dot_changes);
        println!("Zeros count:{}", curr_zeros);
        is_finish = dot_changes == 0;
    }

    let mut new_file_content = String::from("");
    for x in (1..file_content.lines().count()).step_by(3) {
        let curr_line_len = file_content.lines().nth(x).unwrap().len();
        for y in (1..curr_line_len).step_by(3) {
            let curr_ch = get_char_at_position(&file_content, x, y).unwrap();
            new_file_content += &String::from(curr_ch);
        }
        new_file_content += &String::from("\r\n");
    }

    let sum_dots = new_file_content.chars().filter(|c| c==&'.').count();
    // println!("{}", new_file_content);
    // println!("{}", sum_dots);
    println!("{}", sum_dots);
    Some(sum_dots as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one_b() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY,2));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two_b() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY,3));
        assert_eq!(result, Some(8));
    }
}
