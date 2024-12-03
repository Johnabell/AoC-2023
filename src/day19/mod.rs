use std::{
    cmp::Ordering::{Equal, Greater, Less},
    collections::HashMap,
    ops::Range,
};

pub const PUZZLE_INPUT: &str = include_str!("input.txt");

pub fn part1(input: &str) -> usize {
    Sorter::parse(input).accepted_part_total()
}

pub fn part2(input: &str) -> usize {
    Sorter::parse(input).process_all()
}

struct Sorter<'a> {
    workflows: HashMap<&'a str, Workflow<'a>>,
    parts: Vec<MachinePart>,
}

impl<'a> Sorter<'a> {
    const ACCEPTED: &'static str = "A";
    const REJECTED: &'static str = "R";
    const START: &'static str = "in";

    fn parse(input: &'a str) -> Self {
        let (workflows, parts) = input.split_once("\n\n").unwrap();

        Self {
            workflows: workflows
                .split('\n')
                .map(Workflow::parse)
                .map(|workflow| (workflow.name, workflow))
                .collect(),
            parts: parts.split('\n').map(MachinePart::parse).collect(),
        }
    }

    fn accepted_part_total(&self) -> usize {
        self.parts.iter().map(|part| self.process_part(part)).sum()
    }

    fn process_part(&self, part: &MachinePart) -> usize {
        let mut workflow = &self.workflows[Self::START];
        loop {
            match workflow.map(part) {
                Self::ACCEPTED => return part.rating(),
                Self::REJECTED => return 0,
                next => workflow = &self.workflows[next],
            }
        }
    }

    fn process_all(&self) -> usize {
        let mut parts = vec![MappedRange {
            destination: "in",
            part: MachinePartRange::default(),
        }];
        let mut accepted = Vec::new();
        loop {
            match parts.pop() {
                None => break,
                Some(part) => self.workflows[part.destination]
                    .map_all(part.part)
                    .into_iter()
                    .for_each(|part| match part.destination {
                        Self::ACCEPTED => accepted.push(part.part),
                        Self::REJECTED => {}
                        _ => parts.push(part),
                    }),
            }
        }
        accepted.iter().map(MachinePartRange::rating).sum()
    }
}

struct Workflow<'a> {
    name: &'a str,
    rules: Vec<Rule<'a>>,
}

impl<'a> Workflow<'a> {
    fn parse(input: &'a str) -> Self {
        let (name, rest) = input.split_once('{').unwrap();

        Self {
            name,
            rules: rest
                .strip_suffix('}')
                .unwrap()
                .split(',')
                .map(Rule::parse)
                .collect(),
        }
    }

    fn map(&self, part: &MachinePart) -> &'a str {
        self.rules.iter().find_map(|rule| rule.map(part)).unwrap()
    }
    fn map_all(&self, part: MachinePartRange) -> Vec<MappedRange> {
        let mut mapped_ranges = Vec::new();
        let mut current_part = part;
        for rule in &self.rules {
            match rule.map_many(current_part.clone()) {
                (None, None) => {}
                (None, Some(unmapped)) => current_part = unmapped,
                (Some(mapped), None) => mapped_ranges.push(mapped),
                (Some(mapped), Some(unmapped)) => {
                    current_part = unmapped;
                    mapped_ranges.push(mapped);
                }
            }
        }
        mapped_ranges
    }
}

struct Rule<'a> {
    destination: &'a str,
    comp: Comparison,
}

impl<'a> Rule<'a> {
    fn parse(input: &'a str) -> Self {
        match input.split_once(':') {
            None => Self {
                destination: input,
                comp: Comparison::MatchAll,
            },
            Some((comparison, destination)) => Self {
                destination,
                comp: Comparison::parse(comparison),
            },
        }
    }

