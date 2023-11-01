pub fn part_one(input: &str) -> Option<u32> {
    let values = input
        // Split each elf by double linebreak
        .split("\n\n")
        // Split each set of calories by a single linebreak
        .map(
            |x| {
                x.split('\n')
                    // Parse out each calorie value
                    .map(|x| x.parse::<u32>().unwrap_or(0))
                    // Collect each value into a vec of u32s
                    .collect::<Vec<u32>>()
            }, // Store each elf's calories in a vec of u32s
        )
        .collect::<Vec<Vec<u32>>>();

    let most_calories = values.iter().map(|x| x.iter().sum()).max();

    most_calories
}

pub fn part_two(input: &str) -> Option<u32> {
    let values = input
    // Split each elf by double linebreak
    .split("\n\n")
    // Split each set of calories by a single linebreak
    .map(
        |x| {
            x.split('\n')
                // Parse out each calorie value
                .map(|x| x.parse::<u32>().unwrap_or(0))
                // Collect each value into a vec of u32s
                .collect::<Vec<u32>>()
        }, // Store each elf's calories in a vec of u32s
    )
    .collect::<Vec<Vec<u32>>>();

    // Sum and store the calories stored by each elf
    let mut elves = values.iter().map(|x| x.iter().sum()).collect::<Vec<u32>>();
    // Sort the clories each elf has
    elves.sort();
    // Grab the first 3 elves (index 0 through 2) and sum their calories
    Some(elves[0..2].iter().sum())
}

advent_of_code::main!(1);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 1));
        assert_eq!(result, Some(69693));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 1));
        assert_eq!(result, Some(11411));
    }
}
