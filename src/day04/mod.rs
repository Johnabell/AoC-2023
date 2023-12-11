pub const PUZZLE_INPUT: &str = include_str!("input.txt");

pub fn part1(input: &str) -> u32 {
    input
        .split('\n')
        .map(Card::parse)
        .map(|card| card.score())
        .sum()
}

pub fn part2(input: &str) -> u32 {
    ScratchCards::parse(input).score()
}

struct ScratchCards {
    cards: Vec<Card>,
}

impl ScratchCards {
    fn parse(input: &str) -> Self {
        let cards = input.split('\n').map(Card::parse).collect();
        Self { cards }
    }

    fn score(&self) -> u32 {
        let mut number_of_cards = vec![1_u32; self.cards.len()];
        self.cards.iter().enumerate().for_each(|(index, card)| {
            let score = card.number_of_wins();
            let count_of_this_card = number_of_cards[index];

            for i in 1..=score {
                if let Some(num) = number_of_cards.get_mut(index + i) {
                    *num += count_of_this_card;
                }
            }
        });

        number_of_cards.iter().sum()
    }
}

struct Card {
    winning_numbers: Vec<u32>,
    actual_numbers: Vec<u32>,
}

impl Card {
    fn parse(input: &str) -> Self {
        let (_, numbers) = input.split_once(": ").expect("No colon in line");
        let (winning_numbers, actual_numbers) = numbers
            .split_once(" | ")
            .expect("No pipe separating numbers");

        Self {
            winning_numbers: Self::parse_numbers(winning_numbers),
            actual_numbers: Self::parse_numbers(actual_numbers),
        }
    }

    fn parse_numbers(input: &str) -> Vec<u32> {
        input
            .split_whitespace()
            .map(|i| i.parse().expect("Couldn't parse number"))
            .collect()
    }

    fn score(&self) -> u32 {
        1 << self.number_of_wins() >> 1
    }

    fn number_of_wins(&self) -> usize {
        self.actual_numbers
            .iter()
            .filter(|num| self.winning_numbers.contains(num))
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

    #[test]
    fn test_input_part1() {
        assert_eq!(part1(TEST_INPUT), 13);
    }

    #[test]
    fn test_puzzle_input_part1() {
        assert_eq!(part1(PUZZLE_INPUT), 21158);
    }

    #[test]
    fn test_input_part2() {
        assert_eq!(part2(TEST_INPUT), 30);
    }

    #[test]
    fn test_puzzle_input_part2() {
        assert_eq!(part2(PUZZLE_INPUT), 6050769);
    }
}
