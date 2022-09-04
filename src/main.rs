mod table;
mod grade2;
extern crate rand;

use std::collections::HashSet;
use crate::table::{Card, Deck, Hand};
use crate::grade2::*;

fn main() {
    println!("Hello, world!");
    //let mut deck = Deck::new();
    //println!("{deck}");
    // let mut p1 = Hand::new();
    let cards = vec![Card{shape:0, num:5}, Card{shape:1, num:5}, Card{shape:2, num:5}, Card{shape:3, num:4}, 
		     Card{shape:2, num:4}, Card{shape:2, num:10}, Card{shape:3, num:9}];  // deck.deal_cards(3);
    println!("{cards:?}");
    //p1.add_cards(cards);
    let r = Grade::new(&cards);
    //let c : &HashSet<Card> = &r.cards;
    println!("{r}");
    //println!("{c:?}");
    //deck.shuffle();
    //println!("deck suffled.");
    //println!("{cards:?}");
    //println!("{deck}");
    //println!("{p1}");
}
