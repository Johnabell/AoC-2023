use std::ops::{AddAssign, BitAnd, BitOr, BitOrAssign, Shl, Shr, Sub};

pub const PUZZLE_INPUT: &str = include_str!("input.txt");

pub fn part1<T: Number>(input: &str, steps: usize) -> usize {
    Garden::<T>::parse(input).plots_part1(steps)
}

pub fn part2<T: Number>(input: &str, steps: usize) -> usize {
    Garden::<T>::parse(input).plots_part2(steps)
}

struct Garden<T> {
    rows: Vec<T>,
    mask: T,
    starting_position: (usize, usize),
}

impl<T> Garden<T>
where
    T: Number,
{
    fn parse(input: &str) -> Self {
        let mut rows = vec![];
        let mut starting_position = None;
        for (row_index, line) in input.split('\n').enumerate() {
            let mut row = T::zero();
            for (i, c) in line.chars().enumerate() {
                let (val, includes_s) = match c {
                    '.' => (T::one(), false),
                    '#' => (T::zero(), false),
                    'S' => (T::one(), true),
                    _ => panic!("Unexpected symbol in input"),
                };
                row += val << i;
                if includes_s {
                    starting_position = Some((row_index, i));
                }
            }
            rows.push(row);
        }
        let mask = (T::one() << (rows.len())) - T::one();

        Self {
            rows,
            starting_position: starting_position.unwrap(),
            mask,
        }
    }

    fn plots_part1(&self, steps: usize) -> usize {
        self.plots(steps, self.starting_position)
            .total_plots_reached
    }

    fn plots_part2(&self, steps: usize) -> usize {
        let result = self.plots(steps, self.starting_position);
        dbg!(&result);
        let mut plots = result.total_plots_reached;
        if let Some(top_exit) = result.top {
            let result = self.plots(
                steps - top_exit.step - 1,
                (self.rows.len() - 1, top_exit.location),
            );
            plots += result.total_plots_reached;
        }
        if let Some(bottom_exit) = result.bottom {
            let result = self.plots(
                dbg!(steps - bottom_exit.step - 1),
                (0, bottom_exit.location),
            );
            dbg!(&result);
            plots += result.total_plots_reached;
        }
        if let Some(left_exit) = result.left {
            let result = self.plots(
                steps - left_exit.step - 1,
                (left_exit.location, self.mask.count_ones() as usize - 1),
            );
            plots += result.total_plots_reached;
        }
        if let Some(right_exit) = result.right {
            let result = self.plots(steps - right_exit.step - 1, (right_exit.location, 0));
            plots += result.total_plots_reached;
        }

        plots
    }

    fn plots(&self, steps: usize, (row, column): (usize, usize)) -> PlotResult {
        let mut result = PlotResult::default();
        let mut current_rows: Vec<_> = (0..self.rows.len())
            .map(|i| {
                if i == row {
                    T::one() << column
                } else {
                    T::zero()
                }
            })
            .collect();
        let mut next_rows = current_rows.to_owned();
        for step in 0..steps {
            next_rows.iter_mut().for_each(|val| *val = T::zero());
            current_rows
                .iter()
                .copied()
                .enumerate()
                .for_each(|(index, val)| {
                    if index > 0 {
                        next_rows[index - 1] |= val & self.rows[index - 1];
                    } else if val > T::zero() {
                        result.set_top_exit(val, step);
                    }
                    if val & T::one() == T::one() {
                        result.set_left_exit(index, step);
                    }
                    if (val << 1) & self.mask < val {
                        result.set_right_exit(index, step);
                    }
                    next_rows[index] |= self.next(val) & self.rows[index];
                    if index < next_rows.len() - 1 {
                        next_rows[index + 1] |= val & self.rows[index + 1];
                    } else if val > T::zero() {
                        result.set_bottom_exit(val, step);
                    }
                });
            std::mem::swap(&mut current_rows, &mut next_rows);
            //dbg!(current_rows.iter().copied().map(T::count_ones).sum::<u32>());
        }
        result.set_plot_reached(current_rows.into_iter().map(T::count_ones).sum::<u32>() as usize);
        result
    }

    fn next(&self, current: T) -> T {
        ((current << 1) | (current >> 1)) & self.mask
    }
}

