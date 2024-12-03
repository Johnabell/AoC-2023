use std::ops::{Index, IndexMut};

use crate::two_iter::TwoIter;

pub const PUZZLE_INPUT: &str = include_str!("input.txt");

pub fn part1(input: &str) -> usize {
    LightMaze::parse(input).energized()
}

pub fn part2(input: &str) -> usize {
    LightMaze::parse(input).max_energized()
}

struct LightMaze {
    instruments: Vec<Vec<Instrument>>,
}

struct VisitedCache {
    visited: Vec<Visited>,
    width: usize,
}

impl VisitedCache {
    fn new(width: usize, height: usize) -> Self {
        Self {
            visited: vec![Visited::None; width * height],
            width,
        }
    }
    fn len(&self) -> usize {
        self.visited
            .iter()
            .filter(|v| !matches!(v, Visited::None))
            .count()
    }
}

impl Index<Position> for VisitedCache {
    type Output = Visited;

    fn index(&self, Position(x, y): Position) -> &Self::Output {
        &self.visited[y * self.width + x]
    }
}

impl IndexMut<Position> for VisitedCache {
    fn index_mut(&mut self, Position(x, y): Position) -> &mut Self::Output {
        &mut self.visited[y * self.width + x]
    }
}

impl LightMaze {
    fn parse(input: &str) -> Self {
        let instruments = input
            .split('\n')
            .map(|row| row.chars().map(Instrument::from_char).collect::<Vec<_>>())
            .collect();
        Self { instruments }
    }

    fn width(&self) -> usize {
        self.instruments.first().unwrap().len()
    }

    fn energized(&self) -> usize {
        let mut visited = VisitedCache::new(self.width(), self.instruments.len());
        self.walk_path(Position::default(), Direction::East, &mut visited);
        visited.len()
    }

    fn max_energized(&self) -> usize {
        Direction::ALL
            .iter()
            .map(|initial_dir| {
                (0..self.width())
                    .map(|i| {
                        let mut visited = VisitedCache::new(self.width(), self.instruments.len());
                        let initial_position = match initial_dir {
                            Direction::North => Position(i, self.instruments.len() - 1),
                            Direction::East => Position(0, i),
                            Direction::South => Position(i, 0),
                            Direction::West => Position(self.width() - 1, i),
                        };
                        self.walk_path(initial_position, *initial_dir, &mut visited);
                        visited.len()
                    })
                    .max()
                    .unwrap()
            })
            .max()
            .unwrap()
    }

    fn walk_path(&self, position: Position, direction: Direction, visited: &mut VisitedCache) {
        let visit_type = self[position].as_visited(direction);
        match visited[position] {
            Visited::Both => return,
            Visited::None => visited[position] = visit_type,
            v if v == visit_type => return,
            _ => visited[position] = Visited::Both,
        }
        self[position]
            .out_directions(direction)
            .for_each(|next_direction| {
                if position.can_move(next_direction, self.width(), self.instruments.len()) {
                    let next_position = position.move_next(next_direction);
                    self.walk_path(next_position, next_direction, visited);
                }
            });
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Visited {
    Horizontal,
    Vertical,
    Both,
    None,
}

impl Index<Position> for LightMaze {
    type Output = Instrument;

    fn index(&self, Position(x, y): Position) -> &Self::Output {
        &self.instruments[y][x]
    }
}

#[derive(Debug, Clone, Copy)]
enum Instrument {
    VSplitter,
    HSplitter,
    NWSEMirror,
    NESWMirror,
    None,
}

impl Instrument {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::None,
            '|' => Self::VSplitter,
            '-' => Self::HSplitter,
            '\\' => Self::NWSEMirror,
            '/' => Self::NESWMirror,
            _ => panic!("Unknown instrument"),
        }
    }

    fn out_directions(&self, input_direction: Direction) -> OutDirections {
        match (self, input_direction) {
            (Instrument::VSplitter, Direction::West) | (Instrument::VSplitter, Direction::East) => {
                OutDirections::Two(Direction::North, Direction::South)
            }
            (Instrument::VSplitter, direction) => OutDirections::One(direction),
            (Instrument::HSplitter, Direction::South)
            | (Instrument::HSplitter, Direction::North) => {
                OutDirections::Two(Direction::East, Direction::West)
            }
            (Instrument::HSplitter, direction) => OutDirections::One(direction),
            (Instrument::NWSEMirror, Direction::North) => OutDirections::One(Direction::West),
            (Instrument::NWSEMirror, Direction::East) => OutDirections::One(Direction::South),
            (Instrument::NWSEMirror, Direction::South) => OutDirections::One(Direction::East),
            (Instrument::NWSEMirror, Direction::West) => OutDirections::One(Direction::North),
            (Instrument::NESWMirror, Direction::North) => OutDirections::One(Direction::East),
            (Instrument::NESWMirror, Direction::East) => OutDirections::One(Direction::North),
            (Instrument::NESWMirror, Direction::South) => OutDirections::One(Direction::West),
            (Instrument::NESWMirror, Direction::West) => OutDirections::One(Direction::South),
            (Instrument::None, direction) => OutDirections::One(direction),
        }
    }

    fn as_visited(&self, direction: Direction) -> Visited {
        match (self, direction) {
            (Self::NWSEMirror, Direction::East | Direction::North) => Visited::Vertical,
            (Self::NWSEMirror, _) => Visited::Horizontal,
            (Self::NESWMirror, Direction::East | Direction::South) => Visited::Horizontal,
            (Self::NESWMirror, _) => Visited::Vertical,
            (_, direction) => direction.as_visited(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    const ALL: [Direction; 4] = [Self::North, Self::East, Self::South, Self::West];
    fn as_visited(&self) -> Visited {
        match self {
            Direction::North | Direction::South => Visited::Vertical,
            Direction::East | Direction::West => Visited::Horizontal,
        }
    }
}

type OutDirections = TwoIter<Direction>;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
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

    fn can_move(&self, direction: Direction, max_len: usize, max_height: usize) -> bool {
        match direction {
            Direction::North => self.1 > 0,
            Direction::South => self.1 < max_height - 1,
            Direction::East => self.0 < max_len - 1,
            Direction::West => self.0 > 0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

    #[test]
    fn test_input_part1() {
        assert_eq!(part1(TEST_INPUT), 46);
    }
    #[test]
    fn test_puzzle_input_part1() {
        assert_eq!(part1(PUZZLE_INPUT), 6361);
    }
    #[test]
    fn test_input_part2() {
        assert_eq!(part2(TEST_INPUT), 51);
    }
    #[test]
    fn test_puzzle_input_part2() {
        assert_eq!(part2(PUZZLE_INPUT), 6701);
    }
}
