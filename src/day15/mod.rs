use std::hash::Hasher;

pub const PUZZLE_INPUT: &str = include_str!("input.txt");

pub fn part1(input: &str) -> usize {
    input
        .split(',')
        .map(|seq| {
            let mut hasher = SimpleHasher::default();
            hasher.write(seq.as_bytes());
            hasher.finish() as usize
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let instructions = input.split(',').map(Instruction::parse);
    let mut lens_array = LensArray::new();
    lens_array.apply_instructions(instructions);
    lens_array.power()
}

struct LensArray<'a> {
    boxes: Vec<LensBox<'a>>,
}

impl<'a> LensArray<'a> {
    fn new() -> Self {
        Self {
            boxes: vec![Default::default(); 256],
        }
    }
    fn apply_instructions(&mut self, instructions: impl Iterator<Item = Instruction<'a>>) {
        instructions.for_each(|instructions| {
            self.boxes[instructions.label_hash()].apply_instruction(instructions)
        })
    }

    fn power(&self) -> usize {
        self.boxes
            .iter()
            .enumerate()
            .map(|(index, lens_box)| (index + 1) * lens_box.power())
            .sum()
    }
}

enum Instruction<'a> {
    Remove { label: &'a str },
    Add { label: &'a str, focal_length: u8 },
}

impl<'a> Instruction<'a> {
    fn parse(input: &'a str) -> Self {
        match input.split_once('-') {
            Some((label, _)) => Self::Remove { label },
            None => match input.split_once('=') {
                Some((label, focal_length)) => Self::Add {
                    label,
                    focal_length: focal_length.parse().unwrap(),
                },
                None => panic!("Failed to parse instruction"),
            },
        }
    }

    fn label(&self) -> &'a str {
        match self {
            Instruction::Remove { label } => label,
            Instruction::Add { label, .. } => label,
        }
    }

    fn label_hash(&self) -> usize {
        let mut hasher = SimpleHasher::default();
        hasher.write(self.label().as_bytes());
        hasher.finish() as usize
    }
}

#[derive(Default, Clone)]
struct LensBox<'a> {
    lenses: Vec<Lens<'a>>,
}

impl<'a> LensBox<'a> {
    fn apply_instruction(&mut self, instruction: Instruction<'a>) {
        match instruction {
            Instruction::Remove { label } => self.lenses.retain(|lens| lens.label != label),
            Instruction::Add {
                label,
                focal_length,
            } => match self.lenses.iter_mut().find(|lens| lens.label == label) {
                Some(lens) => lens.focal_length = focal_length,
                None => self.lenses.push(Lens {
                    label,
                    focal_length,
                }),
            },
        }
    }

    fn power(&self) -> usize {
        self.lenses
            .iter()
            .enumerate()
            .map(|(index, lens)| (index + 1) * lens.focal_length as usize)
            .sum()
    }
}

#[derive(Clone)]
struct Lens<'a> {
    label: &'a str,
    focal_length: u8,
}

#[derive(Default)]
struct SimpleHasher {
    value: u64,
}

impl Hasher for SimpleHasher {
    fn finish(&self) -> u64 {
        self.value
    }

    fn write(&mut self, bytes: &[u8]) {
        self.value = bytes
            .iter()
            .fold(self.value, |acc, val| ((acc + *val as u64) * 17) % 256);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#;

    #[test]
    fn test_input_part1() {
        assert_eq!(part1(TEST_INPUT), 1320);
    }
    #[test]
    fn test_puzzle_input_part1() {
        assert_eq!(part1(PUZZLE_INPUT), 507291);
    }
    #[test]
    fn test_input_part2() {
        assert_eq!(part2(TEST_INPUT), 145);
    }
    #[test]
    fn test_puzzle_input_part2() {
        assert_eq!(part2(PUZZLE_INPUT), 296921);
    }
}
