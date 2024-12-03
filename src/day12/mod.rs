use std::str::FromStr;

pub const PUZZLE_INPUT: &str = include_str!("input.txt");

pub fn part1(input: &str) -> usize {
    SpringRecord::parse(input).possible_configurations()
}

pub fn part2(input: &str) -> usize {
    todo!();
}

#[derive(Debug)]
struct SpringRecord {
    records: Vec<Record>,
}

impl SpringRecord {
    fn parse(input: &str) -> Self {
        let records = input.split('\n').map(Record::parse).collect();
        Self { records }
    }

    fn possible_configurations(&self) -> usize {
        self.records.iter().map(Record::all_possible_matches).sum()
    }
}

#[derive(Debug)]
struct Record {
    springs: Vec<SpringStatus>,
    damaged_groups: Vec<usize>,
}

impl Record {
    fn parse(input: &str) -> Self {
        let (springs, groups) = input.split_once(' ').unwrap();

        Self {
            springs: springs.chars().map(SpringStatus::from_char).collect(),
            damaged_groups: groups
                .split(',')
                .map(FromStr::from_str)
                .map(Result::unwrap)
                .collect(),
        }
    }

    fn all_possible_matches(&self) -> usize {
        let mut option = self.springs.clone();
        let unknown_indexes = self.unknown_indexes();

        (0..(1 << unknown_indexes.len()))
            .filter(|combination| {
                unknown_indexes
                    .iter()
                    .enumerate()
                    .for_each(|(offset, index)| {
                        option[*index] =
                            SpringStatus::from_bit(((1 << offset) & combination) >> offset)
                    });
                Self::matches(&option, &self.damaged_groups)
            })
            .count()
    }
    fn possible_matches(&self) -> usize {
        for range in self.broken_unknown_ranges() {
            for i in 0..self.damaged_groups.len() {}
        }

        0
    }
    fn matches_for_broken_unknown_range(
        springs: &[SpringStatus],
        damaged_groups: &[usize],
    ) -> Option<usize> {
        None
    }
    fn broken_unknown_ranges(&self) -> impl Iterator<Item = &[SpringStatus]> {
        self.springs
            .split(|status| status.is_operational())
            .filter(|slice| slice.is_empty())
    }
    fn next_broken_range(springs: &[SpringStatus]) -> &[SpringStatus] {
        let mut skipped = 0;
        let len = springs
            .iter()
            .skip_while(|status| {
                skipped += 1;
                status.is_operational()
            })
            .take_while(|status| status.not_operational())
            .count();
        &springs[skipped..(skipped + len)]
    }

    fn unknown_indexes(&self) -> Vec<usize> {
        self.springs
            .iter()
            .enumerate()
            .filter_map(|(index, status)| match status {
                SpringStatus::Unknown => Some(index),
                _ => None,
            })
            .collect()
    }

    fn matches(springs: &[SpringStatus], damaged_groups: &[usize]) -> bool {
        let mut expected_damaged_groups = damaged_groups.iter();
        let mut current_damage_group = None;
        for spring in springs {
            match spring {
                SpringStatus::Operational => {
                    if current_damage_group.is_some()
                        && current_damage_group.as_ref() != expected_damaged_groups.next()
                    {
                        return false;
                    }
                    current_damage_group = None;
                }
                SpringStatus::Damaged => {
                    current_damage_group = Some(current_damage_group.unwrap_or(0) + 1)
                }
                SpringStatus::Unknown => panic!("Shouldn't call matches with unknown springs"),
            }
        }
        expected_damaged_groups.next() == current_damage_group.as_ref()
            && expected_damaged_groups.next().is_none()
    }
}

fn to_unsigned(springs: &[SpringStatus]) -> usize {
    springs.iter().enumerate().fold(0, |acc, (index, status)| {
        acc + ((*status as usize) << index)
    })
}

#[derive(Debug, Clone, Copy)]
enum SpringStatus {
    Unknown = 0,
    Damaged = 1,
    Operational,
}

impl SpringStatus {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            _ => Self::Unknown,
        }
    }

    fn from_bit(bit: usize) -> Self {
        match bit {
            0 => Self::Operational,
            1 => Self::Damaged,
            _ => panic!("Should only be called with 0 and 1"),
        }
    }
    fn not_operational(&self) -> bool {
        !matches!(self, Self::Operational)
    }
    fn is_operational(&self) -> bool {
        matches!(self, Self::Operational)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;

    #[test]
    fn test_input_part1() {
        assert_eq!(part1(TEST_INPUT), 21);
    }
    #[test]
    fn test_puzzle_input_part1() {
        assert_eq!(part1(PUZZLE_INPUT), 7047);
    }
    #[test]
    fn test_input_part2() {
        assert_eq!(part2(TEST_INPUT), 525152);
    }
    // #[test]
    // fn test_puzzle_input_part2() {
    //     assert_eq!(part2(PUZZLE_INPUT), 0);
    // }
}
