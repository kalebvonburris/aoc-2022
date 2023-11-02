pub fn part_one(input: &str) -> Option<u32> {
    // Trim leading and trailing newlines and whitespace
    let input = input.trim();
    let mut total = 0;
    for line in input.split('\n') {
        // Grab the left_compartment (first half of the string)
        let left_compartment: Vec<char> = line[0..line.len() / 2].chars().collect::<Vec<char>>();
        // Initialize a value to represent the intersection of the two halves
        let mut char_intersection = 0;
        // Find the intersection of the two halves
        for c in line[line.len() / 2..line.len()].chars() {
            if left_compartment.contains(&c) {
                char_intersection = c as u32;
                break;
            }
        }
        // Add the value of the intersection to the total
        // Uppercase
        if char_intersection < 'a' as u32 {
            total += char_intersection - '@' as u32 + 26;
        }
        // Lowercase
        else {
            total += char_intersection - '`' as u32;
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    // Trim leading and trailing newlines and whitespace
    let input = input.trim();
    let mut total = 0;
    let lines = input.split('\n').collect::<Vec<&str>>();

    for i in (0..lines.len()).step_by(3) {
        // Gather the three lines
        let first = lines[i].chars().collect::<Vec<char>>();
        let second = lines[i + 1].chars().collect::<Vec<char>>();
        let third = lines[i + 2];
        // Initialize a value to represent the intersection of the first two groups
        let mut first_second_intersection: Vec<char> = Vec::new();
        // Find the chars contained in the first and second lines
        for c in second {
            if first.contains(&c) {
                first_second_intersection.push(c);
            }
        }
        // Find the intersection of the third line and the first and second lines
        let mut char_intersection = 0;
        for c in third.chars() {
            if first_second_intersection.contains(&c) {
                char_intersection = c as u32;
            }
        }
        // Add the value of the intersection to the total
        // Uppercase
        if char_intersection < 'a' as u32 {
            total += char_intersection - '@' as u32 + 26;
        }
        // Lowercase
        else {
            total += char_intersection - '`' as u32;
        }
    }
    Some(total)
}

advent_of_code::main!(3);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let example = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"#;
        let result = part_one(example);
        assert_eq!(result, Some(157));
    }

    #[test]
    fn test_part_two() {
        let example = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"#;
        let result = part_two(example);
        assert_eq!(result, Some(70));
    }
}
