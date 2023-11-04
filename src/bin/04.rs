pub fn part_one(input: &str) -> Option<u32> {
    let input = input.trim();
    let mut pairs = 0;
    for line in input.split('\n') {
        // Trim carriage returns
        let line = line.trim();
        // Split the line into two ranges
        let elves = line
            .split(',')
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        // Grab the values of the first range
        let first_range = elves[0]
            .split('-')
            .map(|x| x.parse::<u32>())
            .collect::<Result<Vec<u32>, _>>()
            .unwrap();
        // Grab the second range
        let second_range = elves[1]
            .split('-')
            .map(|x| x.parse::<u32>())
            .collect::<Result<Vec<u32>, _>>()
            .unwrap();
        // Perform range comparison
        if first_range[0] <= second_range[0] && first_range[1] >= second_range[1]
            || second_range[0] <= first_range[0] && second_range[1] >= first_range[1]
        {
            pairs += 1;
        }
    }

    Some(pairs)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = input.trim();
    let mut pairs = 0;
    for line in input.split('\n') {
        // Trim carriage returns
        let line = line.trim();
        // Split the line into two ranges
        let elves = line
            .split(',')
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        // Grab the values of the first range
        let first_range = elves[0]
            .split('-')
            .map(|x| x.parse::<u32>())
            .collect::<Result<Vec<u32>, _>>()
            .unwrap();
        // Grab the second range
        let second_range = elves[1]
            .split('-')
            .map(|x| x.parse::<u32>())
            .collect::<Result<Vec<u32>, _>>()
            .unwrap();
        // Perform range comparison
        if (first_range[0] <= second_range[1] && first_range[1] >= second_range[1]) ||
           (second_range[0] <= first_range[1] && second_range[1] >= first_range[1]) {
            pairs += 1;
        }
    }

    Some(pairs)
}

advent_of_code::main!(4);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 4));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 4));
        assert_eq!(result, Some(4));
    }
}
