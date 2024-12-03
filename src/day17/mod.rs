use std::ops::{Index, IndexMut};

pub const PUZZLE_INPUT: &str = include_str!("input.txt");

pub fn part1(input: &str) -> usize {
    City::parse(input).shortest_path()
}

pub fn part2(input: &str) -> usize {
    todo!();
}

struct City {
    height: usize,
    width: usize,
    blocks: Vec<u32>,
}

impl City {
    fn parse(input: &str) -> Self {
        let width = input.find('\n').unwrap();
        let blocks: Vec<_> = input
            .split('\n')
            .flat_map(|row| row.chars().map(|c| c.to_digit(10).unwrap()))
            .collect();
        let height = blocks.len() / width;

        Self {
            width,
            blocks,
            height,
        }
    }

    fn shortest_path(&self) -> usize {
        let mut nodes = Nodes::new(self.width, self.height);

        for _ in 0..(self.width * self.height * 9) {
            let Some(min_node) = nodes.min() else {
                break;
            };
            if min_node.position == Position(self.width - 1, self.height - 1) {
                break;
            }
            let min_node = min_node.clone();
            for direction in Direction::ALL
                .into_iter()
                .filter(|direction| min_node.can_move(*direction, self.width, self.height))
            {
                let position = min_node.position.move_next(direction);
                for (heat_loss, incoming_dir, mut steps_in_same_direction) in
                    min_node.heat_loss_iter()
                {
                    let new_node = &mut nodes[position];
                    let new_steps_in_same_direction = if incoming_dir == direction {
                        match steps_in_same_direction.next() {
                            None => {
                                dbg!(&position);
                                dbg!(&direction);
                                dbg!(&incoming_dir);
                                continue;
                            }
                            Some(v) => v,
                        }
                    } else {
                        Default::default()
                    };
                    if new_node.heat_loss(direction, new_steps_in_same_direction)
                        > &mut heat_loss.saturating_add(self[position])
                    {
                        *new_node.heat_loss(direction, new_steps_in_same_direction) =
                            heat_loss + self[position];
                        nodes.mark_unvisited(position, new_steps_in_same_direction, direction);
                    }
                    nodes.mark_visited(min_node.position, steps_in_same_direction, direction);
                }
            }
        }
        //dbg!(nodes.nodes.iter().collect::<Vec<_>>());
        dbg!(&nodes[Position(self.width - 1, self.height - 1)])
            .min_heat_loss()
            .0 as usize
    }
}

struct Nodes {
    nodes: Vec<Node>,
    width: usize,
    visited: Vec<[bool; 12]>,
}

impl Nodes {
    fn new(width: usize, height: usize) -> Self {
        let mut nodes = Vec::with_capacity(height * width);
        for y in 0..height {
            for x in 0..width {
                nodes.push(Node::new(Position(x, y)));
            }
        }
        let visited = vec![[false; 12]; height * width];

        nodes[0] = Node::initial();
        Self {
            nodes,
            width,
            visited,
        }
    }

    fn min(&self) -> Option<&Node> {
        self.nodes
            .iter()
            .map(|node| {
                let min = node
                    .heat_loss_iter()
                    .filter(|(_, dir, steps_in_same_direction)| {
                        self.is_visited(node.position, *steps_in_same_direction, *dir)
                    })
                    .map(|(heat_loss, _, _)| heat_loss)
                    .min();
                (min, node)
            })
            .min()
            .map(|(_, node)| node)
    }

