pub const PUZZLE_INPUT: &str = include_str!("input.txt");

pub fn calculate<const EXPANSION_FACTOR: usize>(input: &str) -> usize {
    StartMap::<EXPANSION_FACTOR>::parse(input).total_length()
}

#[derive(Debug)]
struct StartMap<const N: usize = 1> {
    galaxies: Vec<Position>,
}

impl<const EXPANSION_FACTOR: usize> StartMap<EXPANSION_FACTOR> {
    fn parse(input: &str) -> Self {
        let mut galaxies: Vec<_> = input
            .split('\n')
            .enumerate()
            .flat_map(|(row_index, row)| {
                row.chars()
                    .enumerate()
                    .filter_map(move |(column_index, val)| {
                        if matches!(val, '#') {
                            Some(Position(row_index, column_index))
                        } else {
                            None
                        }
                    })
            })
            .collect();

        Self::expand_galaxies(&mut galaxies);

        Self { galaxies }
    }

    fn expand_galaxies(galaxies: &mut [Position]) {
        let (horizontal_galaxies, vertical_galaxies) = galaxies.iter().fold(
            (Vec::new(), Vec::new()),
            |(mut horizontal_galaxies, mut vertical_galaxies), Position(x, y)| {
                horizontal_galaxies.set(*x, true);
                vertical_galaxies.set(*y, true);
                (horizontal_galaxies, vertical_galaxies)
            },
        );
        let horizontal_scaling = Self::to_expansions(horizontal_galaxies);
        let vertical_scaling = Self::to_expansions(vertical_galaxies);
        galaxies.iter_mut().for_each(|pos| {
            pos.0 += horizontal_scaling[pos.0];
            pos.1 += vertical_scaling[pos.1];
        });
    }

    fn to_expansions(galaxy_indexes: Vec<bool>) -> Vec<usize> {
        let mut expansions = vec![0; galaxy_indexes.len()];
        let mut expansion = 0;
        for (index, contains_galaxy) in galaxy_indexes.iter().enumerate() {
            if !contains_galaxy {
                expansion += EXPANSION_FACTOR - 1;
            }
            expansions[index] = expansion;
        }
        expansions
    }

    fn total_length(&self) -> usize {
        self.galaxies
            .iter()
            .map(|pos| {
                self.galaxies
                    .iter()
                    .map(|pos2| pos.distance_from(pos2))
                    .sum::<usize>()
            })
            .sum::<usize>()
            / 2
    }
}

trait VecSet<T> {
    fn set(&mut self, index: usize, value: T);
}

impl<T> VecSet<T> for Vec<T>
where
    T: Default,
{
    fn set(&mut self, index: usize, value: T) {
        if self.len() <= index {
            self.resize_with(index + 1, Default::default);
        }
        self[index] = value;
    }
}

#[derive(Debug)]
struct Position(usize, usize);

impl Position {
    fn distance_from(&self, other: &Self) -> usize {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;

    #[test]
    fn test_input_part1() {
        assert_eq!(calculate::<2>(TEST_INPUT), 374);
    }
    #[test]
    fn test_puzzle_input_part1() {
        assert_eq!(calculate::<2>(PUZZLE_INPUT), 9947476);
    }
    #[test]
    fn test_input_part2() {
        assert_eq!(calculate::<10>(TEST_INPUT), 1030);
        assert_eq!(calculate::<100>(TEST_INPUT), 8410);
    }
    #[test]
    fn test_puzzle_input_part2() {
        assert_eq!(calculate::<1000000>(PUZZLE_INPUT), 519939907614);
    }
}
