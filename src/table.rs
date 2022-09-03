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
    fn shape_to_string(&self) -> String {
        match self.shape {
            0 => String::from("S"),
            1 => String::from("D"),
            2 => String::from("H"),
            3 => String::from("C"),
            _ => String::from("Not Existant Shape"),
        }
    }

    fn num_to_string(&self) -> String {
        match self.num {
            0 => String::from("2"),
            1 => String::from("3"),
            2 => String::from("4"),
            3 => String::from("5"),
            4 => String::from("6"),
            5 => String::from("7"),
            6 => String::from("8"),
            7 => String::from("9"),
            8 => String::from("T"),
            9 => String::from("J"),
            10 => String::from("Q"),
            11 => String::from("K"),
            12 => String::from("A"),
            _ => String::from("Not Exist Number"),
        }
    }

    fn to_string(&self) -> String {
        self.shape_to_string() + &self.num_to_string()
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
        write!(f, "{:?}", self.cards /*.iter().collect::<String>()*/)
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
