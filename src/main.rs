mod table;
mod grade;
mod holdem;
extern crate rand;

use std::collections::HashSet;
use crate::table::{Card, Deck, Hand};
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
	//let g = sf.0;
	//let cards = sf.1;
	//println!("{g:?}");
	//println!("{cards:?}");
	//println!("{sf}");

    println!("Hello, world!");
    //let mut deck = Deck::new();
    //println!("{deck}");
    // let mut p1 = Hand::new();
   // let cards = vec![Card{shape:0, num:5}, Card{shape:1, num:5}, Card{shape:2, num:5}, Card{shape:3, num:4}, 
//		     Card{shape:2, num:4}, Card{shape:2, num:10}, Card{shape:3, num:9}];  // deck.deal_cards(3);
 //   println!("{cards:?}");
    //p1.add_cards(cards);
 //   let r = Grade::new(&cards);
    //let c : &HashSet<Card> = &r.cards;
 //   println!("{r}");
    //println!("{c:?}");
    //deck.shuffle();
    //println!("deck suffled.");
    //println!("{cards:?}");
    //println!("{deck}");
    //println!("{p1}");
}
