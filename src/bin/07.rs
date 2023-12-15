use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::str::FromStr;
advent_of_code::solution!(7);

trait SplitToVector<T: FromStr> {
    fn to_vector(self) -> Vec<T>
    where
        <T as FromStr>::Err: Debug;
}

fn get_card_order() -> HashMap<String, i32> {
    let mut result: HashMap<String, i32> = HashMap::new();
    result.insert(String::from("2"), 1);
    result.insert(String::from("3"), 2);
    result.insert(String::from("4"), 3);
    result.insert(String::from("5"), 4);
    result.insert(String::from("6"), 5);
    result.insert(String::from("7"), 6);
    result.insert(String::from("8"), 7);
    result.insert(String::from("9"), 8);
    result.insert(String::from("T"), 9);
    result.insert(String::from("J"), 10);
    result.insert(String::from("Q"), 11);
    result.insert(String::from("K"), 12);
    result.insert(String::from("A"), 13);
    result
}

fn get_card_order2() -> HashMap<String, i32> {
    let mut result: HashMap<String, i32> = HashMap::new();
    result.insert(String::from("2"), 1);
    result.insert(String::from("3"), 2);
    result.insert(String::from("4"), 3);
    result.insert(String::from("5"), 4);
    result.insert(String::from("6"), 5);
    result.insert(String::from("7"), 6);
    result.insert(String::from("8"), 7);
    result.insert(String::from("9"), 8);
    result.insert(String::from("T"), 9);
    result.insert(String::from("J"), 0);
    result.insert(String::from("Q"), 11);
    result.insert(String::from("K"), 12);
    result.insert(String::from("A"), 13);
    result
}

enum HandType {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    pub fn value(&self) -> i8 {
        match self {
            HandType::FiveOfKind => 7,
            HandType::FourOfKind => 6,
            HandType::FullHouse => 5,
            HandType::ThreeOfKind => 4,
            HandType::TwoPair => 3,
            HandType::OnePair => 2,
            HandType::HighCard => 1,
        }
    }
    pub fn name(&self) -> String {
        match self {
            HandType::FiveOfKind => String::from("FiveOfKind"),
            HandType::FourOfKind => String::from("FourOfKind"),
            HandType::FullHouse => String::from("FullHouse"),
            HandType::ThreeOfKind => String::from("ThreeOfKind"),
            HandType::TwoPair => String::from("TwoPair"),
            HandType::OnePair => String::from("OnePair"),
            HandType::HighCard => String::from("HighCard"),
        }
    }
}

struct Hand {
    cards: String,
    bid: i32,
    hand_type: HandType,
    rank: i32,
}

impl Debug for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // implement the formatting here
        write!(
            f,
            "Hand cards={}; bid={}; hand_type={}; rank={}\n",
            self.cards,
            self.bid,
            self.hand_type.name(),
            self.rank
        )
    }
}

impl Hand {
    fn determine_hand_type(&mut self) {
        let mut uniq_count: HashMap<char, i32> = HashMap::new();
        self.cards
            .chars()
            .into_iter()
            .map(|label| (label, self.cards.chars().filter(|&c| c == label).count()))
            .for_each(|(ch, cou)| {
                uniq_count.insert(ch, cou as i32);
            });

        match uniq_count.values().max() {
            Some(5) => self.hand_type = HandType::FiveOfKind,
            Some(4) => self.hand_type = HandType::FourOfKind,
            Some(3) => {
                if uniq_count.values().any(|f| f == &2) {
                    self.hand_type = HandType::FullHouse
                } else {
                    self.hand_type = HandType::ThreeOfKind
                }
            }
            Some(2) => {
                if uniq_count.values().filter(|&p| p == &2).count() >= 2 {
                    self.hand_type = HandType::TwoPair
                } else {
                    self.hand_type = HandType::OnePair
                }
            }
            _ => self.hand_type = HandType::HighCard,
        }
    }

    fn generate_combinations(max_value: i32, target_sum: i32, depth: usize) -> Vec<Vec<i32>> {
        let mut combinations = Vec::new();

        fn generate_recursive(
            current: Vec<i32>,
            remaining_depth: usize,
            target_sum: i32,
            max_value: i32,
            combinations: &mut Vec<Vec<i32>>,
        ) {
            if remaining_depth == 0 {
                if current.iter().sum::<i32>() <= target_sum {
                    combinations.push(current);
                }
                return;
            }

            for i in 0..=max_value {
                let mut new_combination = current.clone();
                new_combination.push(i);
                generate_recursive(
                    new_combination,
                    remaining_depth - 1,
                    target_sum,
                    max_value,
                    combinations,
                );
            }
        }

        generate_recursive(Vec::new(), depth, target_sum, max_value, &mut combinations);
        combinations
    }

