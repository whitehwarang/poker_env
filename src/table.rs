extern crate rand;

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

#[derive(Copy, Clone, Eq, PartialEq, Ord, Hash)]
pub struct Card {
    pub shape: usize,
    pub num: usize,
}

pub struct Deck {
    pub cards: [Card; 52],
    pub cursor: usize,
}

pub struct Hand {
    cards_num: usize,
    pub cards: Vec<Card>,
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.num.partial_cmp(&other.num)
    }
}

impl Card {
    fn to_string(&self) -> String {
        let shape_str: &str = match self.shape {
            0 => "S",
            1 => "D",
            2 => "H",
            3 => "C",
            _ => "Not Exist Shape",
        };
        let num_str: &str = match self.num {
            0 => "2",
            1 => "3",
            2 => "4",
            3 => "5",
            4 => "6",
            5 => "7",
            6 => "8",
            7 => "9",
            8 => "T",
            9 => "J",
            10 => "Q",
            11 => "K",
            12 => "A",
            _ => "Not Exist Number",
        };
        shape_str.to_string() + num_str
    }
}

impl Debug for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Display for Deck {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.cards
                .iter()
                .map(|&x| x.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.cards/*.iter().collect::<String>()*/ )
    }
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = [Card { shape: 0, num: 0 }; 52];
        let cursor: usize = 0;
        for shape in 0..=3 {
            for num in 0..=12 {
                cards[shape * 13 + num] = Card { shape, num }
            }
        }
        Self { cards, cursor }
    }

    pub fn shuffle(&mut self) -> () {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn deal_cards(&mut self, num: usize) -> Vec<Card> {
        let deal_card: Vec<Card> = self.cards[self.cursor..(self.cursor + num)].to_vec();
        self.cursor += num;
        deal_card
    }
}

impl Hand {
    pub fn new() -> Self {
        Self {
            cards: Vec::<Card>::new(),
            cards_num: 0,
        }
    }
    pub fn clone_cards(&self) -> Vec<Card> {
        self.cards.clone()
    }
    pub fn add_cards(&mut self, cards: Vec<Card>) -> () {
        //    if self.cards_num < 7 {
        cards.into_iter().for_each(|card| {
            if self.cards_num < 7 {
                self.cards.push(card);
                self.cards_num += 1;
            }
        })
    }
    pub fn len(&self) -> usize {
        self.cards_num
    }
}

#[cfg(test)]
mod test_for_deck {
    use crate::table::*;
    #[test]
    fn deck_contains_all_cards() {
        let mut deck = Deck::new();
        let mut check_mat = [[false; 13]; 4];
        deck.shuffle();
        deck.cards
            .iter()
            .for_each(|&card| check_mat[card.shape][card.num] = true);
        let mut tf_cnt: u8 = 0;
        for sub1 in check_mat {
            for sub2 in sub1 {
                if sub2 {
                    tf_cnt += 1;
                }
            }
        }
        assert_eq!(tf_cnt, 52);
    }
    #[test]
    fn dealing_card_is_successful() {
        let mut deck = Deck::new();
        let c0 = deck.deal_cards(1);
        let c1 = deck.deal_cards(1);
        let c2 = deck.deal_cards(1);
        let c3 = deck.deal_cards(1);
        assert_eq!(c0[0].shape, 0);
        assert_eq!(c0[0].num, 0);
        assert_eq!(c1[0].shape, 0);
        assert_eq!(c1[0].num, 1);
        assert_eq!(c2[0].shape, 0);
        assert_eq!(c2[0].num, 2);
        assert_eq!(c3[0].shape, 0);
        assert_eq!(c3[0].num, 3);
    }
    #[test]
    fn hand_get_cards_from_deck() {
        let mut deck = Deck::new();
        let mut p1 = Hand::new();
        p1.add_cards(deck.deal_cards(7));
        assert_eq!(p1.len(), 7);
        assert_eq!(p1.cards[6].num, 6);
    }
}
