extern crate itertools;

use crate::table::{Card, Hand};
use itertools::Itertools;
use std::cmp::Ordering;
use std::fmt;

#[derive(PartialEq)]
pub enum Grade<'a> {
    StraightFlush((&'a Card, &'a Card, &'a Card, &'a Card, &'a Card)),
    FourOfAKind((&'a Card, &'a Card, &'a Card, &'a Card, &'a Card)),
    FullHouse((&'a Card, &'a Card, &'a Card, &'a Card, &'a Card)),
    Flush((&'a Card, &'a Card, &'a Card, &'a Card, &'a Card)),
    Straight((&'a Card, &'a Card, &'a Card, &'a Card, &'a Card)),
    ThreeOfAKind((&'a Card, &'a Card, &'a Card, &'a Card, &'a Card)),
    TwoPairs((&'a Card, &'a Card, &'a Card, &'a Card, &'a Card)),
    OnePair((&'a Card, &'a Card, &'a Card, &'a Card, &'a Card)),
    Top((&'a Card, &'a Card, &'a Card, &'a Card, &'a Card)),
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

impl<'a> Grade {
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
            Self::StraightFlush((a, b, c, d, e)) => (8, (*a).num, (*b).num, (*c).num, (*d).num, (*e).num),
            Self::FourOfAKind((a, b, c, d, e)) => (7, (*a).num, (*b).num, (*c).num, (*d).num, (*e).num),
            Self::FullHouse((a, b, c, d, e)) => (6, (*a).num, (*b).num, (*c).num, (*d).num, (*e).num),
            Self::Flush((a, b, c, d, e)) => (5, (*a).num, (*b).num, (*c).num, (*d).num, (*e).num),
            Self::Straight((a, b, c, d, e)) => (4, (*a).num, (*b).num, (*c).num, (*d).num, (*e).num),
            Self::ThreeOfAKind((a, b, c, d, e)) => (3,(*a).num, (*b).num, (*c).num, (*d).num, (*e).num),
            Self::TwoPairs((a, b, c, d, e)) => (2, (*a).num, (*b).num, (*c).num, (*d).num, (*e).num),
            Self::OnePair((a, b, c, d, e)) => (1, (*a).num, (*b).num, (*c).num, (*d).num, (*e).num),
            Self::Top((a, b, c, d, e)) => (0, (*a).num, (*b).num, (*c).num, (*d).num, (*e).num),
        }
    } // end :: as_value()

    pub fn new(cards: &[Card]) -> Self {
        let mut cards: Vec<Card> = cards.clone().to_vec();
        cards.sort();
        cards.reverse();
        let shape_cnts = Self::shape_counts(&cards);
        let num_cnts   = Self::num_counts(&cards);
        if let Some(sf) = Self::is_straight_flush(&cards, &shape_cnts, &num_cnts) {
            return sf;
        }
        if let Some(fk) = Self::is_four_of_a_kind(&cards, &shape_cnts, &num_cnts) {
            return fk;
        }
        if let Some(fh) = Self::is_full_house(&cards, &shape_cnts, &num_cnts) {
            return fh;
        }
        if let Some(fl) = Self::is_flush(&cards, &shape_cnts, &num_cnts) {
            return fl;
        }
        if let Some(st) = Self::is_straight(&cards, &num_cnts) {
            return st;
        }
        if let Some(tk) = Self::is_three_of_a_kind(&cards, &shape_cnts, &num_cnts) {
            return tk;
        }
        if let Some(tp) = Self::is_two_pairs(&cards, &shape_cnts, &num_cnts) {
            return tp;
        }
        if let Some(op) = Self::is_one_pair(&cards, &shape_cnts, &num_cnts) {
            return op;
        }
        Self::Top(Self::extract_top5(&cards))
    } // end :: new()

    fn extract_top5(cards: &[Card]) -> (&'a Card, &'a Card, &'a Card, &'a Card, &'a Card) {
        cards
            .iter()
            .take(5)
            .collect_tuple()
            .unwrap()
    }
    /*  // currently not used code.
    fn extract_top3(cards: &[Card]) -> (usize, usize, usize) {
        cards
            .iter()
            .map(|&card| card.num)
            .take(3)
            .collect_tuple()
            .unwrap()
    }
    fn extract_top2(cards: &[Card]) -> (usize, usize) {
        cards
            .iter()
            .map(|&card| card.num)
            .take(2)
            .collect_tuple()
            .unwrap()
    }
    fn extract_top1(cards: &[Card]) -> (usize,) {
        cards
            .iter()
            .map(|&card| card.num)
            .take(1)
            .collect_tuple()
            .unwrap()
    }*/

    fn shape_counts(cards: &[Card]) -> [usize; 4] {
        let mut cnts: [usize; 4] = [0; 4];
        cards.iter().for_each(|&card| cnts[card.shape] += 1);
        cnts
    }

    fn num_counts(cards: &[Card]) -> [usize; 13] {
        let mut cnts: [usize; 13] = [0; 13];
        cards.iter().for_each(|&card| cnts[card.num] += 1);
        cnts
    }

    fn is_straight_flush(cards: &[Card], shape_cnts: &[usize; 4], num_cnts: &[usize; 13]) -> Option<Self> {
        // search a major shape of the cards.
        let sh_cnt: Option<_> = shape_cnts
            .into_iter()
            .enumerate()
            .filter(|&(_shape, shape_cnt)| *shape_cnt >= 5)
            .nth(0);
        if sh_cnt.is_none() {
            return None;
        }
        let shape = sh_cnt.unwrap().0;

        // filter the cards by the major shape.
        let shape_filtered_cards = cards
            .into_iter()
            .cloned()
            .filter(|&card| card.shape == shape)
            .collect::<Vec<Card>>();

        // reuse a code for checking if it is straight.
        match Self::is_straight(&shape_filtered_cards, &num_cnts) {
            Some(Self::Straight(st_cards)) => Some(Self::StraightFlush(st_cards)),
            None => None,
            _ => None,
        }
    }

    fn is_four_of_a_kind(cards: &[Card], shape_cnts: &[usize; 4], num_cnts: &[usize; 13]) -> Option<Self> {
        // search a major num of the cards.
        let four_kind: Option<_> = num_cnts
            .into_iter()
            .enumerate()
            .filter(|&(_num, num_cnt)| num_cnt == 4)
	    .map(|(num, _num_cnt)| num)
            .nth(0);

        // check
        match four_kind {
            Some(major_num) => {
		cards.sort_by_key(|&card| if card.num == major_num {card.num+13} else {card.num});
		let sorted_cards = cards.into_iter().rev().take(5).collect_tuple().unwrap();
		return Some(Self::FourOfAKind(sorted_cards));
            },
            None => None,
        } // end of match
    } // end of is_four_of_a_kind

    fn is_full_house(cards: &[Card], shape_cnts: &[usize; 4], num_cnts: &[usize; 13]) -> Option<Self> {
        // search major nums of the cards.
        // (3, 2, 1, 1) or (3, 2, 2) or (3, 3, 1) => full house
        let triplets_num = num_cnts.into_iter()..enumerate()
            .filter(|&(_num, num_cnt)| num_cnt == 3)
            .take(2)
            .map(|(num, _num_cnt)| num)
            .collect::<Vec<usize>>();
        if triplets.len() == 0 {
            return None;
        }
	let pairs_num = num_cnt.into_iter().enumerate().filter(|&(_num, num_cnt)| num_cnt == 2).take(2).map(|(num, _num_cnt)| num).collect::<Vec<usize>>();
	if triplets.len() + pairs_num.len() <= 2 {return None;}

	cards.sort_by_key(|&card| if triplets_num.contains(&card.num) {card.num+26} 
			     else if pairs_num.contains(&card.num) {card.num+13} 
				else {card.num});
	let sorted_cards = cards.into_iter().rev().take(5).collect_tuple().unwrap();
	return Some(Self::FullHouse(sorted_cards));
        /*
	if triplets.len() == 2 {
            // (3, 3, 1)
            let major_cards = cards
                .iter()
                .cloned()
                .filter(|&card| card.num == triplets[0].0 || card.num == triplets[1].0)
                .take(5)
                .collect_tuple()
                .unwrap();
            return Some(Self::FullHouse(major_cards));
        }
        // if triplets.len() == 1
        let higher_pair = num_cnts
            .into_iter()
            .enumerate()
            .filter(|&(_num, num_cnt)| num_cnt == 2)
            .max_by_key(|&(num, num_cnt)| num);
        match higher_pair {
	    let major_cards = cards.iter().cloned().filter(|&card| card.num == triplets[0].0 || card.num == higher_pair).collect_tuple().unwrap(); 
            Some((card1, card2)) => Some(Self::FullHouse(major_cards)),
            None => None,
        }*/
    }

    fn is_flush(cards: &[Card], shape_cnts: &[usize; 4], num_cnts: &[usize; 13]) -> Option<Self> {
        // grade : 5
        let shcnt: Option<_> = shape_cnts
            .into_iter()
            .enumerate()
            .filter(|&(_shape, shape_cnt)| *shape_cnt >= 5)
            .nth(0);
        match shcnt {
            Some((major_shape, _)) => {
                let highers = cards
                    .iter()
                    .filter(|&card| card.shape == major_shape)
                    .take(5)
                    .collect_tuple()
                    .unwrap();
                Some(Self::Flush(highers))
            }
            None => None,
        }
    }

    fn is_straight(cards: &[Card], num_cnts: &[usize; 13]) -> Option<Self> {
        // search straight nums.
        let mut cards = cards.to_vec().dedup();
         // delete duplicates by card.num
        // must consider a special case : A2345 (0,1,2,3,12)
        let mut consec_cnt: u8 = 0;
        let mut ace_exist: bool = false;
        let str_lnum = num_cnts.into_iter().enumerate().rev().for_each(|(num, num_cnt)| {
            if num == 12 	{ace_exist = true;}
            if *num_cnt == 0 	{consec_cnt = 0;} 
	    else {
                consec_cnt += 1;
		if consec_cnt == 5 || (consec_cnt == 4 && num == 0 && ace_exist) {
			num
		}
	});
	match consec_cnt {
                5 => {
			cards.sort_by_key(|&card| if card.num >= str_lnum && card.num <= str_lnum + 4 {card.num + 13} else card.num);
			let sorted_cards = cards.into_iter().rev().take(5).collect_tuple().unwrap();
			return Some(Self::Straight(sorted_cards));
		},
                4 if str_lnum == 0 && ace_exist => {
			cards.sort_by_key(|&card| if card.num >= 0 && card.num <= 3 {card.num + 13} else card.num)
			let sorted_cards = cards.into_iter().rev().take(5).collect_tuple().unwrap();
			return Some(Self::Straight(sorted_cards));
	        },
                _ => return None,
        };
    }

    fn is_three_of_a_kind(cards: &[Card], shape_cnts: &[usize; 4], num_cnts: &[usize; 13]) -> Option<Self> {
        // grade : 3
        let num_triplet: Option<_> = num_cnts
            .into_iter()
            .enumerate()
            .filter(|&(_num, num_cnt)| *num_cnt == 3)
            .nth(0);
        match num_triplet {
            Some((num, _)) => {
		cards.sort_by_key(|&card| if card.num == num {13} else {card.num});
		let sorted_cards = cards.into_iter().rev().take(5).collect_tuple().unwrap();
                Some(Self::ThreeOfAKind(sorted_cards))
            },
            None => None,
        }
    }

    fn is_two_pairs(cards: &'a [Card], shape_cnts: &'a [usize; 4], num_cnts: &'a [usize; 13]) -> Option<Self> {
        // grade : 2
        // (2, 2, 1, 1, 1), (2, 2, 2, 1)
        let pairs = num_cnts
            .iter()
            .enumerate()
            .filter(|&(_num, num_cnt)| *num_cnt == 2)
            .rev()
            .take(3)
            .collect::<Vec<(usize, &usize)>>();
        let pairs_len = pairs.len();
        match pairs_len {
            0..=1 => None,
            2..=3 => {
		cards.sort_by_key(|&card| if card.num == pairs[0].0 {14} else if card.num == pairs[1].0 {13} else {card.num});
                let sorted_cards = cards.into_iter().rev().take(5).collect_tuple().unwrap();
                Some(Self::TwoPairs(sorted_cards))
            },
            _ => None,
        }
    }

    fn is_one_pair(cards: &[Card], shape_cnts: &[usize; 4], num_cnts: &[usize; 13]) -> Option<Self> {
        // grade : 1
        let pair_num = num_cnts
            .into_iter()
            .enumerate()
            .filter(|&(_num, num_cnt)| *num_cnt == 2)
            .nth(0);
        match pair_num {
            None => None,
            Some((num, _)) => {
		cards.sort_by_key(|&card| if card.num == num {15} else {card.num});
                let sorted_cards = cards.iter().rev().take(5).collect_tuple().unwrap();
                Some(Self::OnePair(sorted_cards))
            },
        }
    }
}

/*
}
#[cfg(test)]
mod test_for_rank {
    use super::*;
    use crate::table::*;
    #[test]
    fn ranking_test_straight_flush() {
        let mut deck = Deck::new();
        //deck.deal_cards(13);deck.deal_cards(13);deck.deal_cards(13);deck.deal_cards(8);
        let cards: Vec<Card> = deck.deal_cards(7);
        let sf = Rank::is_straight_flush(&cards);
        assert!(sf.is_some());
        assert_eq!(sf.unwrap().cards, HashSet::from([
            Card{shape:0, num:2},Card{shape:0, num:3},Card{shape:0, num:4},
            Card{shape:0, num:5},Card{shape:0, num:6}]));
    }
    #[test]
    fn ranking_test_four_of_a_kind() {
        let cards: Vec<Card> = vec![
            Card { shape: 0, num: 5 },
            Card { shape: 1, num: 5 },
            Card { shape: 2, num: 5 },
            Card { shape: 3, num: 5 },
            Card { shape: 0, num: 1 },
            Card { shape: 2, num: 10 },
            Card { shape: 3, num: 11 },
        ];
        let fk = Rank::is_four_of_a_kind(&cards);
        assert!(fk.is_some());
        assert_eq!(
            fk.unwrap().cards,
            HashSet::from([
                Card { shape: 0, num: 5 },
                Card { shape: 1, num: 5 },
                Card { shape: 2, num: 5 },
                Card { shape: 3, num: 5 },
                Card { shape: 3, num: 11 }
            ])
        );
    }
    #[test]
    fn ranking_test_full_house() {
        let cards: Vec<Card> = vec![
            Card { shape: 0, num: 10 },
            Card { shape: 1, num: 5 },
            Card { shape: 2, num: 5 },
            Card { shape: 3, num: 4 },
            Card { shape: 3, num: 4 },
            Card { shape: 2, num: 4 },
            Card { shape: 0, num: 5 },
        ];
        let mut hand: Hand = Hand::new();
        hand.add_cards(cards);
        let rank = Rank::new(&hand);
        assert_eq!(rank.grade, 6);
        assert_eq!(
            rank.cards,
            HashSet::from([
                Card { shape: 1, num: 5 },
                Card { shape: 2, num: 5 },
                Card { shape: 0, num: 5 },
                Card { shape: 3, num: 4 },
                Card { shape: 2, num: 4 }
            ])
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

*/
