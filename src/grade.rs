extern crate itertools;

use crate::table::{Card, Hand};
use itertools::Itertools;
use std::cmp::Ordering;
use std::fmt;

#[derive(PartialEq)]
pub enum Grade {
    StraightFlush((usize, usize, usize, usize, usize)),
    FourOfAKind((usize, usize, usize, usize, usize)),
    FullHouse((usize, usize, usize, usize, usize)),
    Flush((usize, usize, usize, usize, usize)),
    Straight((usize, usize, usize, usize, usize)),
    ThreeOfAKind((usize, usize, usize, usize, usize)),
    TwoPairs((usize, usize, usize, usize, usize)),
    OnePair((usize, usize, usize, usize, usize)),
    Top((usize, usize, usize, usize, usize)),
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
            Self::StraightFlush(tuple) => format!("Straight Flush	: {:?}, {:?}, {:?}, {:?}, {:?}", tuple.0, tuple.1, tuple.2, tuple.3, tuple.4),
            Self::FourOfAKind(tuple) => format!("Four Of A Kind	: {:?} {:?}, {:?}, {:?}, {:?}", tuple.0, tuple.1, tuple.2, tuple.3, tuple.4),
            Self::FullHouse(tuple) => format!("Full House		: {:?} {:?}, {:?}, {:?}, {:?}", tuple.0, tuple.1, tuple.2, tuple.3, tuple.4),
            Self::Flush(tuple) => format!("Flush		: {:?} {:?}, {:?}, {:?}, {:?}", tuple.0, tuple.1, tuple.2, tuple.3, tuple.4),
            Self::Straight(tuple) => format!("Straight		: {:?} {:?}, {:?}, {:?}, {:?}", tuple.0, tuple.1, tuple.2, tuple.3, tuple.4),
            Self::ThreeOfAKind(tuple) => format!("Three Of A Kind	: {:?} {:?}, {:?}, {:?}, {:?}", tuple.0, tuple.1, tuple.2, tuple.3, tuple.4),
            Self::TwoPairs(tuple) => format!("TwoPairs		: {:?} {:?}, {:?}, {:?}, {:?}", tuple.0, tuple.1, tuple.2, tuple.3, tuple.4),
            Self::OnePair(tuple) => format!("One Pair		: {:?} {:?}, {:?}, {:?}, {:?}", tuple.0, tuple.1, tuple.2, tuple.3, tuple.4),
            Self::Top(tuple) => format!("Top			: {:?} {:?}, {:?}, {:?}, {:?}", tuple.0, tuple.1, tuple.2, tuple.3, tuple.4),
        }
    } // end :: to_string()

    fn as_value(&self) -> (u8, usize, usize, usize, usize, usize) {
        match self {
            Self::StraightFlush((a, b, c, d, e)) => (8, *a, *b, *c, *d, *e),
            Self::FourOfAKind((a, b, c, d, e)) => (7, *a, *b, *c, *d, *e),
            Self::FullHouse((a, b, c, d, e)) => (6, *a, *b, *c, *d, *e),
            Self::Flush((a, b, c, d, e)) => (5, *a, *b, *c, *d, *e),
            Self::Straight((a, b, c, d, e)) => (4, *a, *b, *c, *d, *e),
            Self::ThreeOfAKind((a, b, c, d, e)) => (3, *a, *b, *c, *d, *e),
            Self::TwoPairs((a, b, c, d, e)) => (2, *a, *b, *c, *d, *e),
            Self::OnePair((a, b, c, d, e)) => (1, *a, *b, *c, *d, *e),
            Self::Top((a, b, c, d, e)) => (0, *a, *b, *c, *d, *e),
        }
    } // end :: as_value()

    pub fn new(cards: &[Card]) -> Self {
        let mut cards: Vec<Card> = cards.clone().to_vec();
        cards.sort();
        cards.reverse();
        if let Some(sf) = Self::is_straight_flush(&cards) {
            return sf;
        }
        if let Some(fk) = Self::is_four_of_a_kind(&cards) {
            return fk;
        }
        if let Some(fh) = Self::is_full_house(&cards) {
            return fh;
        }
        if let Some(fl) = Self::is_flush(&cards) {
            return fl;
        }
        if let Some(st) = Self::is_straight(&cards) {
            return st;
        }
        if let Some(tk) = Self::is_three_of_a_kind(&cards) {
            return tk;
        }
        if let Some(tp) = Self::is_two_pairs(&cards) {
            return tp;
        }
        if let Some(op) = Self::is_one_pair(&cards) {
            return op;
        }
        Self::Top(Self::extract_top5(&cards))
    } // end :: new()

    fn extract_top5(cards: &[Card]) -> (usize, usize, usize, usize, usize) {
        cards
            .iter()
            .map(|&card| card.num)
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

    fn is_straight_flush(cards: &[Card]) -> Option<Self> {
        // search a major shape of the cards.
        let shape_cnts = Self::shape_counts(cards);
        let sh_cnt: Option<_> = shape_cnts
            .into_iter()
            .enumerate()
            .filter(|&(_shape, shape_cnt)| shape_cnt >= 5)
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
        match Self::is_straight(&shape_filtered_cards) {
            Some(Self::Straight(nums)) => Some(Self::StraightFlush(nums)),
            None => None,
            _ => None,
        }
    }

    fn is_four_of_a_kind(cards: &[Card]) -> Option<Self> {
        // search a major num of the cards.
        let num_cnts = Self::num_counts(cards);
        let four_kind: Option<_> = num_cnts
            .into_iter()
            .enumerate()
            .filter(|&(_num, num_cnt)| num_cnt == 4)
            .nth(0);

        // check
        match four_kind {
            Some((major_num, _)) => {
                let another_highest = cards
                    .into_iter()
                    .cloned()
                    .filter(|&card| card.num != major_num)
                    .max_by_key(|&card| card.num)
                    .map(|card| card.num)
                    .unwrap();
                Some(Self::FourOfAKind((
                    major_num,
                    major_num,
                    major_num,
                    major_num,
                    another_highest,
                )))
            }
            None => None,
        } // end of match
    } // end of is_four_of_a_kind

    fn is_full_house(cards: &[Card]) -> Option<Self> {
        // search major nums of the cards.
        let num_cnts = Self::num_counts(cards);
        // (3, 2, 1, 1) or (3, 2, 2) or (3, 3, 1) => full house
        let triplets = num_cnts
            .into_iter()
            .enumerate()
            .filter(|&(_num, num_cnt)| num_cnt == 3)
            .take(2)
            .collect::<Vec<(usize, usize)>>();
        if triplets.len() == 0 {
            return None;
        }
        if triplets.len() == 2 {
            // (3, 3, 1)
            let higher_nums = cards
                .iter()
                .cloned()
                .filter(|&card| card.num == triplets[0].0 || card.num == triplets[1].0)
                .take(5)
                .map(|card| card.num)
                .collect_tuple()
                .unwrap();
            return Some(Self::FullHouse(higher_nums));
        }
        // if triplets.len() == 1
        let high_pair_cnt = num_cnts
            .into_iter()
            .enumerate()
            .filter(|&(_num, num_cnt)| num_cnt == 2)
            .max_by_key(|&(num, num_cnt)| num);
        match high_pair_cnt {
            Some((higher_p_num, _)) => Some(Self::FullHouse((
                triplets[0].0,
                triplets[0].0,
                triplets[0].0,
                higher_p_num,
                higher_p_num,
            ))),
            None => None,
        }
    }

    fn is_flush(cards: &[Card]) -> Option<Self> {
        // grade : 5
        let shape_cnts = Self::shape_counts(cards);
        let shcnt: Option<_> = shape_cnts
            .into_iter()
            .enumerate()
            .filter(|&(_shape, shape_cnt)| shape_cnt >= 5)
            .nth(0);
        match shcnt {
            Some((major_shape, _)) => {
                let highers = cards
                    .into_iter()
                    .cloned()
                    .filter(|&card| card.shape == major_shape)
                    .take(5)
                    .map(|card| card.num)
                    .collect_tuple()
                    .unwrap();
                Some(Self::Flush(highers))
            }
            None => None,
        }
    }

    fn is_straight(cards: &[Card]) -> Option<Self> {
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
                return match consec_cnt {
                    5 => Some(Self::Straight((num + 4, num + 3, num + 2, num + 1, num))),
                    4 if num == 0 && ace_exist => Some(Self::Straight((3, 2, 1, 0, 0))),
                    _ => None,
                };
            }
        }
        return None;
    }

    fn is_three_of_a_kind(cards: &[Card]) -> Option<Self> {
        // grade : 3
        let num_cnts = Self::num_counts(cards);
        let num_triplet: Option<_> = num_cnts
            .into_iter()
            .enumerate()
            .filter(|&(_num, num_cnt)| num_cnt == 3)
            .nth(0);
        match num_triplet {
            Some((num, _)) => {
                let res_cards: (usize, usize) = cards
                    .into_iter()
                    .filter(|&card| card.num != num)
                    .take(2)
                    .map(|&card| card.num)
                    .collect_tuple()
                    .unwrap();
                Some(Self::ThreeOfAKind((
                    num,
                    num,
                    num,
                    res_cards.0,
                    res_cards.1,
                )))
            }
            None => None,
        }
    }

    fn is_two_pairs(cards: &[Card]) -> Option<Self> {
        // grade : 2
        // (2, 2, 1, 1, 1), (2, 2, 2, 1)
        let num_cnts = Self::num_counts(cards);
        let num_pairs = num_cnts
            .into_iter()
            .enumerate()
            .filter(|&(_num, num_cnt)| num_cnt == 2)
            .rev()
            .take(3)
            .collect::<Vec<(usize, usize)>>();
        let num_pairs_len = num_pairs.len();
        match num_pairs_len {
            0..=1 => None,
            2..=3 => {
                let res_num = cards
                    .into_iter()
                    .filter(|&card| card.num != num_pairs[0].0 && card.num != num_pairs[1].0)
                    .nth(0)
                    .unwrap()
                    .num;
                Some(Self::TwoPairs((
                    num_pairs[0].0,
                    num_pairs[0].0,
                    num_pairs[1].0,
                    num_pairs[1].0,
                    res_num,
                )))
            }
            _ => None,
        }
    }

    fn is_one_pair(cards: &[Card]) -> Option<Self> {
        // grade : 1
        let num_cnts = Self::num_counts(cards);
        let pair_num: Option<usize> = num_cnts
            .into_iter()
            .enumerate()
            .filter(|&(_num, num_cnt)| num_cnt == 2)
            .map(|(num, _num_cnt)| num)
            .nth(0);
        match pair_num {
            None => None,
            Some(num) => {
                let res_nums: (usize, usize, usize) = cards
                    .into_iter()
                    .filter(|&card| card.num != num)
                    .map(|card| card.num)
                    .take(3)
                    .collect_tuple()
                    .unwrap();
                Some(Self::OnePair((
                    num, num, res_nums.0, res_nums.1, res_nums.2,
                )))
            }
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
