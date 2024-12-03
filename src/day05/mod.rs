use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::Range;
use std::str::FromStr;

use crate::two_iter::TwoIter;

pub const PUZZLE_INPUT: &str = include_str!("input.txt");

pub fn part1(input: &str) -> i64 {
    Pipeline::parse(input).run_part1()
}

pub fn part2(input: &str) -> i64 {
    Pipeline::parse(input).run_part2()
}

#[derive(Debug)]
struct Pipeline {
    part1_seeds: Vec<i64>,
    part2_seeds: Vec<Range<i64>>,
    maps: HashMap<Type, Map>,
}

impl Pipeline {
    fn parse(input: &str) -> Self {
        let (seeds, maps) = input.split_once("\n\n").expect("No seeds");
        let seeds = seeds
            .strip_prefix("seeds: ")
            .unwrap()
            .split(' ')
            .map(FromStr::from_str)
            .collect::<Result<Vec<i64>, _>>()
            .unwrap();

        Self {
            part1_seeds: seeds.clone(),
            part2_seeds: seeds[..]
                .chunks(2)
                .map(|range| (range[0])..(range[0] + range[1]))
                .collect(),
            maps: maps
                .split("\n\n")
                .map(Map::parse)
                .fold(HashMap::new(), |mut hash_map, map| {
                    hash_map.insert(map.input, map);
                    hash_map
                }),
        }
    }

    fn run_part1(&self) -> i64 {
        self.part1_seeds
            .iter()
            .map(|seed| self.map_seed(*seed))
            .min()
            .unwrap()
    }

    fn map_seed(&self, seed: i64) -> i64 {
        let mut value = seed;
        let mut current_type = Type::Seed;
        while current_type != Type::Location {
            value = self.maps[&current_type].map(value);
            current_type = self.maps[&current_type].output;
        }
        value
    }

    fn run_part2(&self) -> i64 {
        self.map_seed_part2(self.part2_seeds.clone())
            .into_iter()
            .map(|range| range.start)
            .min()
            .unwrap()
    }

    fn map_seed_part2(&self, seed: Vec<Range<i64>>) -> Vec<Range<i64>> {
        let mut ranges = seed;
        let mut current_type = Type::Seed;
        while current_type != Type::Location {
            ranges = ranges
                .into_iter()
                .flat_map(|value| self.maps[&current_type].map_range(value))
                .collect();
            current_type = self.maps[&current_type].output;
        }
        ranges
    }
}

#[derive(Debug)]
struct Map {
    input: Type,
    output: Type,
    mappings: Vec<Mapping>,
}

impl Map {
    fn parse(input: &str) -> Self {
        let (types, mappings) = input.split_once('\n').expect("Unexpected map");
        let (input, rest) = types.split_once('-').expect("No from mapping");
        let (_, rest) = rest.split_once('-').expect("No to mapping");
        let (output, _) = rest.split_once(' ').expect("No to mapping");

        Self {
            input: input.parse().unwrap(),
            output: output.parse().unwrap(),
            mappings: mappings.split('\n').map(Mapping::parse).collect(),
        }
    }

    fn map(&self, value: i64) -> i64 {
        for mapping in &self.mappings {
            match mapping.map(value) {
                Some(value) => return value,
                _ => continue,
            }
        }
        value
    }

    fn map_range(&self, initial_value: Range<i64>) -> Vec<Range<i64>> {
        let mut mapped = Vec::new();
        let mut unmapped = vec![initial_value];
        for mapping in &self.mappings {
            unmapped = unmapped
                .into_iter()
                .flat_map(|value| match mapping.map_range(value) {
                    (Some(value), rest) => {
                        mapped.push(value);
                        rest
                    }
                    (None, rest) => rest,
                })
                .collect();
        }
        mapped.extend(unmapped);

        mapped
    }
}

#[derive(Debug)]
struct Mapping {
    input_range: Range<i64>,
    diff: i64,
}

impl Mapping {
    fn parse(input: &str) -> Self {
        let mut values = input.split(' ').map(FromStr::from_str).map(Result::unwrap);
        let to = values.next().unwrap();
        let from = values.next().unwrap();
        let range = values.next().unwrap();

        Self {
            input_range: from..(from + range),
            diff: to - from,
        }
    }

    fn map(&self, value: i64) -> Option<i64> {
        if self.input_range.contains(&value) {
            Some(value + self.diff)
        } else {
            None
        }
    }

    fn map_range(&self, value: Range<i64>) -> (Option<Range<i64>>, Remainder) {
        match (
            self.input_range.start.cmp(&value.end),
            self.input_range.end.cmp(&value.start),
        ) {
            (Ordering::Greater | Ordering::Equal, _) => (None, Remainder::One(value)),
            (_, Ordering::Less | Ordering::Equal) => (None, Remainder::One(value)),
            _ => {
                match (
                    self.input_range.start.cmp(&value.start),
                    self.input_range.end.cmp(&value.end),
                ) {
                    (Ordering::Greater, Ordering::Greater | Ordering::Equal) => (
                        Some((self.input_range.start + self.diff)..(value.end + self.diff)),
                        Remainder::One((value.start)..(self.input_range.start)),
                    ),
                    (Ordering::Greater, Ordering::Less) => (
                        Some(
                            (self.input_range.start + self.diff)
                                ..(self.input_range.end + self.diff),
                        ),
                        Remainder::Two(
                            (value.start)..(self.input_range.start),
                            (self.input_range.end)..(value.end),
                        ),
                    ),
                    (Ordering::Less | Ordering::Equal, Ordering::Less) => (
                        Some((value.start + self.diff)..(self.input_range.end + self.diff)),
                        Remainder::One((self.input_range.end)..(value.end)),
                    ),
                    (Ordering::Less | Ordering::Equal, Ordering::Greater | Ordering::Equal) => (
                        Some((value.start + self.diff)..(value.end + self.diff)),
                        Remainder::Zero,
                    ),
                }
            }
        }
    }
}

type Remainder = TwoIter<Range<i64>>;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Type {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl FromStr for Type {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "seed" => Ok(Self::Seed),
            "soil" => Ok(Self::Soil),
            "fertilizer" => Ok(Self::Fertilizer),
            "water" => Ok(Self::Water),
            "light" => Ok(Self::Light),
            "temperature" => Ok(Self::Temperature),
            "humidity" => Ok(Self::Humidity),
            "location" => Ok(Self::Location),
            _ => Err("Unrecognised type"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

    #[test]
    fn test_input_part1() {
        assert_eq!(part1(TEST_INPUT), 35);
    }
    #[test]
    fn test_puzzle_input_part1() {
        assert_eq!(part1(PUZZLE_INPUT), 196167384);
    }
    #[test]
    fn test_input_part2() {
        assert_eq!(part2(TEST_INPUT), 46);
    }
    #[test]
    fn test_puzzle_input_part2() {
        assert_eq!(part2(PUZZLE_INPUT), 125742456);
    }
}
