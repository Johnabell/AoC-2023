use num::Integer;
use std::collections::HashMap;

pub const PUZZLE_INPUT: &str = include_str!("input.txt");

pub fn part1(input: &str) -> u32 {
    Map::parse(input).follow()
}

pub fn part2(input: &str) -> u128 {
    Map::parse(input).follow_part2()
}

struct Map<'a> {
    directions: Directions<'a>,
    nodes: HashMap<&'a str, Node<'a>>,
}

impl<'a> Map<'a> {
    const START_NODE: &'static str = "AAA";
    const END_NODE: &'static str = "ZZZ";
    fn parse(input: &'a str) -> Self {
        let (dir, nodes) = input.split_once("\n\n").unwrap();

        Self {
            directions: Directions {
                directions: dir.as_bytes(),
                position: 0,
            },
            nodes: nodes
                .split('\n')
                .map(Node::parse)
                .map(|node| (node.name, node))
                .collect(),
        }
    }

    fn follow(self) -> u32 {
        let mut node = self.nodes.get(Self::START_NODE).unwrap();

        for (i, direction) in self.directions.enumerate() {
            node = self.nodes.get(node.child(direction)).unwrap();
            if node.name == Self::END_NODE {
                return (i + 1) as u32;
            }
        }
        unreachable!()
    }

    fn follow_part2(self) -> u128 {
        let mut nodes: Vec<_> = self
            .nodes
            .iter()
            .filter_map(|(_key, value)| {
                if value.is_start_node() {
                    Some(value)
                } else {
                    None
                }
            })
            .collect();
        let mut visited_z_nodes = vec![vec![]; nodes.len()];
        let total_length = self.nodes.len() * self.directions.directions.len() + 1;
        for (i, direction) in self.directions.enumerate().take(total_length) {
            nodes = nodes
                .into_iter()
                .map(|node| self.nodes.get(node.child(direction)).unwrap())
                .collect();
            for (j, node) in nodes.iter().enumerate() {
                if node.is_end_node() {
                    visited_z_nodes[j].push((i + 1) as u128);
                }
            }
        }
        // We might be able to do this last bit more efficiently by checking for when cycles end
        *visited_z_nodes
            .into_iter()
            .reduce(|acc, next| Self::lcms(&acc, &next))
            .unwrap()
            .iter()
            .min()
            .unwrap()
    }

    fn lcms<T: Integer>(a: &[T], b: &[T]) -> Vec<T> {
        a.iter().flat_map(|i| b.iter().map(|j| j.lcm(i))).collect()
    }
}

struct Node<'a> {
    name: &'a str,
    left: &'a str,
    right: &'a str,
}

impl<'a> Node<'a> {
    fn parse(input: &'a str) -> Self {
        let (name, children) = input.split_once(" = ").unwrap();
        let (left, right) = children
            .strip_prefix('(')
            .unwrap()
            .strip_suffix(')')
            .unwrap()
            .split_once(", ")
            .unwrap();
        Self { name, left, right }
    }

    fn child(&self, direction: Direction) -> &'a str {
        match direction {
            Direction::Left => self.left,
            Direction::Right => self.right,
        }
    }

    fn is_start_node(&self) -> bool {
        self.name.ends_with('A')
    }

    fn is_end_node(&self) -> bool {
        self.name.ends_with('Z')
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
}

impl From<u8> for Direction {
    fn from(value: u8) -> Self {
        match value {
            b'L' => Self::Left,
            b'R' => Self::Right,
            _ => panic!("Unrecognised direction"),
        }
    }
}

impl Direction {}

struct Directions<'a> {
    directions: &'a [u8],
    position: usize,
}

impl<'a> Iterator for Directions<'a> {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        let next = Some(self.directions[self.position].into());
        self.position = (self.position + 1) % self.directions.len();
        next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;
    const TEST_INPUT_2: &str = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;

    const TEST_INPUT_PART_2: &str = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;

    #[test]
    fn test_input1_part1() {
        assert_eq!(part1(TEST_INPUT_1), 2);
    }
    #[test]
    fn test_input2_part1() {
        assert_eq!(part1(TEST_INPUT_2), 6);
    }
    #[test]
    fn test_puzzele_input_part1() {
        assert_eq!(part1(PUZZLE_INPUT), 16579);
    }
    #[test]
    fn test_input_part2() {
        assert_eq!(part2(TEST_INPUT_PART_2), 6);
    }
    #[test]
    fn test_puzzele_input_part2() {
        assert_eq!(part2(PUZZLE_INPUT), 12927600769609);
    }
}
