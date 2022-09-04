extern crate itertools;

use crate::table::{Card};
use itertools::Itertools;
use std::cmp::Ordering;
use std::fmt;

#[derive(PartialEq, Debug)]
pub enum Grade {
    StraightFlush((Card, Card, Card, Card,Card)),
    FourOfAKind((Card, Card, Card, Card,Card)),
    FullHouse((Card, Card, Card, Card,Card)),
    Flush((Card, Card, Card, Card,Card)),
    Straight((Card, Card, Card, Card,Card)),
    ThreeOfAKind((Card, Card, Card, Card,Card)),
    TwoPairs((Card, Card, Card, Card,Card)),
    OnePair((Card, Card, Card, Card,Card)),
    Top((Card, Card, Card, Card,Card)),
}

impl PartialOrd for Grade {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_value().partial_cmp(&other.as_value())
    }
}

impl fmt::Display for Grade {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Grade {
    pub fn to_string(&self) -> String {
        match self {
            Self::StraightFlush(tuple) => format!("Straight Flush	: {:?}", tuple),
            Self::FourOfAKind(tuple) => format!("Four Of A Kind	: {:?}", tuple),
            Self::FullHouse(tuple) => format!("Full House		: {:?}", tuple),
            Self::Flush(tuple) => format!("Flush		: {:?}", tuple),
            Self::Straight(tuple) => format!("Straight		: {:?}", tuple),
            Self::ThreeOfAKind(tuple) => format!("Three Of A Kind	: {:?}", tuple),
            Self::TwoPairs(tuple) => format!("TwoPairs		: {:?}", tuple),
            Self::OnePair(tuple) => format!("One Pair		: {:?}", tuple),
            Self::Top(tuple) => format!("Top			: {:?}", tuple),
        }
    } // end :: to_string()

    fn as_value(&self) -> (u8, usize, usize, usize, usize, usize) {
        match self {
            Self::StraightFlush((a, b, c, d, e)) => (8, a.num, b.num, c.num, d.num, e.num),
            Self::FourOfAKind((a, b, c, d, e)) => (7,a.num, b.num, c.num, d.num, e.num),
            Self::FullHouse((a, b, c, d, e)) => (6, a.num, b.num, c.num, d.num, e.num),
            Self::Flush((a, b, c, d, e)) => (5,a.num, b.num, c.num, d.num, e.num),
            Self::Straight((a, b, c, d, e)) => (4, a.num, b.num, c.num, d.num, e.num),
            Self::ThreeOfAKind((a, b, c, d, e)) => (3, a.num, b.num, c.num, d.num, e.num),
            Self::TwoPairs((a, b, c, d, e)) => (2, a.num, b.num, c.num, d.num, e.num),
            Self::OnePair((a, b, c, d, e)) => (1, a.num, b.num, c.num, d.num, e.num),
            Self::Top((a, b, c, d, e)) => (0, a.num, b.num, c.num, d.num, e.num),
        }
    } // end :: as_value()

    pub fn new(cards: &[Card]) -> Self {
        let mut cards: Vec<Card> = cards.clone().to_vec();
        if let Some(sf) = Self::is_straight_flush(&mut cards) {
            return sf;
        }
        if let Some(fk) = Self::is_four_of_a_kind(&mut cards) {
            return fk;
        }
        if let Some(fh) = Self::is_full_house(&mut cards) {
            return fh;
        }
        if let Some(fl) = Self::is_flush(&mut cards) {
            return fl;
        }
        if let Some(st) = Self::is_straight(&mut cards) {
            return st;
        }
        if let Some(tk) = Self::is_three_of_a_kind(&mut cards) {
            return tk;
        }
        if let Some(tp) = Self::is_two_pairs(&mut cards) {
            return tp;
        }
        if let Some(op) = Self::is_one_pair(&mut cards) {
            return op;
        }
        Self::Top(Self::extract_top5(&mut cards))
    } // end :: new()

    fn extract_top5(cards: &mut [Card]) -> (Card, Card, Card, Card, Card) {
        cards.sort();
	cards
            .iter().cloned()
	    .rev()
            .take(5)
            .collect_tuple()
            .unwrap()
    }

    fn shape_counts(cards: &[Card]) -> [usize; 4] {
        let mut cnts: [usize; 4] = [0; 4];
        cards.iter().for_each(|&card| cnts[card.shape] += 1);
        return cnts;
    }

    fn get_major_shape(cards: &[Card], threshold: usize) -> Option<usize> {
        let shape_cnts = Self::shape_counts(&cards);
        return shape_cnts.iter().enumerate().filter(|&(_shape, shape_cnt)| *shape_cnt >= threshold).map(|(shape, _shape_cnt)| shape ).nth(0);
    }

