use std::{
    collections::HashSet,
    ops::{Index, IndexMut},
};

pub const PUZZLE_INPUT: &str = include_str!("input.txt");

pub fn part1(input: &str) -> usize {
    Map::parse(input).mid_point()
}

pub fn part2(input: &str) -> usize {
    Map::parse(input).enclosed()
}

struct Map {
    map: Vec<Vec<Pipe>>,
}

impl Map {
    fn parse(input: &str) -> Self {
        Self {
            map: input
                .split('\n')
                .map(|line| line.chars().map(Pipe::parse).collect())
                .collect(),
        }
    }

    fn mid_point(&self) -> usize {
        let mut count = 0;
        self.walk_path(|_| count += 1);
        count / 2
    }

    fn walk_path<F>(&self, mut on_section: F)
    where
        F: FnMut(Position),
    {
        let (mut position, mut direction) = self.initial_position_and_direction();
        // Move off start
        position = position.move_next(direction);
        on_section(position);
        let mut pipe = self[position];
        direction = pipe.outgoing_direction(direction);

        loop {
            position = position.move_next(direction);
            pipe = self[position];
            on_section(position);
            if pipe == Pipe::Start {
                break;
            }
            direction = pipe.outgoing_direction(direction);
        }
    }

    fn start(&self) -> Position {
        self.iter()
            .find(|(_, pipe)| pipe == &Pipe::Start)
            .unwrap()
            .0
    }

    fn iter(&self) -> impl Iterator<Item = (Position, Pipe)> + '_ {
        self.map.iter().enumerate().flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, pipe)| (Position(x, y), *pipe))
        })
    }

    fn initial_position_and_direction(&self) -> (Position, Direction) {
        let position = self.start();
        let width = self.width();

        for direction in Direction::ALL {
            if position.can_move(direction, width)
                && self[position.move_next(direction)].connects(direction.opposite())
            {
                return (position, direction);
            }
        }

        (position, Direction::South)
    }

    fn width(&self) -> usize {
        self.map.first().unwrap().len()
    }

    fn start_tile_pipe(&self) -> Pipe {
        let position = self.start();
        let width = self.width();
        let mut first_direction = None;

        for direction in Direction::ALL {
            if position.can_move(direction, width)
                && self[position.move_next(direction)].connects(direction.opposite())
            {
                match first_direction {
                    Some(first_direction) => return Pipe::connecting(first_direction, direction),
                    None => first_direction = Some(direction),
                }
            }
        }
        unreachable!();
    }

    fn enclosed(mut self) -> usize {
        let mut path = HashSet::new();
        let mut enclosed = 0;
        self.walk_path(|position| {
            path.insert(position);
        });
        let start = self.start();
        self[start] = self.start_tile_pipe();
        for (position @ Position(x, y), _) in self.iter() {
            if !path.contains(&position) {
                let is_enclosed = (0..y)
                    .map(|i| Position(x, i))
                    .filter(|pos| path.contains(pos))
                    .fold(0, |acc, pos| acc + self[pos].west_count())
                    % 2
                    == 1;
                if is_enclosed {
                    enclosed += 1;
                }
            }
        }

        enclosed
    }
}

impl Index<Position> for Map {
    type Output = Pipe;

