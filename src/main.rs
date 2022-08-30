mod deck;
extern crate rand;

use std::fmt;
use std::fmt::Formatter;
use deck::{Card, Deck};

pub struct Hand<'a> {
    cards_num: usize,
    cards: Vec<&'a Card>,
}

pub struct Rank {
    grade: u8,
    cards: Vec<&Card>,
}

impl<'a> Hand<'a> {
    pub fn new() -> Self {
        Self {
            cards: Vec::<&'a Card>::new(),
            cards_num: 0,
        }
    }

    pub fn get_cards(&self) -> &Vec<&Card> { &self.cards }

    pub fn clone_cards(&self) -> Vec<&Card> { self.cards.clone() }

    pub fn add_card(&mut self, card: &'a Card) -> () {
        if self.cards_num < 7 {
            self.cards.push(card);
            self.cards_num += 1;
        } else {
            ()
        }
    }


    pub fn delete_minor_shapes(&self) -> Self {
        let mut shape_cnt: [u8; 4] = [0; 4];
        self.cards
            .iter()
            .for_each(|card| shape_cnt[card.shape] += 1);
        let mut major_shape: usize = 4;
        shape_cnt
            .into_iter()
            .enumerate()
            .for_each(|(i, each_shp_cnt)| {
                if each_shp_cnt >= 5 {
                    major_shape = i
                }
            });
        for (i, each_shp_cnt) in shape_cnt.into_iter().enumerate() {
            if each_shp_cnt >= 5 {
                major_shape = i;
                break;
            }
        }
        let filtered_cards: Vec<&Card> = self
            .cards
            .iter()
            .filter(|card| card.shape == major_shape)
            .cloned()
            .collect();
        Self {
            cards_num: filtered_cards.len(),
            cards: filtered_cards,
        }
    }

    pub fn delete_minor_nums(&self) -> Self {
        let mut num_cnt: [u8; 13] = [0; 13];
        self.cards.iter().for_each(|card| num_cnt[card.num] += 1);
        let mut minor_nums = Vec::<usize>::new();
        num_cnt
            .into_iter()
            .enumerate()
            .for_each(|(i, each_num_cnt)| {
                if each_num_cnt <= 1 {
                    minor_nums.push(i)
                }
            });
        let filtered_cards: Vec<&Card> = self
            .cards
            .iter()
            .filter(|card| !minor_nums.contains(&card.num))
            .cloned()
            .collect();
        Self {
            cards_num: filtered_cards.len(),
            cards: filtered_cards,
        }
    }
}

//pub struct Rank {
 //   grade: u8,
  //  cards: Vec<&Card>,

impl Rank {
  pub fn new(hand : Hand) -> Self {
    let mut cards : Vec<&Card> = hand.clone_cards();
    cards.sort();
    let sf = Rank::is_straight_flush();
    if sf.is_some() {return sf;}
    let fk = Rank::is_four_of_a_kind();
    if fk.is_some() {return fk;}
    let fh = Rank::is_full_house();
    if fh.is_some() {return fh;}
    let fl = Rank::is_flush();
    if fl.is_some() {return fl;}
    let st = Rank::is_straight();
    if st.is_some() {return st;}
    let three = Rank::is_three_of_a_kind();
    if three.is_some() {return three;}
    let twp = Rank::is_two_pairs();
    if twp.is_some()  {return twp;}
    let op = Rank::is_one_pair();
    if op.is_some()  {return op;}
    return Rank{grade: 0, cards: cards[2..7]}
  }
  fn is_straight_flush() -> Option<Self> {
    
  }
  fn is_four_of_a_kind() -> Option<Self> {
  }
  fn is_full_house() -> Option<Self> {
  
  }
  fn is_flush(cards : &[&Card]) -> Option<Self> {
    cards
  }
  fn is_straight() -> Option<Self> {

  }
  fn is_three_of_a_kind() -> Option<Self> {
  }  
  fn is_two_pairs() -> Option<Self> {
  }
  fn one_pair() -> Option<Self> {

  }
}

impl Display for Rank {
  fn fmt(&self, f : Formatter) -> fmt::Result {
    let grade_str : &str = match grade {
      0 => "Top",
      1 => "One Pair",
      2 => "Two Pair",
      3 => "Three Of A Kind",
      4 => "Straight", 
      5 => "Flush", 
      6 => "Full House",
      7 => "Four Of A Kind",
      8 => "StraightFlush",
      _ => "Non-existant Grade",
    };
    write!(f, "{}", grade_str)
  }
}


fn main() {
    println!("Hello, world!");
    let mut deck = Deck::new();
    println!("{deck}");
    deck.shuffle();
    println!("{deck}");
}
