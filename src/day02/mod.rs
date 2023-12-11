use std::str::FromStr;

pub const PUZZLE_INPUT: &str = include_str!("input.txt");

const CONSTRAINT: Collection = Collection {
    red: 12,
    green: 13,
    blue: 14,
};

pub fn part1(input: &str) -> i32 {
    input
        .split('\n')
        .map(Game::parse)
        .filter(|game| game.possible(CONSTRAINT))
        .map(|game| game.id)
        .sum()
}

pub fn part2(input: &str) -> i32 {
    input
        .split('\n')
        .map(Game::parse)
        .map(Game::min_possible)
        .map(Collection::power)
        .sum()
}

#[derive(Clone, Debug)]
struct Game {
    id: i32,
    attempts: Vec<Collection>,
}

impl Game {
    fn parse(input: &str) -> Self {
        let (game, games) = input.split_once(": ").expect("No colon");
        let (_, id) = game.split_once(' ').expect("No space between game and id");

        Game {
            id: id.parse::<i32>().expect("Couldn't parse id as input"),
            attempts: games.split("; ").map(Collection::parse).collect(),
        }
    }

    fn possible(&self, constraint: Collection) -> bool {
        self.attempts.iter().fold(true, |acc, attempt| {
            acc && attempt.red <= constraint.red
                && attempt.blue <= constraint.blue
                && attempt.green <= constraint.green
        })
    }

    fn min_possible(self) -> Collection {
        self.attempts
            .iter()
            .fold(Collection::default(), |acc, attempt| acc.max(attempt))
    }
}

#[derive(Clone, Copy, Default, Debug)]
struct Collection {
    red: i32,
    green: i32,
    blue: i32,
}

impl Collection {
    fn parse(input: &str) -> Self {
        let mut collection = Self::default();
        input
            .split(", ")
            .map(Self::parse_color)
            .for_each(|(value, color)| collection.with_color(color, value));

        collection
    }

    fn parse_color(input: &str) -> (i32, Color) {
        let (val, color) = input
            .split_once(' ')
            .expect("No space between color and value");
        (
            val.parse::<i32>().unwrap(),
            Color::from_str(color).expect("Couldn't parse color"),
        )
    }

    fn with_color(&mut self, color: Color, value: i32) {
        match color {
            Color::Red => self.red = value,
            Color::Green => self.green = value,
            Color::Blue => self.blue = value,
        }
    }

    fn max(&self, other: &Self) -> Self {
        Self {
            red: std::cmp::max(self.red, other.red),
            green: std::cmp::max(self.green, other.green),
            blue: std::cmp::max(self.blue, other.blue),
        }
    }

    fn power(self) -> i32 {
        self.red * self.blue * self.green
    }
}

enum Color {
    Red,
    Green,
    Blue,
}

impl FromStr for Color {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            "blue" => Ok(Self::Blue),
            _ => Err("No match"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    #[test]
    fn test_input_part1() {
        assert_eq!(part1(TEST_INPUT), 8);
    }

    #[test]
    fn actual_input_part1() {
        assert_eq!(part1(PUZZLE_INPUT), 2331);
    }

    #[test]
    fn test_input_part2() {
        assert_eq!(part2(TEST_INPUT), 2286);
    }

    #[test]
    fn actual_input_part2() {
        assert_eq!(part2(PUZZLE_INPUT), 71585);
    }
}
