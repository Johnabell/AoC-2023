use std::str::FromStr;

pub const PUZZLE_INPUT: &str = include_str!("input.txt");

pub fn part1(input: &str) -> usize {
    Stack::parse(input).disintegratable()
}

pub fn part2(input: &str) -> usize {
    todo!();
}

#[derive(Debug)]
struct Stack {
    blocks: Vec<Block>,
}

impl Stack {
    fn parse(input: &str) -> Self {
        let blocks = input.split('\n').map(Block::parse).collect();
        Self { blocks }
    }

    fn disintegratable(&mut self) -> usize {
        self.settle();
        dbg!(&self);
        self.calculate_disintigratable()
    }

    fn settle(&mut self) {
        self.blocks.sort_by_key(|block_1| block_1.min_z());
    }

    fn calculate_disintigratable(&self) -> usize {
        todo!()
    }
}

#[derive(Debug)]
struct Block {
    start: Coordinate,
    end: Coordinate,
}

impl Block {
    fn parse(input: &str) -> Self {
        let (start, end) = input.split_once('~').unwrap();
        Self {
            start: Coordinate::parse(start),
            end: Coordinate::parse(end),
        }
    }

    fn min_z(&self) -> u32 {
        std::cmp::min(self.start.z, self.end.z)
    }
}

#[derive(Debug)]
struct Coordinate {
    x: u32,
    y: u32,
    z: u32,
}

impl Coordinate {
    fn parse(input: &str) -> Self {
        let mut coordinates = input.split(',').map(FromStr::from_str).map(Result::unwrap);
        let x = coordinates.next().unwrap();
        let y = coordinates.next().unwrap();
        let z = coordinates.next().unwrap();
        Self { x, y, z }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"#;

    #[test]
    fn test_input_part1() {
        assert_eq!(part1(TEST_INPUT), 5);
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
