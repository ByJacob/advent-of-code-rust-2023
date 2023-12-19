use std::collections::HashSet;

advent_of_code::solution!(18);

fn add_point<T>(grid: &mut Vec<Vec<T>>, row: &mut i128, col: &mut i128, value: T, default_value: T)
where
    T: Clone,
{
    let col_len = grid[0].len();
    let row_len = grid.len();
    // Ensure the vector has enough rows\
    while grid.len() as i128 <= *row {
        grid.push(vec![default_value.clone(); col_len]);
    }

    while *row < 0 {
        grid.insert(0, vec![default_value.clone(); col_len]);
        *row += 1;
    }

    // Ensure the row has enough columns
    while grid[*row as usize].len() as i128 <= *col {
        for i in 0..row_len {
            grid[i].push(default_value.clone());
        }
    }

    while *col < 0 {
        for i in 0..row_len {
            grid.get_mut(i).unwrap().insert(0, default_value.clone());
        }
        *col += 1
    }

    // Assign the value to the specified position
    grid[*row as usize][*col as usize] = value;
}

fn get_neighbors(grid: &Vec<Vec<String>>, pos: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    let x = pos.0;
    let y = pos.1;
    let rows = grid.len();
    let cols = grid[0].len();

    for i in x.saturating_sub(1)..=(x + 1).min(rows - 1) {
        for j in y.saturating_sub(1)..=(y + 1).min(cols - 1) {
            if (i, j) != (x, y) && (i == x || j == y) {
                neighbors.push((i, j));
            }
        }
    }

    neighbors
}

struct Point {
    x: f64,
    y: f64,
}

struct Vector {
    start: Point,
    end: Point,
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut vectors: Vec<(i128, i128, i128, i128)> = vec![];

    let mut last_point = (0i128, 0i128);

    // let input = r#"
    //     R 3 X
    //     D 3 X
    //     L 3 X
    //     U 3 X
    // "#.trim();

    for line in input.lines() {
        let line = line.trim();
        let parts: Vec<&str> = line.split(" ").collect();
        let direct = parts.get(0).unwrap().clone();
        let distance = parts.get(1).unwrap().clone().parse::<i128>().unwrap();
        let color = parts
            .get(2)
            .unwrap()
            .clone()
            .trim_matches('(')
            .trim_matches(')');
        let mut new_point = match direct {
            "R" => (last_point.0, last_point.1 + distance),
            "D" => (last_point.0 + distance, last_point.1),
            "L" => (last_point.0, last_point.1 - distance),
            "U" => (last_point.0 - distance, last_point.1),
            _ => {
                panic!("Unknown direction")
            }
        };

        vectors.push((last_point.0, last_point.1, new_point.0, new_point.1));

        if last_point.0 != new_point.0 && last_point.1 != new_point.1 {
            panic!("SOME ERROR");
        }

        last_point = new_point;
        // add_point(&mut points, &mut last_point.0, &mut last_point.1, String::from(color), String::from(""));
    }

    let min_row = vectors
        .iter()
        .map(|(px, py, nx, ny)| px.min(nx))
        .min()
        .unwrap()
        .clone();
    let min_col = vectors
        .iter()
        .map(|(px, py, nx, ny)| py.min(ny))
        .min()
        .unwrap()
        .clone();

    let max_row = vectors
        .iter()
        .map(|(px, py, nx, ny)| px.max(nx))
        .max()
        .unwrap()
        .clone();
    let max_col = vectors
        .iter()
        .map(|(px, py, nx, ny)| py.max(ny))
        .max()
        .unwrap()
        .clone();

    let mut sum_cells: u64 = 0;

    let mut display_points =
        vec![vec![0; (max_col - min_col + 1) as usize]; (max_row - min_row + 1) as usize];

    for v in &vectors {
        let dpx = v.0 + min_row.abs();
        let dpy = v.1 + min_col.abs();
        let dpx2 = v.2 + min_row.abs();
        let dpy2 = v.3 + min_col.abs();
        for x in dpx.min(dpx2)..=dpx2.max(dpx) {
            for y in dpy.min(dpy2)..=dpy2.max(dpy) {
                display_points[x as usize][y as usize] = 5;
            }
        }
    }

