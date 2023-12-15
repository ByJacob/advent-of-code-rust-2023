advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;
    let mut sum = 0;
    for line in input.lines() {
        let mut is_possible = true;
        if let Some((game_id, games)) = line.split_once(":") {
            let id_str = game_id.split_whitespace().last().expect("Missing ID");
            let id: i32 = id_str.parse().expect("Missing int");
            for game in games.split(";") {
                game.split(",")
                    .map(|g| g.trim())
                    .map(|g| g.split_once(" ").expect("Missing 2 values"))
                    .map(|(n, c)| (n.parse::<i32>().expect("INT"), c))
                    .for_each(|(n, c)| match c {
                        "red" => is_possible = is_possible && n <= max_red,
                        "green" => is_possible = is_possible && n <= max_green,
                        "blue" => is_possible = is_possible && n <= max_blue,
                        _ => panic!("Unknown color"),
                    });
            }
            if is_possible {
                sum += id;
            }
        }
    }
    print!("Sum gameID: {}", sum);
    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        if let Some((game_id, games)) = line.split_once(":") {
            let id_str = game_id.split_whitespace().last().expect("Missing ID");
            let id: i32 = id_str.parse().expect("Missing int");
            for game in games.split(";") {
                game.split(",")
                    .map(|g| g.trim())
                    .map(|g| g.split_once(" ").expect("Missing 2 values"))
                    .map(|(n, c)| (n.parse::<i32>().expect("INT"), c))
                    .for_each(|(n, c)| match c {
                        "red" => max_red = max_red.max(n),
                        "green" => max_green = max_green.max(n),
                        "blue" => max_blue = max_blue.max(n),
                        _ => panic!("Unknown color"),
                    });
            }
        }
        sum += max_red * max_green * max_blue
    }
    print!("Power: {}", sum);
    Some(sum as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