    fn filter_by_shape(cards: &[Card], shape: usize) -> Vec<Card> {
	return cards.into_iter().cloned().filter(|&card| card.shape == shape).collect::<Vec<Card>>();
    }

    fn num_counts(cards: &[Card]) -> [usize; 13] {
        let mut cnts: [usize; 13] = [0; 13];
        cards.iter().for_each(|&card| cnts[card.num] += 1);
        return cnts;
    }
    
    fn get_major_num(cards: &[Card], num_of_cards: usize) -> Option<usize> {
	let num_cnts = Self::num_counts(&cards);
        return num_cnts.iter().enumerate().filter(|&(_num, num_cnt)| *num_cnt == num_of_cards).map(|(num, _num_cnt)|num).nth(0);
    }

    fn take_highest_5_cards(cards: &[Card]) -> (Card, Card, Card, Card, Card) {
        return cards.iter().cloned().rev().take(5).collect_tuple().unwrap();
    }

    fn is_straight_flush(cards: &mut [Card]) -> Option<Self> {
        // search a major shape of the cards.

        let opt_major_shape: Option<_> = Self::get_major_shape(&cards, 5);
        if opt_major_shape.is_none() {
            return None;
        }
        let major_shape = opt_major_shape.unwrap();

        // filter the cards by the major shape.
        let mut shape_filtered_cards : Vec<_> = Self::filter_by_shape(&cards, major_shape);
        // reuse a code for checking if it is straight.
        match Self::is_straight(&mut shape_filtered_cards) {
            Some(Self::Straight(same_shaped_straight_cards)) => Some(Self::StraightFlush(same_shaped_straight_cards)),
            None => None,
            _ => None,
        }
    }

    fn is_four_of_a_kind(cards: &mut [Card]) -> Option<Self> {
        // search a major num of the cards.
        let four_kind_num: Option<_> = Self::get_major_num(&cards, 4);

        // check
        match four_kind_num {
            Some(major_num) => {
		cards.sort_by_key(|&card| if card.num == major_num {13+card.num} else {card.num});
		let made_cards = Self::take_highest_5_cards(&cards);
		return Some(Self::FourOfAKind(made_cards));
            },
            None => None,
        } // end of match
    } // end of is_four_of_a_kind

    fn is_full_house(cards: &mut [Card]) -> Option<Self> {
        // search major nums of the cards.
	//let triplets : Option<_> = Self::get_major_num(&cards, 3);
	let num_cnts = Self::num_counts(&cards);
        // (3, 2, 1, 1) or (3, 2, 2) or (3, 3, 1) => full house
        let triplets = num_cnts
            .iter().cloned()
            .enumerate()
            .filter(|&(_num, num_cnt)| num_cnt == 3)
	    .map(|(num, _num_cnt)| num )
            .take(2)
            .collect::<Vec<usize>>();
        if triplets.len() == 0 {
            return None;
        }
	let twins = num_cnts.iter().cloned().enumerate().filter(|&(_num, num_cnt)| num_cnt == 2).take(2).map(|(num, _num_cnt)| num).collect::<Vec<usize>>();
	if triplets.len() + twins.len() <= 1 {return None;}
	
	cards.sort_by_key(|&card| if triplets.contains(&card.num) {card.num + 26} else if twins.contains(&card.num) {card.num + 13} else {card.num});
	let made_cards = Self::take_highest_5_cards(&cards);
        return Some(Self::FullHouse(made_cards));
    }

    fn is_flush(cards: &mut [Card]) -> Option<Self> {
        // grade : 5
	let opt_major_shape : Option<usize> = Self::get_major_shape(cards, 5);
        match opt_major_shape {
            Some(major_shape) => {
		cards.sort_by_key(|&card| if card.shape == major_shape {card.num+13} else {card.num} );
		cards.reverse();
                let made_cards = Self::take_highest_5_cards(&cards);
		Some(Self::Flush(made_cards))
            },
            None => None,
        }
    }

    fn is_straight(cards: &mut [Card]) -> Option<Self> {
        // search straight nums.
        let num_cnts = Self::num_counts(cards);
        let mut cards = cards.to_vec();
        cards.dedup(); // delete duplicates by card.num
                       // must consider a special case : A2345 (0,1,2,3,12)
        let mut consec_cnt: u8 = 0;
        let mut ace_exist: bool = false;
        for (num, num_cnt) in num_cnts.into_iter().enumerate().rev() {
            if num == 12 {
                ace_exist = true;
            }
            if num_cnt == 0 {
                consec_cnt = 0;
            } else {
                consec_cnt += 1;
                match consec_cnt {
                    5 => {
			cards.sort_by_key(|&card| if card.num >= num && card.num <= num + 4 {card.num + 20} else {card.num} );
			cards.reverse();
			let made_cards = Self::take_highest_5_cards(&cards);
			return Some(Self::Straight(made_cards));
		    },  //return Some(Self::Straight((num + 4, num + 3, num + 2, num + 1, num))),
                    4 if num == 0 && ace_exist => {
			cards.sort_by_key(|&card| if card.num <= 3 {card.num + 20} else {card.num} );
			cards.reverse();
			let made_cards = Self::take_highest_5_cards(&cards);
			return Some(Self::Straight(made_cards));
		    }, //Some(Self::Straight((3, 2, 1, 0, 0))),
                    _ => (),
                };
            }
        }
        return None;
    }