    fn index(&self, Position(x, y): Position) -> &Self::Output {
        &self.map[y][x]
    }
}
impl IndexMut<Position> for Map {
    fn index_mut(&mut self, Position(x, y): Position) -> &mut Self::Output {
        &mut self.map[y][x]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Pipe {
    None,
    NorthSouth,
    NorthEast,
    NorthWest,
    EastWest,
    SouthEast,
    SouthWest,
    Start,
}

impl Pipe {
    fn parse(c: char) -> Self {
        match c {
            '|' => Self::NorthSouth,
            '-' => Self::EastWest,
            'L' => Self::NorthEast,
            'J' => Self::NorthWest,
            '7' => Self::SouthWest,
            'F' => Self::SouthEast,
            '.' => Self::None,
            'S' => Self::Start,
            _ => panic!(),
        }
    }

    fn connecting(a: Direction, b: Direction) -> Self {
        match (a, b) {
            (Direction::South, Direction::North) | (Direction::North, Direction::South) => {
                Self::NorthSouth
            }
            (Direction::East, Direction::North) | (Direction::North, Direction::East) => {
                Self::NorthEast
            }
            (Direction::West, Direction::North) | (Direction::North, Direction::West) => {
                Self::NorthWest
            }
            (Direction::East, Direction::South) | (Direction::South, Direction::East) => {
                Self::SouthEast
            }
            (Direction::West, Direction::South) | (Direction::South, Direction::West) => {
                Self::SouthWest
            }
            (Direction::West, Direction::East) | (Direction::East, Direction::West) => {
                Self::EastWest
            }
            _ => Pipe::None,
        }
    }

    fn outgoing_direction(&self, incoming_direction: Direction) -> Direction {
        match (self, incoming_direction) {
            (Pipe::NorthSouth, Direction::North) => Direction::North,
            (Pipe::NorthSouth, Direction::South) => Direction::South,
            (Pipe::NorthEast, Direction::South) => Direction::East,
            (Pipe::NorthEast, Direction::West) => Direction::North,
            (Pipe::NorthWest, Direction::South) => Direction::West,
            (Pipe::NorthWest, Direction::East) => Direction::North,
            (Pipe::EastWest, Direction::East) => Direction::East,
            (Pipe::EastWest, Direction::West) => Direction::West,
            (Pipe::SouthEast, Direction::North) => Direction::East,
            (Pipe::SouthEast, Direction::West) => Direction::South,
            (Pipe::SouthWest, Direction::North) => Direction::West,
            (Pipe::SouthWest, Direction::East) => Direction::South,
            _ => panic!("Unexpected direction {incoming_direction:?} with pipe {self:?}"),
        }
    }

    fn connects(&self, direction: Direction) -> bool {
        matches!(
            (self, direction),
            (Pipe::NorthSouth, Direction::North)
                | (Pipe::NorthSouth, Direction::South)
                | (Pipe::NorthEast, Direction::North)
                | (Pipe::NorthEast, Direction::East)
                | (Pipe::NorthWest, Direction::North)
                | (Pipe::NorthWest, Direction::West)
                | (Pipe::EastWest, Direction::East)
                | (Pipe::EastWest, Direction::West)
                | (Pipe::SouthEast, Direction::South)
                | (Pipe::SouthEast, Direction::East)
                | (Pipe::SouthWest, Direction::South)
                | (Pipe::SouthWest, Direction::West)
        )
    }

    fn west_count(&self) -> usize {
        match self {
            Pipe::EastWest | Pipe::NorthWest | Pipe::SouthWest => 1,
            _ => 0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    const ALL: [Self; 4] = [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ];
    fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct Position(usize, usize);

impl Position {
    fn move_next(&self, direction: Direction) -> Self {
        match direction {
            Direction::North => Position(self.0, self.1 - 1),
            Direction::South => Position(self.0, self.1 + 1),
            Direction::East => Position(self.0 + 1, self.1),
            Direction::West => Position(self.0 - 1, self.1),
        }
    }

    fn can_move(&self, direction: Direction, max_len: usize) -> bool {
        match direction {
            Direction::North => self.1 > 0,
            Direction::South => self.1 < max_len - 1,
            Direction::East => self.0 < max_len - 1,
            Direction::West => self.0 > 0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ"#;

    const TEST_INPUT_PART2: &str = r#"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."#;

    const TEST_INPUT_PART2_2: &str = r#".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."#;

    const TEST_INPUT_PART2_3: &str = r#"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"#;

    #[test]
    fn test_input_part1() {
        assert_eq!(part1(TEST_INPUT), 8);
    }
    #[test]
    fn test_puzzle_input_part1() {
        assert_eq!(part1(PUZZLE_INPUT), 6757);
    }
    #[test]
    fn test_input_part2() {
        assert_eq!(part2(TEST_INPUT), 1);
    }
    #[test]
    fn test_input_part2_1() {
        assert_eq!(part2(TEST_INPUT_PART2), 4);
    }
    #[test]
    fn test_input_part2_2() {
        assert_eq!(part2(TEST_INPUT_PART2_2), 8);
    }
    #[test]
    fn test_input_part2_3() {
        assert_eq!(part2(TEST_INPUT_PART2_3), 10);
    }
    #[test]
    fn test_puzzle_input_part2() {
        assert_eq!(part2(PUZZLE_INPUT), 523);
    }
}