    for i in min_row..=max_row {
        let mut curr_vectors = vec![];
        let mut curr_lines = vec![];
        for v in &vectors {
            if v.0 != v.2 && v.1 != v.3 {
                panic!("SOME ERROR");
            }
            let mut px = v.0;
            let mut py = v.1;
            let mut nx = v.2;
            let mut ny = v.3;
            if px > nx {
                px = v.2;
                nx = v.0;
            }
            if py > ny {
                py = v.3;
                ny = v.1;
            }
            if px == nx && px == i {
                curr_vectors.push((px, py, nx, ny));
            } else if i > px && i < nx {
                curr_vectors.push((px, py, nx, ny));
            } else if i == px || i == nx {
                curr_lines.push((px, py, nx, ny));
            }
        }
        curr_vectors.sort_by(|a, b| (a.1, a.3).cmp(&(b.1, b.3)));
        curr_lines.sort_by(|a, b| (a.1, a.3).cmp(&(b.1, b.3)));

        let mut last_y = 0;
        let mut is_outside = true;
        let mut curr_add: u64 = 0;
        for (px, py, nx, ny) in &curr_vectors {
            if *py == *ny {
                if !is_outside {
                    curr_add += (*py - last_y) as u64;
                    for x in last_y + 1..=*py {
                        let dpx = i + min_row.abs();
                        let dpy = x + min_col.abs();
                        display_points[dpx as usize][dpy as usize] += 1;
                    }
                    last_y = *py;
                    is_outside = true;
                } else {
                    curr_add += 1;
                    let dpx = i + min_row.abs();
                    let dpy = *py + min_col.abs();
                    display_points[dpx as usize][dpy as usize] += 1;
                    last_y = *py;
                    is_outside = false;
                }
            } else {
                if is_outside {
                    curr_add += (*ny - *py + 1) as u64;
                    for y in *py..=*ny {
                        let dpx = i + min_row.abs();
                        let dpy = y + min_col.abs();
                        display_points[dpx as usize][dpy as usize] += 1;
                    }
                } else {
                    curr_add += (*ny - last_y) as u64;
                    for y in last_y + 1..=*ny {
                        let dpx = i + min_row.abs();
                        let dpy = y + min_col.abs();
                        display_points[dpx as usize][dpy as usize] += 1;
                    }
                }
                last_y = *ny;
                let lines_for_x: HashSet<i128> = curr_lines
                    .clone()
                    .iter()
                    .filter(|&l| l.1 == *py || l.1 == *ny || l.3 == *py || l.3 == *ny)
                    .flat_map(|(l0, l1, l2, l3)| [*l0, *l2])
                    .collect();
                let mut lines_for_x: Vec<i128> = lines_for_x.into_iter().collect();
                lines_for_x.sort();
                let curr_x_pos = lines_for_x.iter().position(|v| v == &i).unwrap();
                if lines_for_x.len() == 3 && curr_x_pos == 1 {
                    is_outside = !is_outside;
                }
            }
        }
        sum_cells += curr_add;
    }

    for i in 0..display_points.len() {
        for j in 0..display_points[0].len() {
            if display_points[i as usize][j as usize] > 0 {
                print!("{}", display_points[i as usize][j as usize])
            } else {
                print!(".")
            }
        }
        println!()
    }

    // let counts: usize = points.iter().map(|row| {
    //     row.iter().filter(|&s| s != &String::from("X")).count()
    // }).sum();
    // Some(counts)
    Some(sum_cells)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut vectors: Vec<(i128, i128, i128, i128)> = vec![];

    let mut last_point = (0i128, 0i128);

    // let input = r#"
    //     R 3 X
    //     D 3 X
    //     L 3 X
    //     U 3 X
    // "#.trim();