    fn is_three_of_a_kind(cards: &mut [Card]) -> Option<Self> {
        // grade : 3
	let num_triplet: Option<_> = Self::get_major_num(&cards, 3);
        match num_triplet {
            Some(num) => {
		cards.sort_by_key(|&card| if card.num == num {card.num + 13} else {card.num});
		cards.reverse();
		let made_cards = Self::take_highest_5_cards(&cards);
		return Some(Self::ThreeOfAKind(made_cards));
            },
            None => {return None;},
        }
    }

    fn is_two_pairs(cards: &mut [Card]) -> Option<Self> {
        // grade : 2
        // (2, 2, 1, 1, 1), (2, 2, 2, 1)
        let num_cnts = Self::num_counts(cards);
        let num_pairs = num_cnts
            .into_iter()
            .enumerate()
            .filter(|&(_num, num_cnt)| num_cnt == 2)
            .rev()
            .take(3)
  	    .map(|(num, _num_cnt)| num )
            .collect::<Vec<usize>>();
        let num_pairs_len = num_pairs.len();
        match num_pairs_len {
            0..=1 => {return None;},
            2..=3 => {
		cards.sort_by_key(|&card| if num_pairs.contains(&card.num) {card.num + 13} else {card.num} );
		cards.reverse();
		let made_cards = Self::take_highest_5_cards(&cards);
		return Some(Self::TwoPairs(made_cards));
            },
            _ => {return None;},
        }
    }

    fn is_one_pair(cards: &mut [Card]) -> Option<Self> {
        // grade : 1
        let num_cnts = Self::num_counts(cards);
        let pair_num: Option<usize> = num_cnts
            .into_iter()
            .enumerate()
            .filter(|&(_num, num_cnt)| num_cnt == 2)
            .map(|(num, _num_cnt)| num)
            .nth(0);
        match pair_num {
            None => {return None;},
            Some(num) => {
		cards.sort_by_key(|&card| if card.num == num {card.num + 13} else {card.num} );
		cards.reverse();
		let made_cards = Self::take_highest_5_cards(&cards);
		return Some(Self::OnePair(made_cards));
            },
        }
    }
}


#[cfg(test)]
mod test_for_grade {
    use super::*;
    use crate::table::*;

    #[test]
    fn test_straight_flush() {
        let mut deck = Deck::new();
        deck.deal_cards(13);deck.deal_cards(13);deck.deal_cards(13);deck.deal_cards(5);
        let cards: Vec<Card> = deck.deal_cards(7);
        let sf = Grade::is_straight_flush(&mut cards);
        assert!(sf.is_some());
        assert_eq!(sf.unwrap(), 
		   Grade::StraightFlush((
				Card{shape:3, num:9},
				Card{shape:3, num:8},
				Card{shape:3, num:7},
            			Card{shape:3, num:6},
				Card{shape:3, num:5}
			))
		  );
    }

    #[test]
    fn test_four_of_a_kind() {
        let mut cards: Vec<Card> = vec![
            Card { shape: 0, num: 5 },
            Card { shape: 1, num: 5 },
            Card { shape: 2, num: 5 },
            Card { shape: 3, num: 5 },
            Card { shape: 0, num: 1 },
            Card { shape: 2, num: 10 },
            Card { shape: 3, num: 11 },
        ];
        let fk = Grade::is_four_of_a_kind(&mut cards);
        assert!(fk.is_some());
	assert_eq!(fk.unwrap().grade, 7);
        assert_eq!(
            fk.unwrap(),
            Grade::FourOfAKind((
                Card { shape: 3, num: 5 },
                Card { shape: 2, num: 5 },
                Card { shape: 1, num: 5 },
                Card { shape: 0, num: 5 },
                Card { shape: 3, num: 11 }
            ))
        );
    }

    #[test]
    fn test_full_house() {
        let cards: Vec<Card> = vec![
            Card { shape: 0, num: 10 },
            Card { shape: 1, num: 5 },
            Card { shape: 2, num: 5 },
            Card { shape: 3, num: 4 },
            Card { shape: 3, num: 4 },
            Card { shape: 2, num: 4 },
            Card { shape: 0, num: 5 },
        ];
        //let mut hand: Hand = Hand::new();
        //hand.add_cards(cards);
        let fh = Grade::new(&mut cards);
        assert_eq!(fh.unwrap().grade, 6);
        assert_eq!(
            HashSet::from(fh.unwrap()),
            Grade::FullHouse((
                Card { shape: 2, num: 5 },
                Card { shape: 1, num: 5 },
                Card { shape: 0, num: 5 },
                Card { shape: 3, num: 4 },
                Card { shape: 2, num: 4 }
            ))
        );
    }

