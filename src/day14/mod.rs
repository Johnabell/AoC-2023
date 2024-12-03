use std::collections::HashMap;

pub const PUZZLE_INPUT: &str = include_str!("input.txt");

pub fn part1(input: &str) -> usize {
    Platform::parse(input).total()
}

pub fn part2(input: &str) -> usize {
    Platform::parse(input).run()
}

struct Platform {
    rocks: Vec<Vec<Rock>>,
}

impl Platform {
    fn parse(input: &str) -> Self {
        let rocks = input
            .split('\n')
            .map(|line| line.chars().map(Rock::from_char).collect::<Vec<_>>())
            .collect();

        Self { rocks }
    }

    fn total(&self) -> usize {
        let mut total = 0;
        for column in 0..self.width() {
            let mut weight = self.rocks.len();
            for (row, rock) in self.column_iter(column).enumerate() {
                match rock {
                    Rock::Round => {
                        total += weight;
                        weight -= 1;
                    }
                    Rock::Square => weight = self.rocks.len() - row - 1,
                    Rock::None => {}
                }
            }
        }

        total
    }

    fn run(&mut self) -> usize {
        let mut history = HashMap::new();
        let mut cycle_length: isize = -1;
        let mut cycle = Vec::new();
        for i in 0..1000000 {
            self.cycle();
            if i < 10 {
                continue;
            }
            let value = self.total_north_load();
            let value_history = history.entry(value).or_default();
            match value_history {
                (last_seen, None) => *value_history = (i, Some(i - *last_seen)),
                (last_seen, Some(diff)) => {
                    let new_diff = i - *last_seen;
                    if new_diff == *diff || cycle_length > 0 {
                        if new_diff <= cycle_length {
                            cycle.push(value);
                        } else {
                            cycle_length = new_diff;
                            cycle.clear();
                        }
                    }
                    *value_history = (i, Some(new_diff));
                }
            }
            if cycle.len() == cycle_length as usize {
                return cycle[(999999999 - i as usize - 1) % cycle_length as usize];
            }
        }
        0
    }

    fn cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }

    fn tilt_north(&mut self) {
        for column in 0..self.width() {
            let mut position = 0;
            for row in 0..self.rocks.len() {
                let rock = self.rocks[row][column];
                match rock {
                    Rock::Round => {
                        if position != row {
                            self.rocks[position][column] = Rock::Round;
                            self.rocks[row][column] = Rock::None;
                        }
                        position += 1;
                    }
                    Rock::Square => position = row + 1,
                    Rock::None => {}
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        for column in 0..self.width() {
            let mut position = self.rocks.len() - 1;
            for row in (0..self.rocks.len()).rev() {
                let rock = self.rocks[row][column];
                match rock {
                    Rock::Round => {
                        if position != row {
                            self.rocks[position][column] = Rock::Round;
                            self.rocks[row][column] = Rock::None;
                        }
                        position = position.saturating_sub(1);
                    }
                    Rock::Square => position = row.saturating_sub(1),
                    Rock::None => {}
                }
            }
        }
    }

    fn tilt_west(&mut self) {
        for row in 0..self.rocks.len() {
            let mut position = 0;
            for column in 0..self.width() {
                let rock = self.rocks[row][column];
                match rock {
                    Rock::Round => {
                        if position != column {
                            self.rocks[row][position] = Rock::Round;
                            self.rocks[row][column] = Rock::None;
                        }
                        position += 1;
                    }
                    Rock::Square => position = column + 1,
                    Rock::None => {}
                }
            }
        }
    }

    fn tilt_east(&mut self) {
        for row in 0..self.rocks.len() {
            let mut position = self.width() - 1;
            for column in (0..self.width()).rev() {
                let rock = self.rocks[row][column];
                match rock {
                    Rock::Round => {
                        if position != column {
                            self.rocks[row][position] = Rock::Round;
                            self.rocks[row][column] = Rock::None;
                        }
                        position = position.saturating_sub(1);
                    }
                    Rock::Square => position = column.saturating_sub(1),
                    Rock::None => {}
                }
            }
        }
    }

    fn total_north_load(&self) -> usize {
        let len = self.rocks.len();
        self.rocks
            .iter()
            .enumerate()
            .map(|(index, row)| {
                row.iter()
                    .filter(|rock| matches!(rock, Rock::Round))
                    .count()
                    * (len - index)
            })
            .sum()
    }

    fn width(&self) -> usize {
        self.rocks.first().unwrap().len()
    }

    fn column_iter(&self, index: usize) -> impl Iterator<Item = Rock> + '_ {
        self.rocks.iter().map(move |row| row[index])
    }
}

#[derive(Clone, Copy, Debug)]
enum Rock {
    Round,
    Square,
    None,
}

impl Rock {
    fn from_char(c: char) -> Self {
        match c {
            'O' => Self::Round,
            '#' => Self::Square,
            _ => Self::None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;

    #[test]
    fn test_input_part1() {
        assert_eq!(part1(TEST_INPUT), 136);
    }
    #[test]
    fn test_puzzle_input_part1() {
        assert_eq!(part1(PUZZLE_INPUT), 111979);
    }
    #[test]
    fn test_input_part2() {
        assert_eq!(part2(TEST_INPUT), 64);
    }
    #[test]
    fn test_puzzle_input_part2() {
        assert_eq!(part2(PUZZLE_INPUT), 102055);
    }
}