#[derive(Default, Debug)]
struct PlotResult {
    top: Option<Exit>,
    bottom: Option<Exit>,
    left: Option<Exit>,
    right: Option<Exit>,
    total_plots_reached: usize,
}

impl PlotResult {
    fn set_top_exit<T: Number>(&mut self, val: T, step: usize) {
        if self.top.is_none() {
            assert!(val.count_ones() == 1);
            self.top = Some(Exit {
                location: val.trailing_zeros() as usize,
                step,
            });
        }
    }
    fn set_bottom_exit<T: Number>(&mut self, val: T, step: usize) {
        if self.bottom.is_none() {
            assert!(val.count_ones() == 1);
            self.bottom = Some(Exit {
                location: val.trailing_zeros() as usize,
                step,
            });
        }
    }
    fn set_left_exit(&mut self, index: usize, step: usize) {
        if self.left.is_none() {
            self.left = Some(Exit {
                location: index,
                step,
            });
        } else {
            assert!(self.left.as_ref().unwrap().step < step);
        }
    }
    fn set_right_exit(&mut self, index: usize, step: usize) {
        if self.right.is_none() {
            self.right = Some(Exit {
                location: index,
                step,
            });
        } else {
            assert!(self.right.as_ref().unwrap().step < step);
        }
    }
    fn set_plot_reached(&mut self, plots: usize) {
        self.total_plots_reached = plots;
    }
}

#[derive(Debug)]
struct Exit {
    location: usize,
    step: usize,
}

pub trait Number:
    Sized
    + Copy
    + Clone
    + BitAnd<Output = Self>
    + BitOrAssign<Self>
    + BitOr<Output = Self>
    + PartialOrd
    + AddAssign<Self>
    + std::fmt::Debug
    + Shl<usize, Output = Self>
    + Shr<usize, Output = Self>
    + Sub<Output = Self>
{
    fn one() -> Self;
    fn zero() -> Self;
    fn count_ones(self) -> u32;
    fn trailing_zeros(self) -> u32;
}

impl Number for u16 {
    fn one() -> Self {
        1
    }

    fn zero() -> Self {
        0
    }

    fn count_ones(self) -> u32 {
        u16::count_ones(self)
    }

    fn trailing_zeros(self) -> u32 {
        u16::trailing_zeros(self)
    }
}

#[cfg(test)]
mod test {
    use ethnum::U256;
    impl Number for U256 {
        fn one() -> Self {
            U256::ONE
        }

        fn zero() -> Self {
            U256::ZERO
        }

        fn count_ones(self) -> u32 {
            U256::count_ones(self)
        }

        fn trailing_zeros(self) -> u32 {
            U256::trailing_zeros(self)
        }
    }

    use super::*;

    const TEST_INPUT: &str = r#"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."#;

    #[test]
    fn test_input_part1() {
        assert_eq!(part1::<u16>(TEST_INPUT, 6), 16);
    }
    #[test]
    fn test_puzzle_input_part1() {
        assert_eq!(part1::<U256>(PUZZLE_INPUT, 64), 3632);
    }
    #[test]
    fn test_input_part2() {
        assert_eq!(part2::<u16>(TEST_INPUT, 10), 50);
        assert_eq!(part2::<u16>(TEST_INPUT, 50), 1594);
        assert_eq!(part2::<u16>(TEST_INPUT, 100), 6536);
        assert_eq!(part2::<u16>(TEST_INPUT, 500), 167004);
        assert_eq!(part2::<u16>(TEST_INPUT, 1000), 668697);
        assert_eq!(part2::<u16>(TEST_INPUT, 5000), 16733044);
    }
    #[test]
    fn test_puzzle_input_part2() {
        // assert_eq!(part2::<U256>(PUZZLE_INPUT, 26501365), 0);
    }
}
