pub fn part_one(input: &str) -> Option<u32> {
    // Trim the extra \n on the input
    let input = input.trim_end();
    // Iterate over each strategy and sum the value of each game
    let mut total: u32 = 0;
    for line in input.split('\n') {
        // Split each line into two parts
        let strategy = line.chars().collect::<Vec<char>>();

        // Grab the opponent and choice strategies
        let opponent = strategy[0] as u32 - '@' as u32;
        let choice = strategy[2] as u32 - 'W' as u32;

        // Append the chosen score to the total
        total += choice;

        // Calculate a 0: tie, 1: win, 2: lose case
        let delta = (choice + 3 - opponent) % 3;

        // Win case
        if delta == 1 {
            total += 6;
        }
        // Tie case
        else if delta == 0 {
            total += 3;
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    // Trim the extra \n on the input
    let input = input.trim_end();
    // Iterate over each strategy and sum the value of each game
    let mut total: u32 = 0;
    for line in input.split('\n') {
        // Split each line into two parts
        let strategy = line.chars().collect::<Vec<char>>();

        // Grab the opponent and choice strategies
        let opponent = strategy[0] as u32 - '@' as u32;
        let choice = strategy[2];

        // Lose case
        if choice == 'X' {
            total += match opponent {
                1 => 3,
                2 => 1,
                _ => 2,
            };
        }
        // Tie case
        else if choice == 'Y' {
            total += 3 + opponent;
        }
        // Win case
        else {
            total += 6 + match opponent {
                1 => 2,
                2 => 3,
                _ => 1,
            };
        }
    }

    Some(total)
}
advent_of_code::main!(2);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let example = r#"A Y
B X
C Z"#;
        let expected_result = Some(15);
        let result = part_one(example);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_part_two() {
        let example = r#"A Y
B X
C Z"#;
        let expected_result = Some(12);
        let result: Option<u32> = part_two(example);
        assert_eq!(result, expected_result);
    }
}
