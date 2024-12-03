pub const PUZZLE_INPUT: &str = include_str!("input.txt");

pub fn part1(input: &str) -> usize {
    Instructions::parse(input).area()
}

pub fn part2(input: &str) -> usize {
    Instructions::parse_part2(input).area()
}

struct Instructions {
    instructions: Vec<Instruction>,
}

impl Instructions {
    fn parse(input: &str) -> Self {
        let instructions = input.split('\n').map(Instruction::parse).collect();
        Self { instructions }
    }
    fn parse_part2(input: &str) -> Self {
        let instructions = input.split('\n').map(Instruction::parse_part2).collect();
        Self { instructions }
    }
    fn area(&self) -> usize {
        let mut area = 0;
        let mut perimeter = 0;

        let mut x = 0;
        let mut y = 0;
        for Instruction {
            direction, amount, ..
        } in self.instructions.iter()
        {
            // Shoelace formula
            match direction {
                Direction::Left => {
                    area += amount * y;
                    x -= amount;
                }
                Direction::Right => {
                    area -= amount * y;
                    x += amount;
                }
                Direction::Up => {
                    area += amount * x;
                    y += amount;
                }
                Direction::Down => {
                    area -= amount * x;
                    y -= amount;
                }
            }
            perimeter += amount;
        }
        // Since the shoelace formula will be missing the thick 'boarder',
        // we add half the perimeter plus one for the exterior corners
        ((area.unsigned_abs() + perimeter.unsigned_abs()) / 2 + 1) as usize
    }
}

struct Instruction {
    amount: i64,
    direction: Direction,
}

impl Instruction {
    fn parse(input: &str) -> Self {
        let (dir, rest) = input.split_once(' ').unwrap();
        let (amount, _) = rest.split_once(' ').unwrap();

        Self {
            direction: Direction::parse(dir),
            amount: amount.parse().unwrap(),
        }
    }

    fn parse_part2(input: &str) -> Self {
        let instruction = input
            .split(' ')
            .last()
            .unwrap()
            .strip_prefix("(#")
            .unwrap()
            .strip_suffix(')')
            .unwrap();

        Self {
            direction: Direction::parse(&instruction[5..]),
            amount: i64::from_str_radix(&instruction[0..5], 16).unwrap(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn parse(input: &str) -> Self {
        input.chars().map(Self::from_char).next().unwrap()
    }
    fn from_char(c: char) -> Self {
        match c {
            'R' => Self::Right,
            'L' => Self::Left,
            'U' => Self::Up,
            'D' => Self::Down,
            '0' => Self::Right,
            '1' => Self::Down,
            '2' => Self::Left,
            '3' => Self::Up,
            _ => panic!("Unrecognized direction"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"#;

    #[test]
    fn test_input_part1() {
        assert_eq!(part1(TEST_INPUT), 62);
    }
    #[test]
    fn test_puzzle_input_part1() {
        assert_eq!(part1(PUZZLE_INPUT), 56923);
    }
    #[test]
    fn test_input_part2() {
        assert_eq!(part2(TEST_INPUT), 952408144115);
    }
    #[test]
    fn test_puzzle_input_part2() {
        assert_eq!(part2(PUZZLE_INPUT), 66296566363189);
    }
}
