use std::collections::{HashMap, HashSet};
use std::fs;
advent_of_code::solution!(21);

fn find_chars_in_multiline_string(
    multiline_string: &str,
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

fn get_neighbors(
    grid: &str,
    pos: &(usize, usize),
) -> (Vec<(usize, usize)>, Vec<(usize, usize, isize, isize)>) {
    let mut neighbors = Vec::new();
    let mut outside_neighbors = Vec::new();
    let x = pos.0 as isize;
    let y = pos.1 as isize;
    let rows = grid.lines().count() as isize;
    let cols = grid.lines().next().map_or(0, |line| line.len()) as isize;

    for i in (x - 1)..=(x + 1) {
        for j in (y - 1)..=(y + 1) {
            if (i, j) != (x, y) && (i == x || j == y) {
                if i >= 0 && i < rows && j >= 0 && j < cols {
                    let curr_ch = get_char_at_position(grid, i as usize, j as usize).unwrap();
                    if curr_ch != '#' {
                        neighbors.push((i as usize, j as usize))
                    }
                    continue;
                }
                let mut i2 = i;
                let mut j2 = j;
                let mut di = 0;
                let mut dj = 0;
                if i < 0 {
                    i2 += rows;
                    di -= 1;
                }
                if i >= rows {
                    i2 -= rows;
                    di += 1;
                }
                if j < 0 {
                    j2 += cols;
                    dj -= 1;
                }
                if j >= cols {
                    j2 -= cols;
                    dj += 1;
                }
                if di != 0 && dj != 0 {
                    panic!("This is inpossible");
                }
                if let Some(curr_ch) = get_char_at_position(grid, i2 as usize, j2 as usize) {
                    if curr_ch != '#' {
                        outside_neighbors.push((i2 as usize, j2 as usize, di, dj))
                    }
                } else {
                    panic!("Problem with i2 and j2");
                }
            }
        }
    }

    (neighbors, outside_neighbors)
}

pub fn part_one(input: &str) -> Option<u128> {
    part_one_param(input, 64)
}

pub fn part_one_param(input: &str, target_count: u128) -> Option<u128> {
    let mut positions = find_chars_in_multiline_string(input, 'S').unwrap();
    let mut all_positions = vec![positions.clone()];

    for i in 0..target_count {
        let mut new_positions: HashSet<(usize, usize)> = HashSet::new();
        for position in &positions {
            new_positions.extend(get_neighbors(input, position).0);
        }
        positions = new_positions.iter().map(|e| (e.0, e.1)).collect();
        positions.sort_by(|a, b| a.1.cmp(&b.1));
        positions.sort_by(|a, b| a.0.cmp(&b.0));

        if let Some(position) = all_positions.iter().position(|p| p == &positions) {
            let last_idx = position + (target_count as usize - position) % 2;
            positions = all_positions.get(last_idx).unwrap().clone();
            break;
        }

        all_positions.push(positions.clone())
    }

    let result = positions.len() as u128;
    println!("{:?}", result);
    Some(result)
}

pub fn part_two(input: &str) -> Option<u128> {
    // DONT WORK :C
    part_two_param(input, 26501365)
}

pub fn part_two_param(input: &str, target_count: u128) -> Option<u128> {
    let mut start_positions = find_chars_in_multiline_string(input, 'S').unwrap();
    let mut all_positions_by_layer: HashMap<(isize, isize), Vec<HashSet<(usize, usize)>>> =
        HashMap::new();
    let mut layer_start_exist: HashMap<(isize, isize), u128> = HashMap::new();
    let mut full_layers: HashMap<(isize, isize), (Vec<u128>, u128)> = HashMap::new();
    let start_position_hash: HashSet<(usize, usize)> = start_positions.into_iter().collect();
    all_positions_by_layer.insert((0, 0), vec![start_position_hash]);
    layer_start_exist.insert((0, 0), 0);

    let interesting_layers_key = vec![
        (0isize, -3isize),
        (-1, -2),
        (-3, 0),
        (-1, 2),
        (0, 3),
        (1, 2),
        (3, 0),
        (1, -2),
        (0, -2),
    ];
    let mut interesting_vectors: HashMap<(isize, isize), (HashMap<usize, usize>, u128)> =
        HashMap::new();

    let mut diff_between_layers: HashMap<isize, u128> = HashMap::new();
    let mut curr_max_layer = 3;
    for i in 0..target_count {
        let mut next_layer_new_positions: HashSet<(usize, usize, isize, isize)> = HashSet::new();
        let mut keys: Vec<(isize, isize)> =
            all_positions_by_layer.keys().map(|v| (v.0, v.1)).collect();
        keys.sort_by(|a, b| (a.1 + a.0).cmp(&(b.1 + b.0)));
        for layer in keys {
            let mut all_positions: &mut Vec<HashSet<(usize, usize)>> =
                all_positions_by_layer.get_mut(&layer).unwrap();
            let mut positions = all_positions.last().unwrap().clone();
            if all_positions.len() >= 3 {
                let last_position = all_positions.iter().last().unwrap().clone();
                let prev_positions = all_positions
                    .iter()
                    .rev()
                    .skip(2)
                    .rev()
                    .last()
                    .unwrap()
                    .clone();
                if prev_positions == last_position {
                    let last_two: Vec<u128> = all_positions
                        .iter()
                        .rev()
                        .take(2)
                        .rev()
                        .map(|v| v.len() as u128)
                        .collect();
                    full_layers.insert(layer.clone(), (last_two, i));
                    if interesting_layers_key.contains(&layer) {
                        let mut tmp_vec: HashMap<usize, usize> = HashMap::new();
                        all_positions
                            .iter()
                            .rev()
                            .skip(1)
                            .rev()
                            .enumerate()
                            .for_each(|(idx, e)| {
                                tmp_vec.insert(idx, e.len());
                            });
                        interesting_vectors.insert(layer, (tmp_vec, i - 1));
                    }
                    all_positions_by_layer.remove(&layer);
                    continue;
                }
            }
            let mut new_positions: HashSet<(usize, usize)> = HashSet::new();
            for position in &positions {
                let (neighbors, outside_neighbour) = get_neighbors(input, position);

                new_positions.extend(neighbors);
                let resolve_outside_neighbour: Vec<(usize, usize, isize, isize)> =
                    outside_neighbour
                        .iter()
                        .map(|(x, y, dx, dy)| (*x, *y, layer.0 + dx, layer.1 + dy))
                        .collect();
                next_layer_new_positions.extend(resolve_outside_neighbour);
            }
            positions = new_positions.iter().map(|e| (e.0, e.1)).collect();
            all_positions.push(positions.clone());
        }
        if next_layer_new_positions.len() > 0 {
            for (x, y, layer_x, layer_y) in next_layer_new_positions {
                if full_layers.contains_key(&(layer_x, layer_y)) {
                    continue;
                };
                layer_start_exist.entry((layer_x, layer_y)).or_insert(i + 1);
                let positions = all_positions_by_layer
                    .entry((layer_x, layer_y))
                    .or_insert(vec![]);
                if let Some(last) = positions.last_mut() {
                    last.insert((x, y));
                } else {
                    let mut tmp_hash = HashSet::new();
                    tmp_hash.insert((x, y));
                    positions.push(tmp_hash);
                }
                positions.last_mut().unwrap();
            }
        }
        if interesting_vectors.len() == interesting_layers_key.len() {
            let diff1 = interesting_vectors.get(&(0isize, -2isize)).unwrap();
            let diff2 = interesting_vectors.get(&(0isize, -3isize)).unwrap();
            let repeat_diff = diff2.1 - diff1.1;
            let mut special_layer_count: HashMap<usize, u128> = HashMap::new();
            let target_to_end = target_count - i - 1;
            if target_to_end % repeat_diff != 0 {
                continue;
            }
            let diff_exist = diff_between_layers.len() > 0;
            let mut diff_between_layers2: HashMap<isize, u128> = HashMap::new();
            let mut real_diff_between_layers: HashMap<isize, i128> = HashMap::new();
            let another_full: HashMap<(isize, isize), (Vec<u128>, u128)> = HashMap::from_iter(
                full_layers
                    .iter()
                    .filter(|(&pos, _)| pos.0.abs() + pos.1.abs() > curr_max_layer)
                    .map(|(&pos, &ref v)| (pos, v.clone())),
            );
            let pos_len_nieparzyste = -2;
            if !diff_exist {
                *diff_between_layers.entry(pos_len_nieparzyste).or_insert(0) +=
                    another_full.len() as u128;
            } else {
                *diff_between_layers2.entry(pos_len_nieparzyste).or_insert(0) +=
                    another_full.len() as u128;
                *real_diff_between_layers
                    .entry(pos_len_nieparzyste)
                    .or_insert(*diff_between_layers.get(&pos_len_nieparzyste).unwrap() as i128) -=
                    another_full.len() as i128;
            }
            let mut sum_all_positions_by_layer = 0;
            for (position, apbl) in &all_positions_by_layer {
                let pos_len = apbl.last().unwrap().len() as isize;
                sum_all_positions_by_layer += pos_len;
                if !diff_exist {
                    *diff_between_layers.entry(pos_len).or_insert(0) += 1;
                } else {
                    *diff_between_layers2.entry(pos_len).or_insert(0) += 1;
                    *real_diff_between_layers
                        .entry(pos_len)
                        .or_insert(*diff_between_layers.get(&pos_len).unwrap() as i128) -= 1;
                }
                if position.0 == 0 || position.1 == 0 {
                    *special_layer_count
                        .entry(apbl.last().unwrap().len())
                        .or_insert(0) += 1;
                }
            }
            curr_max_layer += 1;
            if !diff_exist {
                continue;
            }
            diff_between_layers = diff_between_layers2.clone();
            let full_count2 = full_layers.get(&(0, 0)).unwrap().0.get(0).unwrap().clone();
            let full_count1 = full_layers.get(&(0, 0)).unwrap().0.get(1).unwrap().clone();
            let layer_to_finish = target_to_end / repeat_diff;
            let mut all_full_layer_count = 1;
            let mut parzyste = 1;
            let mut nieparzyste = 0;
            for i in 0..(layer_to_finish + curr_max_layer as u128) {
                all_full_layer_count += i * 4;
                if i % 2 == 1 {
                    nieparzyste += i * 4;
                } else {
                    parzyste += i * 4;
                }
            }
            let mut full_cells = parzyste * full_count2 + nieparzyste * full_count1;
            let mut full_cells2 = parzyste * full_count2 + nieparzyste * full_count1;
            // result += sum_all_positions_by_layer as u128;
            let mut another_sum = 0;
            for (pos_len, diff) in real_diff_between_layers {
                let curr_value = diff_between_layers.get(&pos_len).unwrap();
                if diff != 0 {
                    if pos_len == pos_len_nieparzyste {
                        if target_to_end % 2 == 0 {
                            full_cells +=
                                (curr_value + (diff.abs() as u128 * layer_to_finish)) * full_count1;
                        } else {
                            full_cells +=
                                (curr_value + (diff.abs() as u128 * layer_to_finish)) * full_count2;
                        }
                        all_full_layer_count +=
                            (curr_value + (diff.abs() as u128 * layer_to_finish));
                        parzyste += curr_value + (diff.abs() as u128 * layer_to_finish);
                        println!(
                            "inne full scount {}",
                            curr_value + (diff.abs() as u128 * layer_to_finish)
                        )
                    } else {
                        another_sum +=
                            (curr_value + (diff.abs() as u128 * layer_to_finish)) * pos_len as u128;
                    }
                } else {
                    another_sum += curr_value * pos_len as u128;
                }
            }
            let result = full_cells + another_sum;
            println!(
                "{},{}, {}, {}, {}",
                full_cells,
                full_cells2,
                another_sum,
                full_cells + another_sum,
                full_cells2 + another_sum
            );
            println!("NEW RESULTTT {:?}", result);
            return Some(result);
        }
    }

    let mut result = 0;

    for (key, layer3) in all_positions_by_layer {
        if let Some(start_i) = layer_start_exist.get(&key) {
            let correct_position = target_count as i128 - *start_i as i128;
            if correct_position < 0 {
                continue;
            }
            let position = if let Some(p) = layer3.get(correct_position as usize) {
                p
            } else {
                let tmp = (correct_position - layer3.len() as i128 + 1) as usize;
                if let Some(res) = layer3.get(layer3.len() - 1 - tmp % 2) {
                    res
                } else {
                    panic!("ERROR2");
                }
            };
            let test123 = position.clone();
            result += position.len() as u128;
        }
    }
    let mut count_idx: HashMap<usize, u128> = HashMap::new();
    for (pos, (last_two_len, last_idx)) in &full_layers {
        let fix_idx: usize = (target_count as usize - *last_idx as usize + 1) % 2;
        result += last_two_len.get(fix_idx).unwrap();
        *count_idx.entry(fix_idx).or_insert(0) += 1;
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_param(&advent_of_code::template::read_file("examples", DAY), 6);
        assert_eq!(result, Some(16));
    }

    #[test]
    fn test_part_one_b() {
        let result = part_one_param(&advent_of_code::template::read_file("examples", DAY), 64);
        assert_eq!(result, Some(42));
    }

    #[test]
    fn test_part_two_a() {
        let result = part_two_param(&advent_of_code::template::read_file("examples", DAY), 6);
        assert_eq!(result, Some(16));
    }

    #[test]
    fn test_part_two_b() {
        let result = part_two_param(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two_c() {
        let result = part_two_param(&advent_of_code::template::read_file("examples", DAY), 50);
        assert_eq!(result, Some(1594));
    }

    #[test]
    fn test_part_two_d() {
        let result = part_two_param(&advent_of_code::template::read_file("examples", DAY), 100);
        assert_eq!(result, Some(6536));
    }

    #[test]
    fn test_part_two_e() {
        let result = part_two_param(&advent_of_code::template::read_file("examples", DAY), 500);
        assert_eq!(result, Some(167004));
    }

    #[test]
    fn test_part_two_f() {
        let result = part_two_param(&advent_of_code::template::read_file("examples", DAY), 1000);
        assert_eq!(result, Some(668697));
    }

    #[test]
    fn test_part_two_g() {
        let result = part_two_param(&advent_of_code::template::read_file("examples", DAY), 5000);
        assert_eq!(result, Some(16733044));
    }
}