    #[test]
    fn ranking_test_flush() {
        let cards: Vec<Card> = vec![Card{shape:0, num:12},Card{shape:0, num:11},Card{shape:1, num:12},Card{shape:2, num:12},
                Card{shape:0, num:0},Card{shape:0, num:3},Card{shape:0, num:10}	];
        let mut hand: Hand = Hand::new();
        hand.add_cards(cards);
        let rank = Rank::new(&hand);
        assert_eq!(rank.grade, 5);
        assert_eq!(rank.cards, HashSet::from([  Card{shape:0, num:12},
                        Card{shape:0, num:11},
                        Card{shape:0, num:10},
                        Card{shape:0, num:0},
                        Card{shape:0, num:3}]));
    }
    #[test]
    fn ranking_test_straight() {
        let cards: Vec<Card> = vec![Card{shape:1, num:8},Card{shape:3, num:11},Card{shape:1, num:9},Card{shape:2, num:12},
                Card{shape:3, num:12},Card{shape:0, num:12},Card{shape:0, num:10}	];
        let mut hand: Hand = Hand::new();
        hand.add_cards(cards);
        let rank = Rank::new(&hand);
        assert_eq!(rank.grade, 4);
        assert_eq!(rank.cards, HashSet::from([  Card{shape:2, num:12},
                        Card{shape:3, num:11},
                        Card{shape:0, num:10},
                        Card{shape:1, num:9},
                        Card{shape:1, num:8}]));
     }
    #[test]
    fn ranking_test_three_of_a_kind() {
        let cards: Vec<Card> = vec![Card{shape:0, num:3},Card{shape:0, num:11},Card{shape:1, num:2},Card{shape:2, num:12},
                Card{shape:3, num:3},Card{shape:2, num:3},Card{shape:0, num:10}	];
        let mut hand: Hand = Hand::new();
        hand.add_cards(cards);
        let rank = Rank::new(&hand);
        assert_eq!(rank.grade, 3);
        assert_eq!(rank.cards, HashSet::from([  Card{shape:0, num:3},
                        Card{shape:2, num:3},
                        Card{shape:3, num:3},
                        Card{shape:2, num:12},
                        Card{shape:0, num:11}]));
    }
    #[test]
    fn ranking_test_two_pairs() {
        let cards: Vec<Card> = vec![Card{shape:0, num:1},Card{shape:0, num:12},Card{shape:1, num:7}, Card{shape:1, num:12},
                Card{shape:2, num:7},Card{shape:2, num:10},Card{shape:3, num:10}	];
        let mut hand: Hand = Hand::new();
        hand.add_cards(cards);
        let rank = Rank::new(&hand);
        assert_eq!(rank.grade, 2);
        assert_eq!(rank.cards, HashSet::from([  Card{shape:3, num:10},
                        Card{shape:2, num:10},
                        Card{shape:2, num:7},
                        Card{shape:1, num:7},
                        Card{shape:1, num:12}]));
    }
    #[test]
    fn ranking_test_own_pair() {
        let cards: Vec<Card> = vec![Card{shape:0, num:1},Card{shape:0, num:11},Card{shape:1, num:7}, Card{shape:1, num:12},
                Card{shape:2, num:7},Card{shape:2, num:2},Card{shape:3, num:10}	];
        let mut hand: Hand = Hand::new();
        hand.add_cards(cards);
        let rank = Rank::new(&hand);
        assert_eq!(rank.grade, 1);
        assert_eq!(rank.cards, HashSet::from([  Card{shape:1, num:12},
                        Card{shape:0, num:11},
                        Card{shape:2, num:7},
                        Card{shape:1, num:7},
                        Card{shape:3, num:10}]));
    }
    #[test]
    fn ranking_test_top() {
        let cards: Vec<Card> = vec![Card{shape:0, num:1},Card{shape:0, num:3},Card{shape:1, num:5}, Card{shape:1, num:7},
                Card{shape:2, num:8},Card{shape:2, num:10},Card{shape:3, num:11}	];
        let mut hand: Hand = Hand::new();
        hand.add_cards(cards);
        let rank = Rank::new(&hand);
        assert_eq!(rank.grade, 0);
        assert_eq!(rank.cards, HashSet::from([  Card{shape:3, num:11},
                        Card{shape:2, num:10},
                        Card{shape:2, num:8},
                        Card{shape:1, num:7},
                        Card{shape:1, num:5}]));
    }


}


