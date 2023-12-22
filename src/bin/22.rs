use std::collections::{HashMap, HashSet};
advent_of_code::solution!(22);

#[derive(Debug)]
struct Brick {
    name: usize,
    x1: usize,
    y1: usize,
    z1: usize,
    x2: usize,
    y2: usize,
    z2: usize,
}
pub fn part_one(input: &str) -> Option<u32> {
    let mut bricks: Vec<Brick> = vec![];

    let mut max_x = 0usize;
    let mut max_y = 0usize;
    let mut max_z = 0usize;
    for (idx, line) in input.lines().enumerate() {
        let (pos1, pos2) = line.split_once("~").unwrap();
        let part_pos1: Vec<usize> = pos1
            .split(",")
            .map(|p| p.parse::<usize>().unwrap())
            .collect();
        let part_pos2: Vec<usize> = pos2
            .split(",")
            .map(|p| p.parse::<usize>().unwrap())
            .collect();
        bricks.push(Brick {
            name: idx + 1,
            x1: part_pos1[0],
            y1: part_pos1[1],
            z1: part_pos1[2],
            x2: part_pos2[0],
            y2: part_pos2[1],
            z2: part_pos2[2],
        });
        max_x = max_x.max(part_pos1[0]).max(part_pos2[0]);
        max_y = max_y.max(part_pos1[1]).max(part_pos2[1]);
        max_z = max_z.max(part_pos1[2]).max(part_pos2[2]);
    }
    bricks.sort_by(|a, b| b.z1.max(b.z2).cmp(&(a.z1.max(a.z2))));
    let mut space = vec![vec![vec![0; max_y + 1]; max_x + 1]; max_z + 1];
    let mut placed_brick: Vec<Brick> = vec![];
    for brick in bricks.iter().rev() {
        let z_range = brick.z1..=brick.z2;
        let x_range = brick.x1..=brick.x2;
        let y_range = brick.y1..=brick.y2;
        let min_z = z_range.clone().min().unwrap();
        let mut z_diff = 0usize;
        loop {
            let mut elements = vec![];
            if min_z - z_diff > 1 {
                for z in z_range.clone() {
                    for x in x_range.clone() {
                        for y in y_range.clone() {
                            elements.push(space[z - z_diff - 1][x][y])
                        }
                    }
                }
            } else {
                elements.push(0);
            }
            if elements.iter().all(|e| e == &0) && min_z - z_diff > 1 {
                z_diff += 1;
            } else {
                placed_brick.push(Brick {
                    name: brick.name,
                    x1: brick.x1,
                    y1: brick.y1,
                    z1: brick.z1 - z_diff,
                    x2: brick.x2,
                    y2: brick.y2,
                    z2: brick.z2 - z_diff,
                });
                for z in z_range.clone() {
                    for x in x_range.clone() {
                        for y in y_range.clone() {
                            space[z - z_diff][x][y] = brick.name;
                        }
                    }
                }
                break;
            }
        }
    }
    let mut can_remove: Vec<usize> = vec![];
    for (z_idx, z) in space.iter().rev().skip(1).rev().enumerate() {
        let bricks_name: HashSet<usize> =
            HashSet::from_iter(z.into_iter().flatten().map(|v| *v).filter(|v| v > &0));
        let mut next_bricks: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut cannot_remove: Vec<usize> = vec![];
        for brick_name in &bricks_name {
            let curr_brick = placed_brick
                .iter()
                .find(|&b| b.name == *brick_name)
                .unwrap();
            if z_idx + 1 <= curr_brick.z1.max(curr_brick.z2) {
                cannot_remove.push(curr_brick.name);
                continue;
            }
            for x in curr_brick.x1..=curr_brick.x2 {
                for y in curr_brick.y1..=curr_brick.y2 {
                    if space[z_idx + 1][x][y] > 0 {
                        let next_brick_name = space[z_idx + 1][x][y];
                        next_bricks.entry(next_brick_name).or_insert(vec![]);
                        if !next_bricks[&next_brick_name].contains(&curr_brick.name) {
                            next_bricks
                                .get_mut(&next_brick_name)
                                .unwrap()
                                .push(curr_brick.name);
                        }
                    }
                }
            }
        }
        for brick_name in &bricks_name {
            let filtered_bricks: Vec<usize> = next_bricks
                .iter()
                .filter(|(&k, &ref v)| v.contains(brick_name))
                .map(|(_, &ref v)| v.len())
                .collect();
            if filtered_bricks.iter().all(|v| v > &1) && !cannot_remove.contains(brick_name) {
                can_remove.push(*brick_name);
            }
        }
    }

    // println!("{:?}", bricks);
    Some(can_remove.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut bricks: Vec<Brick> = vec![];

    let mut max_x = 0usize;
    let mut max_y = 0usize;
    let mut max_z = 0usize;
    for (idx, line) in input.lines().enumerate() {
        let (pos1, pos2) = line.split_once("~").unwrap();
        let part_pos1: Vec<usize> = pos1
            .split(",")
            .map(|p| p.parse::<usize>().unwrap())
            .collect();
        let part_pos2: Vec<usize> = pos2
            .split(",")
            .map(|p| p.parse::<usize>().unwrap())
            .collect();
        bricks.push(Brick {
            name: idx + 1,
            x1: part_pos1[0],
            y1: part_pos1[1],
            z1: part_pos1[2],
            x2: part_pos2[0],
            y2: part_pos2[1],
            z2: part_pos2[2],
        });
        max_x = max_x.max(part_pos1[0]).max(part_pos2[0]);
        max_y = max_y.max(part_pos1[1]).max(part_pos2[1]);
        max_z = max_z.max(part_pos1[2]).max(part_pos2[2]);
    }
    bricks.sort_by(|a, b| b.z1.max(b.z2).cmp(&(a.z1.max(a.z2))));
    let mut space = vec![vec![vec![0; max_y + 1]; max_x + 1]; max_z + 1];
    let mut placed_brick: Vec<Brick> = vec![];
    for brick in bricks.iter().rev() {
        let z_range = brick.z1..=brick.z2;
        let x_range = brick.x1..=brick.x2;
        let y_range = brick.y1..=brick.y2;
        let min_z = z_range.clone().min().unwrap();
        let mut z_diff = 0usize;
        loop {
            let mut elements = vec![];
            if min_z - z_diff > 1 {
                for z in z_range.clone() {
                    for x in x_range.clone() {
                        for y in y_range.clone() {
                            elements.push(space[z - z_diff - 1][x][y])
                        }
                    }
                }
            } else {
                elements.push(0);
            }
            if elements.iter().all(|e| e == &0) && min_z - z_diff > 1 {
                z_diff += 1;
            } else {
                placed_brick.push(Brick {
                    name: brick.name,
                    x1: brick.x1,
                    y1: brick.y1,
                    z1: brick.z1 - z_diff,
                    x2: brick.x2,
                    y2: brick.y2,
                    z2: brick.z2 - z_diff,
                });
                for z in z_range.clone() {
                    for x in x_range.clone() {
                        for y in y_range.clone() {
                            space[z - z_diff][x][y] = brick.name;
                        }
                    }
                }
                break;
            }
        }
    }
    let mut next_bricks: HashMap<usize, Vec<usize>> = HashMap::new();
    for (z_idx, z) in space.iter().rev().skip(1).rev().enumerate() {
        let bricks_name: HashSet<usize> =
            HashSet::from_iter(z.into_iter().flatten().map(|v| *v).filter(|v| v > &0));
        let mut cannot_remove: Vec<usize> = vec![];
        for brick_name in &bricks_name {
            let curr_brick = placed_brick
                .iter()
                .find(|&b| b.name == *brick_name)
                .unwrap();
            if z_idx + 1 <= curr_brick.z1.max(curr_brick.z2) {
                cannot_remove.push(curr_brick.name);
                continue;
            }
            for x in curr_brick.x1..=curr_brick.x2 {
                for y in curr_brick.y1..=curr_brick.y2 {
                    if space[z_idx + 1][x][y] > 0 {
                        let next_brick_name = space[z_idx + 1][x][y];
                        next_bricks.entry(next_brick_name).or_insert(vec![]);
                        if !next_bricks[&next_brick_name].contains(&curr_brick.name) {
                            next_bricks
                                .get_mut(&next_brick_name)
                                .unwrap()
                                .push(curr_brick.name);
                        }
                    }
                }
            }
        }
    }
    let mut sum_destroy = 0;
    for brick in &placed_brick {
        let filtered_bricks_values: Vec<(usize, Vec<usize>)> = next_bricks
            .iter()
            .filter(|(&k, &ref v)| v.contains(&brick.name))
            .map(|(&k, &ref v)| (k, v.clone()))
            .collect();
        let filtered_bricks: Vec<usize> =
            filtered_bricks_values.iter().map(|v| v.1.len()).collect();
        if !filtered_bricks.iter().all(|v| v > &1) {
            let mut copy_placed_brick: HashSet<usize> =
                placed_brick.iter().map(|b| b.name).collect();
            loop {
                let mut copy2_placed_brick = copy_placed_brick.clone();
                if copy_placed_brick.contains(&brick.name) {
                    copy2_placed_brick.retain(|e| e != &brick.name);
                } else {
                    for (curr_brick, prev_bricks) in &next_bricks {
                        if prev_bricks.iter().all(|e| !copy_placed_brick.contains(e))
                            && copy_placed_brick.contains(curr_brick)
                        {
                            copy2_placed_brick.retain(|e| e != curr_brick);
                            sum_destroy += 1;
                        }
                    }
                }
                if copy_placed_brick.len() != copy2_placed_brick.len() {
                    copy_placed_brick = copy2_placed_brick;
                } else {
                    break;
                }
            }
        }
    }

    println!("{:?}", next_bricks);
    Some(sum_destroy)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }
}
