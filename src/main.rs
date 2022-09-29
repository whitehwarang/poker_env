mod deck;
mod hand;
mod grade;
mod holdem;
extern crate rand;

use std::collections::HashSet;
use crate::deck::{Card, Deck};
use crate::hand::Hand;
use crate::grade::*;
use crate::holdem::*;

fn main() {
        let mut deck = Deck::new();
        deck.deal_cards(13);
        deck.deal_cards(13);
        deck.deal_cards(13);
        deck.deal_cards(5);
        let mut cards: Vec<Card> = deck.deal_cards(7);
        let sf = Grade::is_straight_flush(&mut cards);
        assert!(sf.is_some());
	let sf = sf.unwrap();
	println!("{sf:?}");

    println!("Hello, world!");
}
