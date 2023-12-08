use std::{
    collections::{hash_map, HashMap},
    marker::PhantomData,
    ops::AddAssign,
};

const CARDS: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];
const CARDS_JOKER: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

const JOKER_CARD: char = 'J';

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandValue {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialOrd, Eq, PartialEq)]
struct Hand<T>(String, PhantomData<T>);

#[derive(Debug, PartialEq, PartialOrd, Eq)]
enum NormalParty {}

#[derive(Debug, PartialEq, PartialOrd, Eq)]
enum JokerParty {}

impl Ord for Hand<NormalParty> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0
            .chars()
            .zip(other.0.chars())
            .find(|(a, b)| a != b)
            .map(|(a, b)| {
                CARDS
                    .iter()
                    .position(|v| v == &a)
                    .expect("Should be in dict")
                    .cmp(
                        &CARDS
                            .iter()
                            .position(|v| v == &b)
                            .expect("Should be in dict"),
                    )
            })
            .expect("Difference should happen")
    }
}
impl Ord for Hand<JokerParty> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0
            .chars()
            .zip(other.0.chars())
            .find(|(a, b)| a != b)
            .map(|(a, b)| {
                CARDS_JOKER
                    .iter()
                    .position(|v| v == &a)
                    .expect("Should be in dict")
                    .cmp(
                        &CARDS_JOKER
                            .iter()
                            .position(|v| v == &b)
                            .expect("Should be in dict"),
                    )
            })
            .expect("Difference should happen")
    }
}

fn eval_normal_hand(hand: Vec<(char, u8)>) -> HandValue {
    match hand[0].1 {
        5 => HandValue::FiveOfAKind,
        4 => HandValue::FourOfAKind,
        3 => match hand[1].1 {
            2 => HandValue::FullHouse,
            _ => HandValue::ThreeOfAKind,
        },
        2 => match hand[1].1 {
            2 => HandValue::TwoPairs,
            _ => HandValue::OnePair,
        },
        _ => HandValue::HighCard,
    }
}

fn eval_joker_hand(hand: Vec<(char, u8)>) -> HandValue {
    if let Some(joker_position) = hand.iter().position(|(c, _)| c == &JOKER_CARD) {
        let nbr_jokers = hand.get(joker_position).expect("Joker present").1;
        let mut non_joker_cards = hand.iter().filter(|(v, _)| v != &JOKER_CARD);
        let first_non_joker = non_joker_cards.next();
        let second_non_joker = non_joker_cards.next();
        // hand.iter().
        match first_non_joker {
            Some((_, count)) => match count + nbr_jokers {
                5 => HandValue::FiveOfAKind,
                4 => HandValue::FourOfAKind,
                3 => match second_non_joker {
                    Some((_, second_count)) => match second_count {
                        2 => HandValue::FullHouse,
                        _ => HandValue::ThreeOfAKind,
                    },
                    None => HandValue::ThreeOfAKind,
                },
                2 => match second_non_joker {
                    Some((_, second_count)) => match second_count {
                        2 => HandValue::TwoPairs,
                        _ => HandValue::OnePair,
                    },
                    None => HandValue::OnePair,
                },
                _ => HandValue::HighCard,
            },
            None => HandValue::FiveOfAKind,
        }
    } else {
        eval_normal_hand(hand)
    }
}

impl<T> Hand<T> {
    fn sort_chars(&self) -> Vec<(char, u8)> {
        let mut hash_map: HashMap<char, u8> = HashMap::new();
        self.0.chars().for_each(|c| {
            let v = hash_map.entry(c.clone());
            match v {
                hash_map::Entry::Occupied(mut occ) => {
                    occ.get_mut().add_assign(1);
                }
                hash_map::Entry::Vacant(v) => {
                    v.insert(1u8);
                }
            }
        });
        let mut tot = hash_map.into_iter().collect::<Vec<_>>();
        tot.sort_by(|(_, count_a), (_, count_b)| count_b.cmp(count_a));
        tot
    }
}

impl Hand<NormalParty> {
    fn eval_hand(&self) -> HandValue {
        let hand = self.sort_chars();
        eval_normal_hand(hand)
    }
}

impl Hand<JokerParty> {
    fn eval_hand(&self) -> HandValue {
        let hand = self.sort_chars();
        eval_joker_hand(hand.clone())
    }
}

#[derive(Debug)]
struct HandAndBid<T> {
    hand: Hand<T>,
    bid: u32,
}

impl<T> From<&str> for HandAndBid<T> {
    fn from(value: &str) -> Self {
        let mut hand_and_bid = value.split(' ');
        Self {
            hand: Hand(
                hand_and_bid.next().expect("Should have a hand").to_string(),
                PhantomData,
            ),
            bid: hand_and_bid
                .next()
                .expect("Should have a bid")
                .parse::<u32>()
                .expect("Should be a number"),
        }
    }
}

pub(crate) fn eval_file(file: &str) -> u32 {
    let mut hand_and_bids = file
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| line.into())
        .collect::<Vec<HandAndBid<NormalParty>>>();
    hand_and_bids.sort_by(|a, b| {
        let a_eval = a.hand.eval_hand();
        let b_eval = b.hand.eval_hand();
        if a_eval == b_eval {
            a.hand.cmp(&b.hand)
        } else {
            a_eval.cmp(&b_eval)
        }
    });
    hand_and_bids
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, h_b)| acc + h_b.bid * (idx as u32 + 1))
}

pub(crate) fn eval_file_2(file: &str) -> u32 {
    let mut hand_and_bids = file
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| line.into())
        .collect::<Vec<HandAndBid<JokerParty>>>();
    hand_and_bids.sort_by(|a, b| {
        let a_eval = a.hand.eval_hand();
        let b_eval = b.hand.eval_hand();
        if a_eval == b_eval {
            a.hand.cmp(&b.hand)
        } else {
            a_eval.cmp(&b_eval)
        }
    });
    hand_and_bids
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, h_b)| acc + h_b.bid * (idx as u32 + 1))
}
pub(crate) fn print_sol_1(file: &str) {
    print!("res : {}", eval_file(file));
}
pub(crate) fn print_sol_2(file: &str) {
    print!("res : {}", eval_file_2(file));
}

#[cfg(test)]
mod tests {
    use super::{eval_file, eval_file_2};

    fn data() -> &'static str {
        r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#
    }

    fn data_2() -> &'static str {
        r#"2345A 2
2345J 5
J345A 3
32T3K 7
T55J5 17
KK677 11
KTJJT 23
QQQJA 19
JJJJJ 29
JAAAA 37
AAAAJ 43
AAAAA 53
2AAAA 13
2JJJJ 41
JJJJ2 31"#
    }

    #[test]
    fn test_0() {
        assert_eq!(6440, eval_file(data()));
    }
    #[test]
    fn test_1() {
        assert_eq!(5905, eval_file_2(data()));
    }
    #[test]
    fn test_2() {
        // assert_eq!(3542, eval_file(data_2()));
        assert_eq!(3667, eval_file_2(data_2()));
    }
}