    fn map(&self, part: &MachinePart) -> Option<&'a str> {
        match self.comp {
            Comparison::LessThan(field, value) if part.get_field(field) < value => {
                Some(self.destination)
            }
            Comparison::GreaterThan(field, value) if part.get_field(field) > value => {
                Some(self.destination)
            }
            Comparison::MatchAll => Some(self.destination),
            _ => None,
        }
    }
    fn map_many(
        &self,
        part: MachinePartRange,
    ) -> (Option<MappedRange<'a>>, Option<MachinePartRange>) {
        match self.comp {
            Comparison::LessThan(field, value) => {
                let range = part.get_field(field).clone();
                match (range.start.cmp(&value), range.end.cmp(&value)) {
                    (_, Less) => (
                        Some(MappedRange {
                            destination: self.destination,
                            part,
                        }),
                        None,
                    ),
                    (Less, Greater | Equal) => (
                        Some(MappedRange {
                            destination: self.destination,
                            part: part.clone().set_field(field, range.start..value),
                        }),
                        Some(part.set_field(field, value..range.end)),
                    ),
                    (Greater | Equal, _) => (None, Some(part)),
                }
            }
            Comparison::GreaterThan(field, value) => {
                let range = part.get_field(field).clone();
                match (range.start.cmp(&value), range.end.cmp(&value)) {
                    (_, Less) => (None, Some(part)),
                    (Less, Greater | Equal) => (
                        Some(MappedRange {
                            destination: self.destination,
                            part: part.clone().set_field(field, (value + 1)..range.end),
                        }),
                        Some(part.set_field(field, range.start..(value + 1))),
                    ),
                    (Greater | Equal, _) => (
                        Some(MappedRange {
                            destination: self.destination,
                            part,
                        }),
                        None,
                    ),
                }
            }
            Comparison::MatchAll => (
                Some(MappedRange {
                    destination: self.destination,
                    part,
                }),
                None,
            ),
        }
    }
}

struct MappedRange<'a> {
    destination: &'a str,
    part: MachinePartRange,
}

enum Comparison {
    LessThan(Field, u32),
    GreaterThan(Field, u32),
    MatchAll,
}

impl Comparison {
    fn parse(input: &str) -> Self {
        if let Some((field, value)) = input.split_once('>') {
            Self::GreaterThan(Field::parse(field), value.parse().unwrap())
        } else if let Some((field, value)) = input.split_once('<') {
            Self::LessThan(Field::parse(field), value.parse().unwrap())
        } else {
            Self::MatchAll
        }
    }
}

#[derive(Clone, Copy)]
enum Field {
    X,
    M,
    A,
    S,
}

impl Field {
    fn parse(input: &str) -> Self {
        input.chars().map(Self::from_char).next().unwrap()
    }

    fn from_char(c: char) -> Self {
        match c {
            'x' => Self::X,
            'm' => Self::M,
            'a' => Self::A,
            's' => Self::S,
            _ => panic!("Unknown field"),
        }
    }
}

#[derive(Default, Debug)]
struct MachinePart {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl MachinePart {
    fn parse(input: &str) -> Self {
        input
            .strip_prefix('{')
            .unwrap()
            .strip_suffix('}')
            .unwrap()
            .split(',')
            .fold(Self::default(), |part, field| {
                let (name, amount) = field.split_once('=').unwrap();
                part.set_field(Field::parse(name), amount.parse().unwrap())
            })
    }

    fn set_field(mut self, field: Field, value: u32) -> Self {
        match field {
            Field::X => self.x = value,
            Field::M => self.m = value,
            Field::A => self.a = value,
            Field::S => self.s = value,
        }
        self
    }

    fn get_field(&self, field: Field) -> u32 {
        match field {
            Field::X => self.x,
            Field::M => self.m,
            Field::A => self.a,
            Field::S => self.s,
        }
    }

    fn rating(&self) -> usize {
        (self.x + self.m + self.a + self.s) as usize
    }
}

#[derive(Debug, Clone)]
struct MachinePartRange {
    x: Range<u32>,
    m: Range<u32>,
    a: Range<u32>,
    s: Range<u32>,
}

impl Default for MachinePartRange {
    fn default() -> Self {
        Self {
            x: 1..4001,
            m: 1..4001,
            a: 1..4001,
            s: 1..4001,
        }
    }
}

impl MachinePartRange {
    fn set_field(mut self, field: Field, value: Range<u32>) -> Self {
        match field {
            Field::X => self.x = value,
            Field::M => self.m = value,
            Field::A => self.a = value,
            Field::S => self.s = value,
        }
        self
    }
    fn get_field(&self, field: Field) -> &Range<u32> {
        match field {
            Field::X => &self.x,
            Field::M => &self.m,
            Field::A => &self.a,
            Field::S => &self.s,
        }
    }
    fn rating(&self) -> usize {
        self.x.len() * self.m.len() * self.a.len() * self.s.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"#;

    #[test]
    fn test_input_part1() {
        assert_eq!(part1(TEST_INPUT), 19114);
    }
    #[test]
    fn test_puzzle_input_part1() {
        assert_eq!(part1(PUZZLE_INPUT), 353553);
    }
    #[test]
    fn test_input_part2() {
        assert_eq!(part2(TEST_INPUT), 167409079868000);
    }
    #[test]
    fn test_puzzle_input_part2() {
        assert_eq!(part2(PUZZLE_INPUT), 124615747767410);
    }
}
