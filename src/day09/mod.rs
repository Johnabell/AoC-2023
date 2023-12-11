use std::str::FromStr;

pub const PUZZLE_INPUT: &str = include_str!("input.txt");

pub fn part1(input: &str) -> isize {
    Sequences::parse(input).result_part1()
}

pub fn part2(input: &str) -> isize {
    Sequences::parse(input).result_part2()
}

struct Sequences {
    sequences: Vec<Sequence>,
}

impl Sequences {
    fn parse(input: &str) -> Self {
        let sequences = input.split('\n').map(Sequence::parse).collect();

        Self { sequences }
    }

    fn result_part1(&self) -> isize {
        self.sequences.iter().map(Sequence::next).sum()
    }

    fn result_part2(&self) -> isize {
        self.sequences.iter().map(Sequence::previous).sum()
    }
}

struct Sequence {
    seq: Vec<isize>,
}

impl Sequence {
    fn parse(input: &str) -> Self {
        let seq = input
            .split_whitespace()
            .map(FromStr::from_str)
            .map(Result::unwrap)
            .collect();

        Self { seq }
    }

    fn next(&self) -> isize {
        let mut result = *self.seq.last().unwrap();
        let mut diffs = Self::diffs(&self.seq);
        while diffs.iter().any(|i| *i != 0) {
            result += *diffs.last().unwrap();
            diffs = Self::diffs(&diffs);
        }
        result
    }

    fn previous(&self) -> isize {
        let mut result = *self.seq.first().unwrap();
        let mut diffs = Self::diffs(&self.seq);
        let mut sign = -1;
        while diffs.iter().any(|i| *i != 0) {
            result += sign * *diffs.first().unwrap();
            sign *= -1;
            diffs = Self::diffs(&diffs);
        }
        result
    }

    fn diffs(seq: &[isize]) -> Vec<isize> {
        seq.windows(2).map(|window| window[1] - window[0]).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;

    #[test]
    fn test_input_part1() {
        assert_eq!(part1(TEST_INPUT), 114);
    }
    #[test]
    fn test_puzzle_input_part1() {
        assert_eq!(part1(PUZZLE_INPUT), 1930746032);
    }
    #[test]
    fn test_input_part2() {
        assert_eq!(part2(TEST_INPUT), 2);
    }
    #[test]
    fn test_puzzle_input_part2() {
        assert_eq!(part2(PUZZLE_INPUT), 1154);
    }
}
