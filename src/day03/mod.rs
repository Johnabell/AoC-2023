pub const PUZZLE_INPUT: &str = include_str!("input.txt");
static EMPTY_SYM: Vec<Sym> = Vec::new();
static EMPTY_NUM: Vec<Number> = Vec::new();

pub fn part1(input: &str) -> u32 {
    let mut board = Board::parse(input);
    board.mark_numbers();
    board.sum_marked_numbers()
}

pub fn part2(input: &str) -> u32 {
    Board::parse(input).gear_total()
}

#[derive(Debug, Default)]
struct Board {
    numbers: Vec<Vec<Number>>,
    symbols: Vec<Vec<Sym>>,
}

impl Board {
    fn parse(input: &str) -> Self {
        let mut board = Self::default();
        input.split('\n').for_each(|line| {
            let (numbers, syms) = Self::parse_line(line);
            board.numbers.push(numbers);
            board.symbols.push(syms);
        });

        board
    }

    fn parse_line(input: &str) -> (Vec<Number>, Vec<Sym>) {
        let mut numbers = Vec::new();
        let mut symbols = Vec::new();
        let chars = input.chars().enumerate();
        let mut current_number: Option<Number> = None;
        for (index, char) in chars {
            if char.is_ascii_digit() {
                if let Some(ref mut num) = current_number {
                    num.consume_char(char);
                } else {
                    current_number = Some(Number {
                        start_index: index,
                        len: 1,
                        value: char.to_digit(10).unwrap(),
                        marked: false,
                    });
                }
            } else {
                if let Some(num) = current_number {
                    numbers.push(num);
                    current_number = None;
                }
                if char != '.' {
                    symbols.push(Sym::new(index, char));
                }
            }
        }
        if let Some(num) = current_number {
            numbers.push(num);
        }

        (numbers, symbols)
    }

    fn mark_numbers(&mut self) {
        for line in 0..self.numbers.len() {
            let syms = self.symbols_for_line(line);
            for number in self.numbers[line].iter_mut() {
                number.maybe_mark_vec(syms.iter());
            }
        }
    }

    fn gear_total(&self) -> u32 {
        self.symbols
            .iter()
            .enumerate()
            .flat_map(|(line, syms)| {
                let numbers = self.numbers_for_line(line);
                syms.iter().map(move |sym| sym.gear_ratio(numbers.iter()))
            })
            .sum()
    }

    fn symbols_for_line(&self, line_number: usize) -> Vec<Sym> {
        self.get_syms(line_number)
            .chain(self.get_syms(line_number.saturating_sub(1)))
            .chain(self.get_syms(line_number + 1))
            .copied()
            .collect()
    }

    fn numbers_for_line(&self, line_number: usize) -> Vec<Number> {
        self.get_numbers(line_number)
            .chain(self.get_numbers(line_number.saturating_sub(1)))
            .chain(self.get_numbers(line_number + 1))
            .copied()
            .collect()
    }

    fn get_syms(&self, line_number: usize) -> impl Iterator<Item = &Sym> {
        if line_number >= self.symbols.len() {
            EMPTY_SYM.iter()
        } else {
            self.symbols[line_number].iter()
        }
    }

    fn get_numbers(&self, line_number: usize) -> impl Iterator<Item = &Number> {
        if line_number >= self.symbols.len() {
            EMPTY_NUM.iter()
        } else {
            self.numbers[line_number].iter()
        }
    }

    fn sum_marked_numbers(&self) -> u32 {
        self.numbers
            .iter()
            .flat_map(|line| line.iter().filter(|num| num.marked).map(|num| num.value))
            .sum()
    }
}

#[derive(Debug, Clone, Copy)]
struct Number {
    start_index: usize,
    len: usize,
    value: u32,
    marked: bool,
}

impl Number {
    fn consume_char(&mut self, c: char) {
        self.len += 1;
        self.value = self.value * 10 + c.to_digit(10).unwrap()
    }

    fn maybe_mark(&mut self, symbol: Sym) {
        self.marked |= self.is_adjacent_to(symbol)
    }

    fn is_adjacent_to(&self, symbol: Sym) -> bool {
        self.start_index.saturating_sub(1) <= symbol.index
            && symbol.index <= self.start_index + self.len
    }

    fn maybe_mark_vec<'a>(&mut self, symbols: impl Iterator<Item = &'a Sym>) {
        symbols.for_each(|sym| {
            self.maybe_mark(*sym);
        });
    }
}

#[derive(Debug, Clone, Copy)]
struct Sym {
    value: char,
    index: usize,
}

impl Sym {
    fn new(index: usize, value: char) -> Self {
        Self { index, value }
    }

    fn gear_ratio<'a>(&self, numbers: impl Iterator<Item = &'a Number>) -> u32 {
        if self.value == '*' {
            let adjacent_numbers: Vec<_> = numbers
                .filter(|number| number.is_adjacent_to(*self))
                .collect();
            if adjacent_numbers.len() == 2 {
                return adjacent_numbers.first().unwrap().value
                    * adjacent_numbers.last().unwrap().value;
            }
        }
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 4361);
    }

    #[test]
    fn puzzle_input_part1() {
        assert_eq!(part1(PUZZLE_INPUT), 539590);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 467835);
    }

    #[test]
    fn puzzle_input_part2() {
        assert_eq!(part2(PUZZLE_INPUT), 80703636);
    }
}
