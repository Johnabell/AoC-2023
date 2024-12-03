pub const PUZZLE_INPUT: &str = include_str!("input.txt");

pub fn part1(input: &str) -> usize {
    Map::parse(input).summary::<Part1>()
}

pub fn part2(input: &str) -> usize {
    Map::parse(input).summary::<Part2>()
}

struct Map {
    sections: Vec<MapSection>,
}

impl Map {
    fn parse(input: &str) -> Self {
        let sections = input.split("\n\n").map(MapSection::parse).collect();
        Self { sections }
    }

    fn summary<T: ReflectionTest>(&self) -> usize {
        self.sections
            .iter()
            .map(|section| {
                section
                    .row_reflection::<T>()
                    .map(|r| r * 100)
                    .or_else(|| section.column_reflection::<T>())
                    .expect("No reflection found")
            })
            .sum()
    }
}

struct MapSection {
    rows: Vec<usize>,
    columns: Vec<usize>,
}

impl MapSection {
    fn parse(input: &str) -> Self {
        let mut columns = vec![];
        let mut rows = vec![];
        for (index, line) in input.split('\n').enumerate() {
            if index == 0 {
                columns.resize(line.len(), 0);
            }
            let mut row = 0;
            for (i, c) in line.chars().enumerate() {
                let val = match c {
                    '.' => 0,
                    '#' => 1,
                    _ => panic!("Unexpected symbol in input"),
                };
                row += val << i;
                columns[i] += val << index;
            }
            rows.push(row);
        }
        Self { rows, columns }
    }

    fn row_reflection<T: ReflectionTest>(&self) -> Option<usize> {
        Self::reflection::<T>(&self.rows)
    }
    fn column_reflection<T: ReflectionTest>(&self) -> Option<usize> {
        Self::reflection::<T>(&self.columns)
    }
    fn reflection<T: ReflectionTest>(list: &[usize]) -> Option<usize> {
        list.windows(2)
            .enumerate()
            .filter(|(_index, window)| {
                let val = window[0] ^ window[1];
                val.is_power_of_two() || val == 0
            })
            .map(|(index, _)| index)
            .find(|index| T::is_reflection(list, *index))
            .map(|i| i + 1)
    }
}

trait ReflectionTest {
    fn is_reflection(list: &[usize], reflection_point: usize) -> bool;
}

struct Part1;
struct Part2;

impl ReflectionTest for Part1 {
    fn is_reflection(list: &[usize], reflection_point: usize) -> bool {
        let mut i = 0;
        loop {
            let left = reflection_point - i;
            let right = reflection_point + 1 + i;
            if list[left] != list[right] {
                return false;
            }
            if left == 0 || right == list.len() - 1 {
                break;
            }
            i += 1;
        }
        true
    }
}

impl ReflectionTest for Part2 {
    fn is_reflection(list: &[usize], reflection_point: usize) -> bool {
        let mut i = 0;
        let mut defect_repairs_used = false;
        loop {
            let left = reflection_point - i;
            let right = reflection_point + 1 + i;
            let cmp = list[left] ^ list[right];
            if cmp != 0 {
                if !defect_repairs_used && cmp.is_power_of_two() {
                    defect_repairs_used = true;
                } else {
                    return false;
                }
            }
            if left == 0 || right == list.len() - 1 {
                break;
            }
            i += 1;
        }
        defect_repairs_used
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;

    #[test]
    fn test_input_part1() {
        assert_eq!(part1(TEST_INPUT), 405);
    }
    #[test]
    fn test_puzzle_input_part1() {
        assert_eq!(part1(PUZZLE_INPUT), 34889);
    }
    #[test]
    fn test_input_part2() {
        assert_eq!(part2(TEST_INPUT), 400);
    }
    #[test]
    fn test_puzzle_input_part2() {
        assert_eq!(part2(PUZZLE_INPUT), 34224);
    }
}
