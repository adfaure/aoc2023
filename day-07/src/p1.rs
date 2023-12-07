use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::Hash;
use std::io::BufRead;
use std::{fs::File, io::BufReader};

#[derive(PartialEq, PartialOrd, Hash, Eq, Debug, Clone, Copy)]
enum Card {
    Head(char),
    Number(u32),
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Card::Head(_), Card::Number(_)) => Ordering::Greater,
            (Card::Number(_), Card::Head(_)) => Ordering::Less,
            (Card::Head(this), Card::Head(other)) if other == this => Ordering::Equal,
            (Card::Head(this), Card::Head(other)) => match (this, other) {
                ('A', _) => Ordering::Greater,
                (_, 'A') => Ordering::Less,

                ('K', _) => Ordering::Greater,
                (_, 'K') => Ordering::Less,

                ('Q', _) => Ordering::Greater,
                (_, 'Q') => Ordering::Less,

                ('J', _) => Ordering::Greater,
                (_, 'J') => Ordering::Less,

                ('T', _) => Ordering::Greater,
                (_, 'T') => Ordering::Less,

                (_, _) => panic!(),
            },
            (Card::Number(this), Card::Number(other)) => this.cmp(other),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct Hand(Card, Card, Card, Card, Card);

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.0, self.1, self.2, self.3, self.4).cmp(&(other.0, other.1, other.2, other.3, other.4))
    }
}

impl Hand {
    fn to_vec(self) -> Vec<Card> {
        [self.0, self.1, self.2, self.3, self.4].to_vec()
    }
}

impl Eq for Hand {}

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
        let grouped_cards = hand.clone().to_vec().iter().fold(
            HashMap::new() as HashMap<Card, u32>,
            |mut cards, card| {
                // println!("{:?} - {:?}", *card, card);
                let total = cards.entry(*card).or_insert(0);
                *total += 1;
                cards
            },
        );

        let copy_for_debug = grouped_cards.clone();

        let hand = if grouped_cards.len() == 1 {
            Type::FiveOfAKind(hand)
        } else if grouped_cards.len() == 2 {
            if grouped_cards.into_values().max().unwrap() == 4 {
                Type::FourOfAKind(hand)
            } else {
                Type::FullHouse(hand)
            }
        } else if grouped_cards.len() == 3 {
            if grouped_cards.into_values().max().unwrap() == 3 {
                Type::ThreeOfAKind(hand)
            } else {
                Type::TwoPair(hand)
            }
        } else if grouped_cards.len() == 4 {
            Type::OnePair(hand)
        } else if grouped_cards.len() == 5 {
            Type::HighCard(hand)
        } else {
            unreachable!();
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
                            Some(value) => Card::Number(value),
                            None => Card::Head(c),
                        })
                        .collect_tuple::<(Card, Card, Card, Card, Card)>()
                        .unwrap(),
                ),
                score_str.parse::<u64>().unwrap(),
            )
        })
        .map(|(hand, score)| (Type::from(hand), score))
        .collect_vec();

    hand_score.sort_by(|(h, b), (o_h, o_b)| h.cmp(o_h));

    let score = hand_score
        .iter()
        .enumerate()
        .inspect(|elem| println!("{:?} * {:?}", elem.1 .1, elem.0 + 1))
        .map(|(i, (_, score))| (1 + i) as u64 * score)
        .sum::<u64>();

    for (hand, score) in hand_score.iter() {
        println!("{score} - {hand:?}");
    }

    println!("{score}");
    Ok(())
}
