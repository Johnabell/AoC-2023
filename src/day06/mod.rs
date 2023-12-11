use std::str::FromStr;

pub const PUZZLE_INPUT: &str = include_str!("input.txt");

pub fn part1(input: &str) -> u64 {
    Races::parse(input).calculate()
}

pub fn part2(input: &str) -> u64 {
    Race::parse_part2(input).ways_to_win()
}

#[derive(Debug)]
struct Races {
    races: Vec<Race>,
}

impl Races {
    fn parse(input: &str) -> Self {
        let (times, distances) = input.split_once('\n').unwrap();
        let distance = distances
            .strip_prefix("Distance: ")
            .unwrap()
            .split_whitespace()
            .map(u64::from_str)
            .map(Result::unwrap);
        let races = times
            .strip_prefix("Time: ")
            .unwrap()
            .split_whitespace()
            .map(u64::from_str)
            .map(Result::unwrap)
            .zip(distance)
            .map(Race::new)
            .collect();

        Self { races }
    }

    fn calculate(&self) -> u64 {
        self.races
            .iter()
            .map(Race::ways_to_win)
            .reduce(std::ops::Mul::mul)
            .unwrap()
    }
}

#[derive(Debug)]
struct Race {
    time: u64,
    record: u64,
}

impl Race {
    fn parse_part2(input: &str) -> Self {
        let (times, distances) = input.split_once('\n').unwrap();
        let record: u64 = distances
            .strip_prefix("Distance: ")
            .unwrap()
            .split_whitespace()
            .collect::<String>()
            .parse()
            .unwrap();
        let time: u64 = times
            .strip_prefix("Time: ")
            .unwrap()
            .split_whitespace()
            .collect::<String>()
            .parse()
            .unwrap();

        Self { time, record }
    }

    fn new((time, record): (u64, u64)) -> Self {
        Self { time, record }
    }

    fn ways_to_win(&self) -> u64 {
        let time = self.time as f64;
        let distance = self.record as f64;
        let start = (time - f64::sqrt(time * time - 4.0 * distance)) / 2.0;
        let end = (time + f64::sqrt(time * time - 4.0 * distance)) / 2.0;

        end.strict_floor() as u64 - start.strict_ceil() as u64 + 1
    }
}

trait StrictCeilFloor: Sized {
    fn strict_floor(self) -> Self;
    fn strict_ceil(self) -> Self;
}

impl StrictCeilFloor for f64 {
    fn strict_ceil(self) -> Self {
        let ceil = self.ceil();
        if self == ceil {
            ceil + 1.0
        } else {
            ceil
        }
    }
    fn strict_floor(self) -> Self {
        let floor = self.floor();
        if self == floor {
            floor - 1.0
        } else {
            floor
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;

    #[test]
    fn test_input_part1() {
        assert_eq!(part1(TEST_INPUT), 288);
    }
    #[test]
    fn test_puzzle_input_part1() {
        assert_eq!(part1(PUZZLE_INPUT), 449820);
    }
    #[test]
    fn test_input_part2() {
        assert_eq!(part2(TEST_INPUT), 71503);
    }
    #[test]
    fn test_puzzle_input_part2() {
        assert_eq!(part2(PUZZLE_INPUT), 42250895);
    }
}
