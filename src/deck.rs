extern crate rand;

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Card {
    pub shape: usize,
    pub num: usize,
}

pub struct Deck {
    pub cards: [Card; 52],
    pub cursor: usize,
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let shape_str: &str = match self.shape {
            0 => "S",
            1 => "D",
            2 => "H",
            3 => "C",
            _ => "Not Exist Shape",
        };
        let num_str: &str = match self.num {
            0 => "A",
            1 => "2",
            2 => "3",
            3 => "4",
            4 => "5",
            5 => "6",
            6 => "7",
            7 => "8",
            8 => "9",
            9 => "T",
            10 => "J",
            11 => "Q",
            12 => "K",
            _ => "Not Exist Number",
        };
        write!(f, "{}{}", shape_str, num_str)
    }
}

impl<'a> Deck {
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
    pub fn shuffle(&'a mut self) -> () {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
        // println!("{self}");
    }
    pub fn deal_card(&'a mut self) -> &'a Card {
        let deal_card: &Card = &self.cards[self.cursor];
        self.cursor += 1;
        deal_card
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

#[cfg(test)]
mod test_for_deck {
    use crate::deck::*;
    #[test]
    fn deck_contains_all_cards() {
        let mut deck = Deck::new();
        let mut check_mat = [[false; 13]; 4];
        deck.shuffle();
        deck.cards
            .iter()
            .for_each(|card| check_mat[card.shape][card.num] = true);
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
        let c0 = *deck.deal_card();
        let c1 = *deck.deal_card();
        let c2 = *deck.deal_card();
        let c3 = *deck.deal_card();
        assert_eq!(c0.shape, 0);
        assert_eq!(c0.num, 0);
        assert_eq!(c1.shape, 0);
        assert_eq!(c1.num, 1);
        assert_eq!(c2.shape, 0);
        assert_eq!(c2.num, 2);
        assert_eq!(c3.shape, 0);
        assert_eq!(c3.num, 3);
    }
}