    fn mark_visited(
        &mut self,
        Position(x, y): Position,
        steps_in_same_direction: StepsInSameDirection,
        direction: Direction,
    ) {
        self.visited[y * self.width + x]
            [direction as usize * 3 + steps_in_same_direction as usize] = true;
    }
    fn mark_unvisited(
        &mut self,
        Position(x, y): Position,
        steps_in_same_direction: StepsInSameDirection,
        direction: Direction,
    ) {
        for direction in Direction::ALL.iter() {
            //.filter(|dir| *dir != &direction) {
            for step in StepsInSameDirection::ALL
                .iter()
                .filter(|step| *step <= &steps_in_same_direction)
            {
                self.visited[y * self.width + x][*direction as usize * 3 + *step as usize] = false;
            }
        }
    }
    fn is_visited(
        &self,
        Position(x, y): Position,
        steps_in_same_direction: StepsInSameDirection,
        direction: Direction,
    ) -> bool {
        self.visited[y * self.width + x][direction as usize * 3 + steps_in_same_direction as usize]
    }
}

#[derive(Default, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Debug)]
enum StepsInSameDirection {
    #[default]
    One,
    Two,
    Three,
}

impl StepsInSameDirection {
    const ALL: [Self; 3] = [Self::One, Self::Two, Self::Three];
}

impl Iterator for StepsInSameDirection {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            StepsInSameDirection::One => Some(StepsInSameDirection::Two),
            StepsInSameDirection::Two => Some(StepsInSameDirection::Three),
            StepsInSameDirection::Three => None,
        }
    }
}

impl From<usize> for StepsInSameDirection {
    fn from(value: usize) -> Self {
        match value {
            0 => StepsInSameDirection::One,
            1 => StepsInSameDirection::Two,
            2 => StepsInSameDirection::Three,
            _ => panic!("Cannot convert number greater than 3"),
        }
    }
}

impl Index<Position> for Nodes {
    type Output = Node;

    fn index(&self, Position(x, y): Position) -> &Self::Output {
        &self.nodes[y * self.width + x]
    }
}
impl IndexMut<Position> for Nodes {
    fn index_mut(&mut self, Position(x, y): Position) -> &mut Self::Output {
        &mut self.nodes[y * self.width + x]
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Node {
    position: Position,
    heat_loss: [[u32; 3]; 4],
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.min_heat_loss().0.cmp(&other.min_heat_loss().0)
    }
}

impl Node {
    fn new(position: Position) -> Self {
        Self {
            position,
            heat_loss: [[u32::MAX; 3]; 4],
        }
    }
    fn initial() -> Self {
        Self {
            position: Position::default(),
            heat_loss: [[0, u32::MAX, u32::MAX]; 4],
        }
    }

    fn can_move(&self, direction: Direction, max_len: usize, max_height: usize) -> bool {
        self.position.can_move(direction, max_len, max_height)
    }

    fn min_heat_loss(&self) -> (u32, Direction, StepsInSameDirection) {
        self.heat_loss_iter().min().unwrap()
    }
    fn heat_loss_iter(&self) -> impl Iterator<Item = (u32, Direction, StepsInSameDirection)> + '_ {
        self.heat_loss.iter().enumerate().flat_map(|(index, row)| {
            row.iter()
                .enumerate()
                .map(move |(steps_in_same_direction, heat_loss)| {
                    (
                        *heat_loss,
                        Direction::ALL[index],
                        steps_in_same_direction.into(),
                    )
                })
        })
    }
    fn heat_loss(&mut self, direction: Direction, visit_number: StepsInSameDirection) -> &mut u32 {
        &mut self.heat_loss[direction as usize][visit_number as usize]
    }
}

impl Index<Position> for City {
    type Output = u32;

    fn index(&self, Position(x, y): Position) -> &Self::Output {
        &self.blocks[y * self.width + x]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    const ALL: [Direction; 4] = [Self::North, Self::East, Self::South, Self::West];
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, Hash)]
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

    const TEST_INPUT: &str = r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#;

    #[test]
    fn test_input_part1() {
        assert_eq!(part1(TEST_INPUT), 102);
    }
    #[test]
    fn test_puzzle_input_part1() {
        // assert_eq!(part1(PUZZLE_INPUT), 0);
        // less than 883
        // greater than 864
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