    for line in input.lines() {
        let line = line.trim();
        let parts: Vec<&str> = line.split(" ").collect();
        let direct = parts.get(0).unwrap().clone();
        let distance = parts.get(1).unwrap().clone().parse::<i128>().unwrap();
        let color = parts
            .get(2)
            .unwrap()
            .clone()
            .trim_matches('(')
            .trim_matches(')');
        let distance = i128::from_str_radix(&color[1..6], 16).unwrap();
        let direct = match color.chars().last().unwrap() {
            '0' => "R",
            '1' => "D",
            '2' => "L",
            '3' => "U",
            _ => {
                panic!("Unknown directio nfrom number")
            }
        };
        let mut new_point = match direct {
            "R" => (last_point.0, last_point.1 + distance),
            "D" => (last_point.0 + distance, last_point.1),
            "L" => (last_point.0, last_point.1 - distance),
            "U" => (last_point.0 - distance, last_point.1),
            _ => {
                panic!("Unknown direction")
            }
        };

        vectors.push((last_point.0, last_point.1, new_point.0, new_point.1));

        if last_point.0 != new_point.0 && last_point.1 != new_point.1 {
            panic!("SOME ERROR");
        }

        last_point = new_point;
        // add_point(&mut points, &mut last_point.0, &mut last_point.1, String::from(color), String::from(""));
    }

    let min_row = vectors
        .iter()
        .map(|(px, py, nx, ny)| px.min(nx))
        .min()
        .unwrap()
        .clone();
    let min_col = vectors
        .iter()
        .map(|(px, py, nx, ny)| py.min(ny))
        .min()
        .unwrap()
        .clone();

    let max_row = vectors
        .iter()
        .map(|(px, py, nx, ny)| px.max(nx))
        .max()
        .unwrap()
        .clone();
    let max_col = vectors
        .iter()
        .map(|(px, py, nx, ny)| py.max(ny))
        .max()
        .unwrap()
        .clone();

    let mut sum_cells: u64 = 0;

    for i in min_row..=max_row {
        let mut curr_vectors = vec![];
        let mut curr_lines = vec![];
        for v in &vectors {
            if v.0 != v.2 && v.1 != v.3 {
                panic!("SOME ERROR");
            }
            let mut px = v.0;
            let mut py = v.1;
            let mut nx = v.2;
            let mut ny = v.3;
            if px > nx {
                px = v.2;
                nx = v.0;
            }
            if py > ny {
                py = v.3;
                ny = v.1;
            }
            if px == nx && px == i {
                curr_vectors.push((px, py, nx, ny));
            } else if i > px && i < nx {
                curr_vectors.push((px, py, nx, ny));
            } else if i == px || i == nx {
                curr_lines.push((px, py, nx, ny));
            }
        }
        curr_vectors.sort_by(|a, b| (a.1, a.3).cmp(&(b.1, b.3)));
        curr_lines.sort_by(|a, b| (a.1, a.3).cmp(&(b.1, b.3)));

        let mut last_y = 0;
        let mut is_outside = true;
        let mut curr_add: u64 = 0;
        for (px, py, nx, ny) in &curr_vectors {
            if *py == *ny {
                if !is_outside {
                    curr_add += (*py - last_y) as u64;
                    last_y = *py;
                    is_outside = true;
                } else {
                    curr_add += 1;
                    last_y = *py;
                    is_outside = false;
                }
            } else {
                if is_outside {
                    curr_add += (*ny - *py + 1) as u64;
                } else {
                    curr_add += (*ny - last_y) as u64;
                }
                last_y = *ny;
                let lines_for_x: HashSet<i128> = curr_lines
                    .clone()
                    .iter()
                    .filter(|&l| l.1 == *py || l.1 == *ny || l.3 == *py || l.3 == *ny)
                    .flat_map(|(l0, l1, l2, l3)| [*l0, *l2])
                    .collect();
                let mut lines_for_x: Vec<i128> = lines_for_x.into_iter().collect();
                lines_for_x.sort();
                let curr_x_pos = lines_for_x.iter().position(|v| v == &i).unwrap();
                if lines_for_x.len() == 3 && curr_x_pos == 1 {
                    is_outside = !is_outside;
                }
            }
        }
        sum_cells += curr_add;
    }

    // let counts: usize = points.iter().map(|row| {
    //     row.iter().filter(|&s| s != &String::from("X")).count()
    // }).sum();
    // Some(counts)
    Some(sum_cells)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
