extern crate rand;

use std::fmt;
use std::fmt::{Display, Formatter};
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug, Copy, Clone)]
struct Card {
  shape : usize,
  num   : usize,
}

#[derive(Debug)]
struct Deck {
  cards : [Card; 52],
  cursor: usize,
}

impl Display for Card {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    let shape_str : &str = match self.shape {
			0 => "S",
			1 => "D", 
			2 => "H", 
			3 => "C",
			_ => "Not Exist Shape",
		    };
    let num_str : &str = match self.num {
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

impl Deck {
  fn new() -> Self {
    let mut cards = [Card{shape:0, num:0}; 52];
    let cursor : usize = 0;
    for shape in 0..=3 {
      for num in 0..=12 {
	cards[shape * 13 + num] = Card{shape, num}
      }
    }
    Self{cards, cursor}
  }
  fn shuffle(&mut self) -> () {
    let mut rng = thread_rng();
    self.cards.shuffle(&mut rng);
    // println!("{self}");
  }
}

impl Display for Deck {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.cards.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(", ") )
  }
}

fn main() {
    println!("Hello, world!");
    let mut deck = Deck::new(); 
    println!("{deck}");
    deck.shuffle();
    println!("{deck}");
}
