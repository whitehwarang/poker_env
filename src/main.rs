mod deck;
extern crate rand;

use deck::{Card, Deck};
use std::fmt;
use std::fmt::{Display, Formatter};

pub struct Hand {
    cards_num: usize,
    cards: Vec<Card>,
}

pub struct Rank {
    grade: u8,
    cards: Vec<Card>,
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

    pub fn add_card(&mut self, card: Card) -> () {
        if self.cards_num < 7 {
            self.cards.push(card);
            self.cards_num += 1;
        }
    }
}

pub struct Rank {
    grade: u8,
    cards: Vec<Card>,
}

impl Rank {
    pub fn new(hand: &Hand) -> Self {
        let mut cards: Vec<Card> = hand.clone_cards();
        cards.sort();

        let sf = Rank::is_straight_flush(&cards);
        if sf.is_some() {
            return sf.unwrap();
        }
        let fk = Rank::is_four_of_a_kind(&cards);
        if fk.is_some() {
            return fk.unwrap();
        }
        let fh = Rank::is_full_house(&cards);
        if fh.is_some() {
            return fh.unwrap();
        }
        let fl = Rank::is_flush(&cards);
        if fl.is_some() {
            return fl.unwrap();
        }
        let st = Rank::is_straight(&cards);
        if st.is_some() {
            return st.unwrap();
        }
        let three = Rank::is_three_of_a_kind(&cards);
        if three.is_some() {
            return three.unwrap();
        }
        let twp = Rank::is_two_pairs(&cards);
        if twp.is_some() {
            return twp.unwrap();
        }
        let op = Rank::is_one_pair(&cards);
        if op.is_some() {
            return op.unwrap();
        }
        return Rank {
            grade: 0,
            cards: cards[2..7],
        };
    }
    fn is_straight_flush(cards: &[Card]) -> Option<Self> {
        // grade:8
        let shape_cnts = Rank::shape_counts(cards);
        let sh_cnt: Option<_> = shape_cnts
            .iter()
            .enumerate()
            .filter(|(shape, shape_cnt)| shape_cnt >= 5)
            .nth(0);
        sh_cnt
	
    }
    fn is_four_of_a_kind(cards: &[Card]) -> Option<Self> { // grade:7
	// (4, ~)
	let num_cnts = Rank::num_counts(cards);
	let four_kind = num_cnts
	    .iter()
 	    .enumerate()
            .filter(|(num, num_cnt)| num_cnt == 4 )
            .nth(0);
        match four_kind {
            Some(num, _)) => Some(
				Rank{
				    grade : 7, 
				    cards : cards
						.iter()
						.filter(|card| card.num == num)
						.chain( cards
							.iter()
							.rev()
							.filter(|card| card.num != num)
							.take(1) 
						).collect::<Vec<Card>>(), }
	    	    		),
	    None => None,
        }
    }
    fn is_full_house(cards: &[Card]) -> Option<Self> {
        let num_cnts = Rank::num_counts(cards);
        // (3, 2, 1, 1) or (3, 3, 1) or (3, 2, 2) => full house
        let triplets = num_cnts
            .iter()
            .enumerate()
            .filter(|(i, num_cnt)| num_cnt == 3)
            .take(2)
            .collect::<Vec<(usize, usize)>>();
        if triplets.len() == 0 {
            return None;
        }
        if triplets.len() == 2 {
            // (3, 3, 1)
            return Some(Rank {
                grade: 6,
                cards: cards
                    .into_iter()
                    .filter(|card| card.num == triplets[0].1 || card.num == triplets[1].1)
                    .rev()
                    .take(5)
                    .collect::<Vec<Card>>(),
            });
        }
        // triplets.len() == 1
        let double = num_cnts
            .iter()
            .enumerate()
            .filter(|(i, num_cnt)| num_cnt == 2)
            .max_by_key(|(i, num_cnt)| num_cnt);
        match double {
            Some((i, _)) => Some(Rank {
                grade: 6,
                cards: cards
                    .iter()
                    .filter(|card| card.num == triplets[0].1)
                    .chain(cards.iter().filter(|card| card.num == i))
                    .collect::<Vec<Card>>(),
            }),
            None => None,
        }
    }
    fn is_flush(cards: &[Card]) -> Option<Self> {
        // grade : 5
        let shape_cnts = Rank::shape_counts(cards);
        let shcnt: Option<_> = shape_cnts
            .iter()
            .enumerate()
            .filter(|(shape, shape_cnt)| shape_cnt >= 5)
            .nth(0);
        match shcnt {
            Some((shape, _)) => Some(Rank {
                grade: 5,
                cards: cards
                    .into_iter()
                    .filter(|card| card.shape == shape)
                    .rev()
                    .take(5)
                    .collect::<Vec<Card>>(),
            }),
            None => None,
        }
    }
    fn is_straight(cards: &[Card]) -> Option<Self> {
        // grade : 4
        let num_cnts = Rank::num_counts(cards);
        // special case : A2345 (0,1,2,3,12)
        let mut consec_cnt: u8 = 0;
	let mut ace_exist : bool = false;
        for (num, num_cnt) in num_cnts.iter().enumerate().rev() {
	    if num == 12 {ace_exist = true;}
            if num_cnt >= 1 {
                consec_cnt += 1;
                if ( (consec_cnt == 5) || (num == 0 && ace_exist && consec_cnt == 4) ) { 
                    Rank{grade : 4, 
                         cards : cards.iter().filter(|card| card.num == ) }
                }
            } else {
                consec_cnt = 0;
            }
        }
    }
    fn is_three_of_a_kind(cards: &[Card]) -> Option<Self> {
        // grade : 3
        let num_cnts = Rank::num_counts(cards);
        let num_triplet: Option<_> = num_cnts
            .iter()
            .enumerate()
            .filter(|(num, num_cnt)| num_cnt == 3)
            .nth(0);
        match num_triplet {
            Some((num, _)) => Some(Rank {
                grade: 3,
                cards: cards
                    .iter()
                    .filter(|card| card.num == num)
                    .chain(cards.iter().rev().filter(|card| card.num != num))
                    .take(5)
                    .collect::<Vec<Card>>(),
            }),
            None => None,
        }
    }
    fn is_two_pairs(cards: &[Card]) -> Option<Self> {
        // grade : 2
        // (2, 2, 1, 1, 1), (2, 2, 2, 1)
        let num_cnts = Rank::num_counts(cards);
        let num_pairs = num_cnts
            .iter()
            .enumerate()
            .filter(|(num, num_cnt)| num_cnt == 2)
            .rev()
            .take(3)
            .collect::<Vec<(usize, usize)>>();
        let num_pairs_len = num_pairs.len();
        if num_pairs_len < 2 {
            return None;
        }
        // let higher_pair_nums = num_pairs[..2].iter().map(|(num, num_cnt)| num ).collect::<Vec<usize>>();
        // if num_pairs_len in (2, 3) {
        return Some(Rank {
            grade: 2,
            cards: cards
                .iter()
                .filter(|card| card.num == num_pairs[0].0 || card.num == num_pairs[1].0)
                .chain(
                    cards
                        .iter()
                        .rev()
                        .filter(|card| card.num != num_pairs[0].0 && card.num == num_pairs[1].0)
                        .take(1),
                )
                .collect::<Vec<Card>>(),
        });
    }
    fn is_one_pair(cards: &[Card]) -> Option<Self> {
        // grade : 1
        let num_cnts = Rank::num_counts(cards);
        let num_pair: Option<_> = num_cnts
            .iter()
            .enumerate()
            .filter(|(num, num_cnt)| num_cnt == 2)
            .nth(0);
        match i_ncnt {
            Some((1, _)) => Some(Rank {
                grade: 1,
                cards: cards
                    .iter()
                    .filter(|card| card.num == num_pair.0)
                    .chain(
                        cards
                            .iter()
                            .rev()
                            .filter(|card| card.num != num_pair.0)
                            .take(3),
                    )
                    .collect::<Vec<Card>>(),
            }),
            None => None,
        }
    }

    fn shape_counts(cards: &[Card]) -> [usize; 4] {
        let mut cnts: [usize; 4] = [0; 4];
        cards.iter().for_each(|card| cnts[card.shape] += 1);
        cnts
    }
    fn num_counts(cards: &[Card]) -> [usize; 13] {
        let mut cnts: [usize; 13] = [0; 13];
        cards.iter().for_each(|card| cnts[card.num] += 1);
        cnts
    }
    /*    pub fn delete_minor_shapes(&self) -> Self {
        let mut shape_cnts: [u8; 4] = [0; 4];
        self.cards
            .iter()
            .for_each(|card| shape_cnt[card.shape] += 1);
        let mut major_shape: usize = 4;
        shape_cnts
            .into_iter()
            .enumerate()
            .for_each(|(i, each_shp_cnt)| {
                if each_shp_cnt >= 5 {
                    major_shape = i
                }
            });
        for (i, each_shp_cnt) in shape_cnts.into_iter().enumerate() {
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
    }*/
}

impl Display for Rank {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let grade_str: &str = match self.grade {
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
    let card: Card = deck.deal_card();
    println!("{card}");
    deck.shuffle();
    println!("deck suffled.");
    println!("{card}");
    println!("{deck}");
}
