pub const PUZZLE_INPUT: &str = include_str!("input.txt");

pub fn part1(input: &str) -> u32 {
    Game::<Part1Card>::parse(input).total_winnings()
}

pub fn part2(input: &str) -> u32 {
    Game::<Part2Card>::parse(input).total_winnings()
}

#[derive(Debug)]
struct Game<T: GameCard> {
    hands: Vec<Hand<T>>,
}

impl<T: GameCard> Game<T> {
    fn parse(input: &str) -> Self {
        let hands = input.split('\n').map(Hand::parse).collect();

        Self { hands }
    }

    fn total_winnings(&mut self) -> u32 {
        self.hands.sort();
        self.hands.iter().enumerate().fold(0, |acc, (index, hand)| {
            acc + (hand.bid * (index as u32 + 1))
        })
    }
}

trait GameCard: PartialEq + Eq + PartialOrd + Ord + TypedHand {
    fn from_char(c: char) -> Self;
    fn as_usize(&self) -> usize;
}

trait TypedHand: Sized {
    fn hand_type(cards: &[Self; 5]) -> HandType;
}

macro_rules! impl_from_char {
    ($type:ty, $j_name:ident) => {
        impl GameCard for $type {
            fn from_char(s: char) -> Self {
                match s {
                    'A' => Self::Ace,
                    'K' => Self::King,
                    'Q' => Self::Queen,
                    'J' => Self::$j_name,
                    'T' => Self::Ten,
                    '9' => Self::Nine,
                    '8' => Self::Eight,
                    '7' => Self::Seven,
                    '6' => Self::Six,
                    '5' => Self::Five,
                    '4' => Self::Four,
                    '3' => Self::Three,
                    '2' => Self::Two,
                    _ => panic!("Unrecognized card"),
                }
            }
            fn as_usize(&self) -> usize {
                *self as usize
            }
        }
    };
}

impl_from_char!(Part1Card, Jack);
impl_from_char!(Part2Card, Joker);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[repr(u8)]
enum Part1Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[repr(u8)]
enum Part2Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl From<&[u32]> for HandType {
    fn from(counts: &[u32]) -> Self {
        let mut hand_type = HandType::HighCard;

        for count in counts {
            match count {
                5 => return HandType::FiveOfAKind,
                4 => return HandType::FourOfAKind,
                3 => match hand_type {
                    HandType::OnePair => return HandType::FullHouse,
                    _ => {
                        hand_type = HandType::ThreeOfAKind;
                    }
                },
                2 => match hand_type {
                    HandType::ThreeOfAKind => return HandType::FullHouse,
                    HandType::OnePair => return HandType::TwoPair,
                    _ => {
                        hand_type = HandType::OnePair;
                    }
                },
                _ => continue,
            }
        }
        hand_type
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand<T: GameCard> {
    cards: [T; 5],
    bid: u32,
}

impl<T: GameCard> Hand<T> {
    fn parse(input: &str) -> Self {
        let (cards, bid) = input.split_once(' ').unwrap();
        let mut chars = cards.chars();
        let cards = [
            chars.next().map(T::from_char).unwrap(),
            chars.next().map(T::from_char).unwrap(),
            chars.next().map(T::from_char).unwrap(),
            chars.next().map(T::from_char).unwrap(),
            chars.next().map(T::from_char).unwrap(),
        ];
        Self {
            cards,
            bid: bid.parse().unwrap(),
        }
    }
}

impl TypedHand for Part1Card {
    fn hand_type(cards: &[Self; 5]) -> HandType {
        let mut count = [0; 13];
        cards.iter().for_each(|card| count[card.as_usize()] += 1);
        count[..].into()
    }
}

impl TypedHand for Part2Card {
    fn hand_type(cards: &[Self; 5]) -> HandType {
        let mut count = [0; 15];
        cards.iter().for_each(|card| count[card.as_usize()] += 1);
        let joker_count = count[0];
        let hand_type = count[1..].into();

        match (joker_count, hand_type) {
            (5 | 4, _) => HandType::FiveOfAKind,
            (3, HandType::OnePair) => HandType::FiveOfAKind,
            (3, _) => HandType::FourOfAKind,
            (2, HandType::ThreeOfAKind) => HandType::FiveOfAKind,
            (2, HandType::OnePair) => HandType::FourOfAKind,
            (2, _) => HandType::ThreeOfAKind,
            (1, HandType::FourOfAKind) => HandType::FiveOfAKind,
            (1, HandType::ThreeOfAKind) => HandType::FourOfAKind,
            (1, HandType::TwoPair) => HandType::FullHouse,
            (1, HandType::OnePair) => HandType::ThreeOfAKind,
            (1, _) => HandType::OnePair,
            (_, hand_type) => hand_type,
        }
    }
}

impl<T: GameCard> PartialOrd for Hand<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: GameCard> Ord for Hand<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match T::hand_type(&self.cards).cmp(&T::hand_type(&other.cards)) {
            std::cmp::Ordering::Equal => self.cards.cmp(&other.cards),
            cmp => cmp,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

    #[test]
    fn test_input_part_1() {
        assert_eq!(part1(TEST_INPUT), 6440);
    }

    #[test]
    fn test_puzzle_input_part_1() {
        assert_eq!(part1(PUZZLE_INPUT), 246795406);
    }

    #[test]
    fn test_input_part_2() {
        assert_eq!(part2(TEST_INPUT), 5905);
    }

    #[test]
    fn test_puzzle_input_part_2() {
        assert_eq!(part2(PUZZLE_INPUT), 249356515);
    }
}
