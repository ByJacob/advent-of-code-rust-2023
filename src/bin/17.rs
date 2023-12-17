// Inspired from https://github.com/Lkeurentjes/Advent_of_code/blob/main/2023/2023-17-Clumsy-Crucible/2023-17-Clumsy-Crucible.py

use rand::Rng;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
advent_of_code::solution!(17);

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
enum Direction {
    LEFT,
    RIGHT,
    DOWN,
    UP,
    UNKNOWN,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Node {
    row: isize,
    col: isize,
    distance: u32,
    direction: Direction,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct NodeSimple {
    row: isize,
    col: isize,
    direction: Direction,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(distance_matrix: &Vec<Vec<u32>>, flow_min: isize, flow_max: isize) -> Option<u32> {
    let row_len = distance_matrix.len() as isize;
    let col_len = distance_matrix[0].len() as isize;

    let mut cheapmap: Vec<Vec<u32>> = vec![vec![u32::MAX; col_len as usize]; row_len as usize];
    let mut seen: HashSet<NodeSimple> = HashSet::new();
    let mut path_map: HashMap<(isize, isize), (isize, isize)> = HashMap::new();

    let mut priority_queue = BinaryHeap::new();
    priority_queue.push(Node {
        row: 0,
        col: 0,
        distance: 0,
        direction: Direction::UNKNOWN,
    });

    while let Some(Node {
        row,
        col,
        distance,
        direction,
    }) = priority_queue.pop()
    {
        if seen.contains(&NodeSimple {
            row,
            col,
            direction,
        }) {
            continue;
        }
        seen.insert(NodeSimple {
            row,
            col,
            direction,
        });
        for (dx, dy, dd) in [
            (0, 1, Direction::RIGHT),
            (0, -1, Direction::LEFT),
            (-1, 0, Direction::UP),
            (1, 0, Direction::DOWN),
        ] {
            match direction {
                Direction::LEFT => {
                    if dd == Direction::RIGHT || dd == Direction::LEFT {
                        continue;
                    }
                }
                Direction::RIGHT => {
                    if dd == Direction::LEFT || dd == Direction::RIGHT {
                        continue;
                    }
                }
                Direction::DOWN => {
                    if dd == Direction::UP || dd == Direction::DOWN {
                        continue;
                    }
                }
                Direction::UP => {
                    if dd == Direction::DOWN || dd == Direction::UP {
                        continue;
                    }
                }
                _ => {}
            }
            let mut new_distance = distance;
            let flow_min_row = row + (dx * (flow_min - 1));
            let flow_min_col = col + (dy * (flow_min - 1));
            if 0 <= flow_min_row
                && flow_min_row < row_len
                && 0 <= flow_min_col
                && flow_min_col < col_len
            {
                for i in 1..flow_min {
                    let new_row = row + (dx * i);
                    let new_col = col + (dy * i);
                    new_distance += distance_matrix[new_row as usize][new_col as usize];
                }
            }
            for next in flow_min..=flow_max {
                let new_row = row + (dx * next);
                let new_col = col + (dy * next);
                if 0 <= new_row && new_row < row_len && 0 <= new_col && new_col < col_len {
                    let new_row = new_row as usize;
                    let new_col = new_col as usize;
                    new_distance += distance_matrix[new_row][new_col];
                    if new_distance < cheapmap[new_row][new_col] {
                        cheapmap[new_row][new_col] = new_distance;
                        path_map.insert((new_row as isize, new_col as isize), (row, col));
                    }
                    priority_queue.push(Node {
                        row: new_row as isize,
                        col: new_col as isize,
                        distance: new_distance,
                        direction: dd,
                    })
                }
            }
        }
    }
    let mut paths = vec![];
    let mut next_node = (row_len - 1, col_len - 1);
    paths.push(next_node);
    while let Some(node) = path_map.get(&next_node) {
        paths.push(node.clone());
        next_node = node.clone();
        if node.0 == 0 && node.1 == 0 {
            break;
        }
    }
    let distance = cheapmap[row_len as usize - 1][col_len as usize - 1];
    // println!("{:?}", cheapmap);
    Some(distance)
}

fn parse_graph(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|s| s.to_digit(10).unwrap()).collect())
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let distance_matrix = parse_graph(input);
    dijkstra(&distance_matrix, 1, 3)
}

pub fn part_two(input: &str) -> Option<u32> {
    let distance_matrix = parse_graph(input);
    dijkstra(&distance_matrix, 4, 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two_b() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(71));
    }
}