    fn determine_hand_type_with_joker(&mut self, is_joker: bool) {
        let unique_labels: HashSet<char> = self.cards.chars().collect();
        let count_j = self.cards.chars().filter(|&c| c == 'J').count();
        let mut uniq_count: HashMap<char, i32> = HashMap::new();
        self.cards
            .chars()
            .into_iter()
            .map(|label| (label, self.cards.chars().filter(|&c| c == label).count()))
            .for_each(|(ch, cou)| {
                uniq_count.insert(ch, cou as i32);
            });
        let mut uniq_counts: Vec<HashMap<char, i32>> = Vec::new();
        uniq_counts.push(uniq_count.clone());

        if count_j > 0 {
            uniq_count.remove(&'J');
            let max_value = count_j as i32; // Change this value to set the maximum value for each element
            let target_sum = count_j as i32; // Change this value to set the target sum
            let depth = uniq_count.len(); // Change this value to set the depth of the combinations
            let combinations = Hand::generate_combinations(max_value, target_sum, depth);
            let keys: Vec<char> = uniq_count.keys().map(|k| *k).collect();
            combinations.iter().for_each(|c| {
                let mut tmp_uniq_count = uniq_count.clone();
                for idx in 0..keys.len() {
                    *(tmp_uniq_count.get_mut(keys.get(idx).unwrap()).unwrap()) +=
                        c.get(idx).unwrap();
                }
                tmp_uniq_count.insert('J', (count_j as i32) - c.iter().sum::<i32>());
                uniq_counts.push(tmp_uniq_count.clone());
            });
        }
        let mut max_value = HandType::HighCard;
        for uniq_count in uniq_counts {
            let curr_value = match uniq_count.values().max() {
                Some(5) => HandType::FiveOfKind,
                Some(4) => HandType::FourOfKind,
                Some(3) => {
                    if uniq_count.values().any(|f| f == &2) {
                        HandType::FullHouse
                    } else {
                        HandType::ThreeOfKind
                    }
                }
                Some(2) => {
                    if uniq_count.values().filter(|&p| p == &2).count() >= 2 {
                        HandType::TwoPair
                    } else {
                        HandType::OnePair
                    }
                }
                _ => HandType::HighCard,
            };
            if curr_value.value() > max_value.value() {
                max_value = curr_value
            }
        }

        self.hand_type = max_value;
    }

    pub fn calc_rank(&self, card_order: &HashMap<String, i32>) -> i32 {
        let base: i32 = 20;
        let res = (self.hand_type.value() as i32) * base.pow(5)
            + self
                .cards
                .chars()
                .rev()
                .enumerate()
                .map(|(idx, c)| {
                    (card_order.get(&String::from(c)).unwrap()) * (base.pow(idx as u32))
                })
                .sum::<i32>();
        let test123 = res.clone();
        res
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let mut hands: Vec<Hand> = Vec::new();

    for line in input.lines() {
        let (card, bid) = line
            .split_once(" ")
            .map(|(c, b)| (String::from(c), b.parse::<i32>().unwrap()))
            .unwrap();
        let card = String::from(card);
        let mut hand = Hand {
            cards: card,
            bid: bid,
            hand_type: HandType::HighCard,
            rank: 0,
        };
        hand.determine_hand_type();
        hands.push(hand);
    }

    let card_order = get_card_order();
    hands.sort_by_key(|h| h.calc_rank(&card_order));

    for idx in 0..hands.len() {
        hands.get_mut(idx).unwrap().rank = (idx + 1) as i32
    }
    let mut sum = 0;
    hands.iter().for_each(|h| sum += h.bid * (h.rank as i32));
    println!("{:?}", hands);
    println!("{:?}", sum);
    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hands: Vec<Hand> = Vec::new();

    for line in input.lines() {
        let (card, bid) = line
            .split_once(" ")
            .map(|(c, b)| (String::from(c), b.parse::<i32>().unwrap()))
            .unwrap();
        let card = String::from(card);
        let mut hand = Hand {
            cards: card,
            bid: bid,
            hand_type: HandType::HighCard,
            rank: 0,
        };
        hand.determine_hand_type_with_joker(true);
        hands.push(hand);
    }

    let card_order = get_card_order2();
    hands.sort_by_key(|h| h.calc_rank(&card_order));

    for idx in 0..hands.len() {
        hands.get_mut(idx).unwrap().rank = (idx + 1) as i32
    }
    let mut sum = 0;
    hands.iter().for_each(|h| sum += h.bid * (h.rank as i32));
    // println!("{:?}", hands);
    println!("{:?}", sum);
    Some(sum as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
