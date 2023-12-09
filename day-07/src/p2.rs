use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::Hash;
use std::io::BufRead;
use std::{fs::File, io::BufReader};

#[derive(PartialEq, PartialOrd, Hash, Eq, Debug, Clone, Copy, Ord)]
struct Card(u32);

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
struct Hand(Card, Card, Card, Card, Card);

impl Hand {
    fn to_vec(self) -> Vec<Card> {
        [self.0, self.1, self.2, self.3, self.4].to_vec()
    }
}

impl From<(Card, Card, Card, Card, Card)> for Hand {
    fn from((c1, c2, c3, c4, c5): (Card, Card, Card, Card, Card)) -> Self {
        Hand(c1, c2, c3, c4, c5)
    }
}

#[derive(Eq, Debug, PartialEq, PartialOrd)]
enum Type {
    FiveOfAKind(Hand),
    FourOfAKind(Hand),
    FullHouse(Hand),
    ThreeOfAKind(Hand),
    TwoPair(Hand),
    OnePair(Hand),
    HighCard(Hand),
}

impl Ord for Type {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Type::FiveOfAKind(self_hand), Type::FiveOfAKind(other_hand)) => {
                self_hand.cmp(other_hand)
            }
            (Type::FiveOfAKind(_), _) => Ordering::Greater,
            (_, Type::FiveOfAKind(_)) => Ordering::Less,

            (Type::FourOfAKind(self_hand), Type::FourOfAKind(other_hand)) => {
                self_hand.cmp(other_hand)
            }
            (Type::FourOfAKind(_), _) => Ordering::Greater,
            (_, Type::FourOfAKind(_)) => Ordering::Less,

            (Type::FullHouse(self_hand), Type::FullHouse(other_hand)) => self_hand.cmp(other_hand),
            (Type::FullHouse(_), _) => Ordering::Greater,
            (_, Type::FullHouse(_)) => Ordering::Less,

            (Type::ThreeOfAKind(self_hand), Type::ThreeOfAKind(other_hand)) => {
                self_hand.cmp(other_hand)
            }
            (Type::ThreeOfAKind(_), _) => Ordering::Greater,
            (_, Type::ThreeOfAKind(_)) => Ordering::Less,

            (Type::TwoPair(self_hand), Type::TwoPair(other_hand)) => self_hand.cmp(other_hand),
            (Type::TwoPair(_), _) => Ordering::Greater,
            (_, Type::TwoPair(_)) => Ordering::Less,

            (Type::OnePair(self_hand), Type::OnePair(other_hand)) => self_hand.cmp(other_hand),
            (Type::OnePair(_), _) => Ordering::Greater,
            (_, Type::OnePair(_)) => Ordering::Less,

            (Type::HighCard(self_hand), Type::HighCard(other_hand)) => self_hand.cmp(other_hand),
        }
    }
}

impl From<Hand> for Type {
    fn from(hand: Hand) -> Self {
        let (nb_joker, grouped_cards) = hand.clone().to_vec().iter().fold(
            (0, HashMap::new() as HashMap<Card, u32>),
            |(nb_joker, mut cards), card| {
                if card == &Card(1) {
                    (nb_joker + 1, cards)
                } else {
                    let total = cards.entry(*card).or_insert(0);
                    *total += 1;
                    (nb_joker, cards)
                }
            },
        );

        let grouped_cards_with_joker = grouped_cards
            .into_iter()
            .map(|(key, value)| (key, value + nb_joker))
            .collect::<HashMap<_, _>>();

        let copy_for_debug = grouped_cards_with_joker.clone();

        let hand = if grouped_cards_with_joker.len() == 1 || nb_joker == 5 {
            Type::FiveOfAKind(hand)
        } else if grouped_cards_with_joker.len() == 2 {
            if grouped_cards_with_joker.into_values().max().unwrap() == 4 {
                Type::FourOfAKind(hand)
            } else {
                Type::FullHouse(hand)
            }
        } else if grouped_cards_with_joker.len() == 3 {
            if grouped_cards_with_joker.into_values().max().unwrap() == 3 {
                Type::ThreeOfAKind(hand)
            } else {
                Type::TwoPair(hand)
            }
        } else if grouped_cards_with_joker.len() == 4 {
            Type::OnePair(hand)
        } else if grouped_cards_with_joker.len() == 5 {
            Type::HighCard(hand)
        } else {
            unreachable!("{hand:?}");
        };

        println!("{copy_for_debug:?} -> {hand:?}");
        hand
    }
}

fn main() -> std::io::Result<()> {
    let mut hand_score = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            let (hand_str, score_str) = line.split_once(" ").unwrap();
            (
                Hand::from(
                    hand_str
                        .chars()
                        .map(|c| match c.to_digit(10) {
                            Some(value) => Card(value),
                            None => match c {
                                'A' => Card(14),
                                'K' => Card(13),
                                'Q' => Card(12),
                                'T' => Card(11),
                                'J' => Card(1),
                                _ => unreachable!(),
                            },
                        })
                        .collect_tuple::<(Card, Card, Card, Card, Card)>()
                        .unwrap(),
                ),
                score_str.parse::<u64>().unwrap(),
            )
        })
        .map(|(hand, score)| (Type::from(hand), score))
        .collect_vec();

    hand_score.sort_by(|(h, _), (o_h, _)| h.cmp(o_h));

    let score = hand_score
        .iter()
        .enumerate()
        // .inspect(|elem| println!("{:?} * {:?}", elem.1 .1, elem.0 + 1))
        .map(|(i, (_, score))| (1 + i) as u64 * score)
        .sum::<u64>();

    for (hand, score) in hand_score.iter() {
        println!("{score} - {hand:?}");
    }

    println!("{score}");
    Ok(())
}
