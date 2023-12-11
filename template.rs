pub const PUZZLE_INPUT: &str = include_str!("input.txt");

pub fn part1(input: &str) -> usize {
    todo!();
}

pub fn part2(input: &str) -> usize {
    todo!();
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#""#;

    #[test]
    fn test_input_part1() {
        assert_eq!(part1(TEST_INPUT), 0);
    }
    #[test]
    fn test_puzzle_input_part1() {
        assert_eq!(part1(PUZZLE_INPUT), 0);
    }
    #[test]
    fn test_input_part2() {
        assert_eq!(part2(TEST_INPUT), 0);
    }
    #[test]
    fn test_puzzle_input_part2() {
        assert_eq!(part2(PUZZLE_INPUT), 0);
    }
}
