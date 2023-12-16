use std::collections::HashSet;
use std::ops::Deref;
advent_of_code::solution!(16);

fn get_char_at_point(grid: &str, point: &Point) -> Option<char> {
    get_char_at_position(grid, point.row, point.col)
}
fn get_char_at_position(grid: &str, row: usize, col: usize) -> Option<char> {
    let mut current_x = 0;

    for (line_idx, line) in grid.lines().enumerate() {
        if current_x == row {
            if let Some(char_at_position) = line.chars().nth(col) {
                return Some(char_at_position);
            }
        }

        current_x += 1;
    }

    None
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
struct Point {
    row: usize,
    col: usize
}

impl Deref for Point {
    type Target = Point;

    fn deref(&self) -> &Self::Target {
        // Choose which field to dereference to
        self
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

impl Deref for Direction {
    type Target = Direction;

    fn deref(&self) -> &Self::Target {
        self
    }
}

pub fn part_one(input: &str) -> Option<u32> {

    let row_count = input.lines().count();
    let col_count = input.lines().nth(0).unwrap().len();

    let mut last_points = vec![(Point {row: 0, col: 0}, Direction::RIGHT)];
    let mut light_points: Vec<(Point, Direction)> = vec![*last_points.get(0).unwrap()];
    loop {
        let mut next_points: Vec<(Point, Direction)> = vec![];

        for (point, direction) in last_points {
            let mut next_points2: Vec<(Point, Direction)> = vec![];
            if let Some(curr_ch) = get_char_at_point(input, &point) {
                calculate_next_points(row_count, col_count, &mut next_points2, &point, direction, curr_ch);
                for next_point in next_points2.clone().iter() {
                    if light_points.iter().any(|p| p.0 == next_point.0 && p.1 == next_point.1) {
                        let curr_len = next_points2.len();
                        next_points2.retain(|p| p.0 != next_point.0 && p.1 != next_point.1);
                        let after_len = next_points2.len();
                    } else {
                        light_points.push((*next_point.0, *next_point.1));
                    }
                }
            } else {
                panic!("Missing points {:?}", point);
            }
            next_points.extend(next_points2);
        }
        last_points = next_points;
        if last_points.len() <=0 {
            break;
        }
    }
    let light_values: Vec<Point> = light_points.clone().iter().map(|p| *p.0).collect();
    let mut light_points2: Vec<Point> = vec![];
    for i in 0..row_count {
        for j in 0..row_count {
            let curr_point = Point{row: i, col: j};
            if light_values.contains(&curr_point) {
                // print!("#");
                light_points2.push(curr_point);
            } else {
                // print!(".");
            }
        }
        // println!()
    }
    // println!("{:?}", light_points);
    Some(light_points2.len() as u32)
}

fn calculate_next_points(row_count: usize, col_count: usize, next_points: &mut Vec<(Point, Direction)>, point: &Point, direction: Direction, curr_ch: char) {
    match curr_ch {
        '|' => {
            match direction {
                Direction::RIGHT | Direction::LEFT => {
                    if point.row as i32 - 1 >= 0 {
                        next_points.push((Point { row: point.row - 1, col: point.col }, Direction::UP));
                    }
                    if point.row + 1 < row_count {
                        next_points.push((Point { row: point.row + 1, col: point.col }, Direction::DOWN));
                    }
                }
                Direction::DOWN => {
                    if point.row + 1 < row_count {
                        next_points.push((Point { row: point.row + 1, col: point.col }, Direction::DOWN));
                    }
                }
                Direction::UP => {
                    if point.row as i32 - 1 >= 0 {
                        next_points.push((Point { row: point.row - 1, col: point.col }, Direction::UP));
                    }
                }
            }
        }
        '-' => {
            match direction {
                Direction::DOWN | Direction::UP => {
                    if point.col as i32 - 1 >= 0 {
                        next_points.push((Point { row: point.row, col: point.col - 1 }, Direction::LEFT));
                    }
                    if point.col + 1 < col_count {
                        next_points.push((Point { row: point.row, col: point.col + 1 }, Direction::RIGHT));
                    }
                }
                Direction::LEFT => {
                    if point.col as i32 - 1 >= 0 {
                        next_points.push((Point { row: point.row, col: point.col - 1 }, Direction::LEFT));
                    }
                }
                Direction::RIGHT => {
                    if point.col + 1 < col_count {
                        next_points.push((Point { row: point.row, col: point.col + 1 }, Direction::RIGHT));
                    }
                }
            }
        }
        '/' => {
            match direction {
                Direction::RIGHT => {
                    if point.row as i32 - 1 >= 0 {
                        next_points.push((Point { row: point.row - 1, col: point.col }, Direction::UP));
                    }
                }
                Direction::LEFT => {
                    if point.row + 1 < row_count {
                        next_points.push((Point { row: point.row + 1, col: point.col }, Direction::DOWN));
                    }
                }
                Direction::UP => {
                    if point.col + 1 < col_count {
                        next_points.push((Point { row: point.row, col: point.col + 1 }, Direction::RIGHT));
                    }
                }
                Direction::DOWN => {
                    if point.col as i32 - 1 >= 0 {
                        next_points.push((Point { row: point.row, col: point.col - 1 }, Direction::LEFT));
                    }
                }
            }
        }
        '\\' => {
            match direction {
                Direction::LEFT => {
                    if point.row as i32 - 1 >= 0 {
                        next_points.push((Point { row: point.row - 1, col: point.col }, Direction::UP));
                    }
                }
                Direction::RIGHT => {
                    if point.row + 1 < row_count {
                        next_points.push((Point { row: point.row + 1, col: point.col }, Direction::DOWN));
                    }
                }
                Direction::DOWN => {
                    if point.col + 1 < col_count {
                        next_points.push((Point { row: point.row, col: point.col + 1 }, Direction::RIGHT));
                    }
                }
                Direction::UP => {
                    if point.col as i32 - 1 >= 0 {
                        next_points.push((Point { row: point.row, col: point.col - 1 }, Direction::LEFT));
                    }
                }
            }
        }
        '.' => {
            match direction {
                Direction::UP => {
                    if point.row as i32 - 1 >= 0 {
                        next_points.push((Point { row: point.row - 1, col: point.col }, Direction::UP));
                    }
                }
                Direction::DOWN => {
                    if point.row + 1 < row_count {
                        next_points.push((Point { row: point.row + 1, col: point.col }, Direction::DOWN));
                    }
                }
                Direction::RIGHT => {
                    if point.col + 1 < col_count {
                        next_points.push((Point { row: point.row, col: point.col + 1 }, Direction::RIGHT));
                    }
                }
                Direction::LEFT => {
                    if point.col as i32 - 1 >= 0 {
                        next_points.push((Point { row: point.row, col: point.col - 1 }, Direction::LEFT));
                    }
                }
            }
        }
        _ => { panic!("Unknown character {}", curr_ch); }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let row_count = input.lines().count();
    let col_count = input.lines().nth(0).unwrap().len();
    let mut max_sum: u32 = 0;
    for i in 0..row_count {
        for j in 0..row_count {
            let mut start_points = if i==0 && j == 0 {
                vec![(Point { row: 0, col: 0 }, Direction::RIGHT), (Point { row: 0, col: 0 }, Direction::DOWN)]
            } else if i == row_count-1 && j==0 {
                vec![(Point { row: row_count-1, col: 0 }, Direction::RIGHT), (Point { row: row_count-1, col: 0 }, Direction::UP)]
            } else if i == row_count-1 && j==col_count-1 {
                vec![(Point { row: row_count-1, col: col_count-1 }, Direction::LEFT), (Point { row: row_count-1, col: col_count-1 }, Direction::UP)]
            } else if i == 0 && j==col_count-1 {
                vec![(Point { row: 0, col: col_count-1 }, Direction::LEFT), (Point { row: 0, col: col_count-1 }, Direction::DOWN)]
            } else if i == 0 {
                vec![(Point { row: 0, col: j }, Direction::DOWN)]
            } else if i == row_count-1 {
                vec![(Point { row: row_count-1, col: j }, Direction::UP)]
            } else if j == 0 {
                vec![(Point { row: i, col: 0 }, Direction::LEFT)]
            } else if j == col_count-1 {
                vec![(Point { row: i, col: col_count-1 }, Direction::RIGHT)]
            } else {
                continue
            };
            for start_point in start_points {
                let mut last_points = vec![start_point];
                // println!("Test enter by row {} and col {}", i, j);
                let mut light_points: Vec<(Point, Direction)> = last_points.clone();
                loop {
                    let mut next_points: Vec<(Point, Direction)> = vec![];

                    for (point, direction) in last_points {
                        let mut next_points2: Vec<(Point, Direction)> = vec![];
                        if let Some(curr_ch) = get_char_at_point(input, &point) {
                            calculate_next_points(row_count, col_count, &mut next_points2, &point, direction, curr_ch);
                            for next_point in next_points2.clone().iter() {
                                if light_points.iter().any(|p| p.0 == next_point.0 && p.1 == next_point.1) {
                                    next_points2.retain(|p| p.0 != next_point.0 && p.1 != next_point.1);
                                } else {
                                    light_points.push((*next_point.0, *next_point.1));
                                }
                            }
                        } else {
                            panic!("Missing points {:?}", point);
                        }
                        next_points.extend(next_points2);
                    }
                    last_points = next_points;
                    if last_points.len() <= 0 {
                        break;
                    }
                }
                let unique_points: HashSet<Point> = light_points.iter().map(|(p, _)| *p).collect();
                if unique_points.len() as u32 > max_sum {
                    // println!("{:?}", unique_points);
                    max_sum = unique_points.len() as u32;
                }
            }
        }
    }

    println!("{:?}", max_sum);
    Some(max_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
