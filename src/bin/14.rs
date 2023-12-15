use indicatif::{ProgressBar, ProgressStyle};
advent_of_code::solution!(14);

fn rotate_text(text: &str, direction: &str) -> String {
    let lines: Vec<&str> = text.lines().collect();
    let rows = lines.len();
    let cols = lines.iter().map(|line| line.chars().count()).max().unwrap_or(0);

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

    let rotated_lines: Vec<String> = rotated_matrix.iter().map(|row| row.iter().collect()).collect();
    rotated_lines.join("\n")
}

pub fn part_one(input: &str) -> Option<u32> {
    let file_content = rotate_text(input, "left");

    let mut lines_after_move:Vec<String> = vec![];

    for line in file_content.lines() {
        let mut tmp_line = String::from(line.chars().collect::<String>());
        for idx in 0..tmp_line.len() {
            let curr_ch = tmp_line.chars().nth(idx).unwrap();
            if curr_ch == 'O' {
                let mut diff = 1;
                loop {
                    let prev_ch = if idx as i32 - diff >=0 {
                        tmp_line.chars().nth(idx-diff as usize)
                    } else {
                        None
                    };
                    if let Some(prev_ch) = prev_ch {
                        let prev_idx = idx - diff as usize;
                        if prev_ch == '.' {
                            tmp_line = tmp_line.chars().enumerate().map(|(idx_c, c)| {
                                if idx_c == prev_idx+1 {
                                    '.'
                                } else if idx_c == prev_idx {
                                    'O'
                                } else {
                                    c
                                }
                            }).collect::<String>();
                            diff += 1;
                        }
                        else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
        lines_after_move.push(tmp_line);
    }
    let file_content = rotate_text(lines_after_move.join("\r\n").as_str(), "right");
    let mut sum:i32 = 0;
    for (idx, line) in file_content.lines().rev().enumerate() {
        let count_o = line.chars().filter(|c| c==&'O').count() as i32;
        sum += (idx as i32+1)*count_o
    }
    // println!("{}", file_content);
    println!("{}", sum);
    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let file_content = rotate_text(input, "left");
    let mut file_content = rotate_text(file_content.as_str(), "left");

    let all_it: u64 = 1000000000;
    let progress = ProgressBar::new(all_it*4);

    progress.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} ({percent}%) Rotates").expect("REASON")
            .progress_chars("=> "),
    );


    let mut all_variants: Vec<String> = vec![];
    for i in 0..all_it {
        for j in 0..4 {
            progress.inc(1);
            let mut lines_after_move:Vec<String> = vec![];
            file_content = rotate_text(file_content.as_str(), "right");
            for line in file_content.lines() {
                let mut tmp_line = String::from(line.chars().collect::<String>());
                for idx in 0..tmp_line.len() {
                    let curr_ch = tmp_line.chars().nth(idx).unwrap();
                    if curr_ch == 'O' {
                        let mut diff = 1;
                        loop {
                            let prev_ch = if idx as i32 - diff >=0 {
                                tmp_line.chars().nth(idx-diff as usize)
                            } else {
                                None
                            };
                            if let Some(prev_ch) = prev_ch {
                                let prev_idx = idx - diff as usize;
                                if prev_ch == '.' {
                                    tmp_line = tmp_line.chars().enumerate().map(|(idx_c, c)| {
                                        if idx_c == prev_idx+1 {
                                            '.'
                                        } else if idx_c == prev_idx {
                                            'O'
                                        } else {
                                            c
                                        }
                                    }).collect::<String>();
                                    diff += 1;
                                }
                                else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                    }
                }
                lines_after_move.push(tmp_line);
            }
            file_content = lines_after_move.join("\r\n")
        }

        if !all_variants.contains(&file_content) {
            all_variants.push(file_content.clone());
        } else {
            let start_repeat = all_variants.iter().position(|x| x==&file_content).unwrap();
            for i in 0..start_repeat{
                all_variants.remove(0);
            }
            let after_cycles = (all_it-1-start_repeat as u64)%(all_variants.len() as u64);
            // for p in &all_variants {
            //     let mut sum:i32 = 0;
            //     for (idx, line) in p.lines().rev().enumerate() {
            //         let count_o = line.chars().filter(|c| c==&'O').count() as i32;
            //         sum += (idx as i32+1)*count_o
            //     }
            //     println!("{}", sum);
            // }
            file_content = all_variants.get(after_cycles as usize).unwrap().to_string();
            // println!("{}", after_cycles);
            break;
        }
    }

    let file_content = rotate_text(file_content.as_str(), "right");
    let file_content = rotate_text(file_content.as_str(), "right");

    let mut sum:i32 = 0;
    for (idx, line) in file_content.lines().rev().enumerate() {
        let count_o = line.chars().filter(|c| c==&'O').count() as i32;
        sum += (idx as i32+1)*count_o
    }
    // println!("{}", file_content);
    println!();
    println!("{}", sum);
    Some(sum as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
