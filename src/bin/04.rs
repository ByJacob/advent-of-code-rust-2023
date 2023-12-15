use std::collections::HashMap;
advent_of_code::solution!(4);

trait SplitToVector {
    fn to_vector(self) -> Vec<i32>;
}

impl SplitToVector for String {
    fn to_vector(self) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        self.split_whitespace().for_each(|number| result.push(number.parse().unwrap()));
        result
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    input.lines().for_each(|line|{
        let (card_id, numbers) = line.split_once(":").unwrap();
        let (_, card_id) = card_id.split_once(" ").unwrap();
        let card_id: i32 = card_id.trim().parse().unwrap();
        let (winning_numbers, my_numbers) = numbers
            .split_once("|")
            .map(|(winning, my)| (String::from(winning.trim()), String::from(my.trim())))
            .map(|(winning, my)| (winning.to_vector(), my.to_vector()))
            .unwrap();
        let common_elements: Vec<i32> = winning_numbers
            .iter()
            .filter(|&x| my_numbers.contains(x))
            .cloned()
            .collect();
        let mut cur_points = 0;
        (0..common_elements.len()).for_each(|i| {
            cur_points = if cur_points == 0 {1} else {cur_points*2};
        });
        sum += cur_points;
    });

    print!("Sum points: {}", sum);
    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cards: HashMap<i32, i32> = HashMap::new();
    input.lines().for_each(|line|{
        let (card_id, numbers) = line.split_once(":").unwrap();
        let (_, card_id) = card_id.split_once(" ").unwrap();
        let card_id: i32 = card_id.trim().parse().unwrap();
        let (winning_numbers, my_numbers) = numbers
            .split_once("|")
            .map(|(winning, my)| (String::from(winning.trim()), String::from(my.trim())))
            .map(|(winning, my)| (winning.to_vector(), my.to_vector()))
            .unwrap();
        let common_elements: Vec<i32> = winning_numbers
            .iter()
            .filter(|&x| my_numbers.contains(x))
            .cloned()
            .collect();
        cards.entry(card_id).and_modify(|v| *v += 1).or_insert(1);
        let multiplex = *cards.get(&card_id).unwrap();
        // println!("{:?}", multiplex);
        (0..common_elements.len()).for_each(|i| {
            cards.entry(card_id+1+(i as i32))
                .and_modify(|v| *v += multiplex)
                .or_insert(multiplex);
        });
    });
    let sum: i32 = cards.values().sum();
    print!("Sum points: {}", sum);
    Some(sum as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
