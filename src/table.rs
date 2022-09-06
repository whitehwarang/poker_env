extern crate rand;

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp;
use std::fmt;

#[derive(Copy, Clone, Eq, PartialEq, Ord, Hash)]
pub struct Card {
    pub shape: usize,
    pub num: usize,
}

#[derive(fmt::Debug, Copy, Clone)]
pub struct Deck {
    pub cards: [Card; 52],
    pub cursor: usize,
}

#[derive(fmt::Debug, Clone)]
pub struct Hand {
    cards_cnt: usize,
    pub cards: Vec<Card>,
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.num.partial_cmp(&other.num)
    }
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.cards)
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
        self.cards.shuffle(&mut rng); // rand::seq::SliceRandom;
    }

    pub fn partial_shuffle(&mut self) -> () {
	let mut rng = thread_rng();
	self.cards[self.cursor..].shuffle(&mut rng);
    }

    pub fn set_front_cards(&mut self, front_cards: Vec<Card>) -> Result<(), &str> {
	if self.cursor >= 2 {return Err("Front Cards' Slot Already occupied.");}
	let i_front_cards: Vec<usize> = self.cards.iter()
		.enumerate()
		.filter(|(_i, &card)| front_cards.contains(&card))
		.map(|(i, _card)| i )
		.collect::<_>();
	self.cards.swap(0, i_front_cards[0]);
	self.cards.swap(1, i_front_cards[1]);
	/*i_front_cards.into_iter()
		.enumerate()
		.for_each(|(j, (i, &card))| {self.cards.swap(j, i);});*/
	self.cursor = front_cards.len();
	return Ok(());
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
            cards_cnt: 0,
	    cards: Vec::<Card>::new(),
        }
    }
    pub fn clone(&self) -> Self {
	let cards_cnt = self.cards_cnt;
	let cards = self.cards.clone();
	Self {cards_cnt, cards}
    }

    pub fn add_card(&mut self, card: Card) -> () {
        if self.cards_cnt < 7 {
	    self.cards.push(card);
	    self.cards_cnt += 1;
	}
    }

    pub fn add_cards(&mut self, cards: &[Card]) -> () {
        //    if self.cards_cnt < 7 {
        cards.into_iter().for_each(|card| {
            if self.cards_cnt < 7 {
                self.cards.push(*card);
                self.cards_cnt += 1;
            }
        })
    }
    pub fn len(&self) -> usize {
        self.cards_cnt
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
        let fc = deck.deal_cards(4);
	deck.deal_cards(13);
	let fc2 = deck.deal_cards(3);
	assert_eq!(fc[0], Card{shape:0, num:0});
	assert_eq!(fc[1], Card{shape:0, num:1});
	assert_eq!(fc[2], Card{shape:0, num:2});
	assert_eq!(fc[3], Card{shape:0, num:3});
	assert_eq!(fc2[0], Card{shape:1, num:4});
	assert_eq!(fc2[1], Card{shape:1, num:5});
	assert_eq!(fc2[2], Card{shape:1, num:6});
    }
    #[test]
    fn hand_get_cards_from_deck() {
        let mut deck = Deck::new();
        let mut p1 = Hand::new();
        p1.add_cards(&deck.deal_cards(7));
        assert_eq!(p1.len(), 7);
        assert_eq!(p1.cards[6], Card{shape:0, num:6});
    }
    #[test]
    fn partial_shuffling_test() {
	let mut deck = Deck::new();
	let cards: Vec<Card> = deck.deal_cards(7);
	let post_cards : Vec<Card> = deck.cards[7..14].to_vec();
	deck.partial_shuffle();
	assert_eq!(cards, deck.cards[..7]);
	assert_ne!(post_cards, deck.cards[7..14]);
    }
}
