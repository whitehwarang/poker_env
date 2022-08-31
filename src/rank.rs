use crate::table::{Card, Hand};
use std::fmt;
use std::collections::HashSet;

pub struct Rank {
    pub grade: u8,
    pub cards: HashSet<Card>,
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let grade_str: &str = match self.grade {
            0 => "Top		   :  ",
            1 => "One Pair  	   :  ",
            2 => "Two Pairs  	   :  ",
            3 => "Three Of A Kind  :  ",
            4 => "Straight  	   :  ",
            5 => "Flush		   :  ",
            6 => "Full House	   :  ",
            7 => "Four Of A Kind   :  ",
            8 => "Straight Flush   :  ",
            _ => "Non-existant Grade  :  ",
        };

        let mut cards_string = String::from("");
        for card in self.cards.iter() {
            cards_string += &(card.to_string() + " ");
        }
        write!(f, "{}", grade_str.to_string() + &cards_string)
    }
}

impl Rank {
    pub fn new(hand: &Hand) -> Self {
        let mut cards: Vec<Card> = hand.clone_cards();
        cards.sort();

        let sf = Self::is_straight_flush(&cards);
        if sf.is_some() {
            return sf.unwrap();
        }
        let fk = Self::is_four_of_a_kind(&cards);
        if fk.is_some() {
            return fk.unwrap();
        }
        let fh = Self::is_full_house(&cards);
        if fh.is_some() {
            return fh.unwrap();
        }
        let fl = Self::is_flush(&cards);
        if fl.is_some() {
            return fl.unwrap();
        }
        let st = Self::is_straight(&cards);
        if st.is_some() {
            return st.unwrap();
        }
        let three = Self::is_three_of_a_kind(&cards);
        if three.is_some() {
            return three.unwrap();
        }
        let twp = Self::is_two_pairs(&cards);
        if twp.is_some() {
            return twp.unwrap();
        }
        let op = Self::is_one_pair(&cards);
        if op.is_some() {
            return op.unwrap();
        }
        return Self {
            grade: 0,
            cards: {
               let mut hs : HashSet<Card> = HashSet::new();
               for card in cards[2..7].into_iter() { hs.insert(*card); }
               hs },
        };
    }

    fn is_straight_flush(cards: &[Card]) -> Option<Self> {
        // grade:8
        let shape_cnts = Rank::shape_counts(cards);
        let sh_cnt: Option<_> = shape_cnts
            .into_iter()
            .enumerate()
            .filter(|&(_shape, shape_cnt)| shape_cnt >= 5)
            .nth(0);
        if sh_cnt.is_none() {
            return None;
        }
        let shape = sh_cnt.unwrap().0;
        let shape_filtered_cards = cards
            .into_iter()
            .cloned()
            .filter(|&card| card.shape == shape)
            .collect::<Vec<Card>>();

        let num_cnts = Rank::num_counts(&shape_filtered_cards);
        // special case : A2345 (0,1,2,3,12)
        let mut consec_cnt: u8 = 0;
        let mut ace_exist: bool = false;
        for (num, num_cnt) in num_cnts.into_iter().enumerate().rev() {
            if num == 12 {
                ace_exist = true;
            }
            if num_cnt >= 1 {
                consec_cnt += 1;
                if (consec_cnt == 5) || (num == 0 && ace_exist && consec_cnt == 4) {
                    return Some(Rank {
                        grade: 8,
                        cards: cards
                            .iter()
                            .cloned()
                            .filter(|&card| {
                                if ace_exist && num == 0 {
                                    (card.num <= 3) || (card.num == 12)
                                } else {
                                    card.num <= num + 4 && card.num >= num
                                }
                            })
                            .collect::<HashSet<Card>>(),
                    }); // end of Some
                } // end of if ( ( consec_cnt ~
            } else {
                consec_cnt = 0;
            }
        } // end of for
        return None;
    }

    fn is_four_of_a_kind(cards: &[Card]) -> Option<Self> {
        // grade:7
        // (4, ~)
        let num_cnts = Rank::num_counts(cards);
        let four_kind = num_cnts
            .into_iter()
            .enumerate()
            .filter(|&(_num, num_cnt)| num_cnt == 4)
            .nth(0);
        match four_kind {
            Some((num, _)) => Some(Rank {
                grade: 7,
                cards: cards
                    .iter()
                    .cloned()
                    .filter(|&card| card.num == num)
                    .chain(
                        cards
                            .iter()
                            .cloned()
                            .rev()
                            .filter(|&card| card.num != num)
                            .take(1),
                    )
                    .collect::<HashSet<Card>>(),
            }), // end of Some
            None => None,
        } // end of match four_kind
    } // end of is_four_of_a_kind

    fn is_full_house(cards: &[Card]) -> Option<Self> {
        let num_cnts = Rank::num_counts(cards);
        // (3, 2, 1, 1) or (3, 3, 1) or (3, 2, 2) => full house
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
            return Some(Rank {
                grade: 6,
                cards: cards
                    .iter()
                    .cloned()
                    .filter(|&card| card.num == triplets[0].0 || card.num == triplets[1].0)
                    .rev()
                    .take(5)
                    .collect::<HashSet<Card>>(),
            });
        }
        // triplets.len() == 1
        println!("{cards:?}");
        let double = num_cnts
            .into_iter()
            .enumerate()
            .filter(|&(_num, num_cnt)| num_cnt == 2)
            .max_by_key(|&(_num, num_cnt)| num_cnt);
        match double {
            Some((num, _)) => Some(Rank {
                grade: 6,
                cards: cards
                    .iter()
                    .cloned()
                    .filter(|&card| card.num == triplets[0].0 || card.num == num)
                    .collect::<HashSet<Card>>(),
            }),
            None => None,
        }
    }

    fn is_flush(cards: &[Card]) -> Option<Self> {
        // grade : 5
        let shape_cnts = Rank::shape_counts(cards);
        let shcnt: Option<_> = shape_cnts
            .into_iter()
            .enumerate()
            .filter(|&(_shape, shape_cnt)| shape_cnt >= 5)
            .nth(0);
        match shcnt {
            Some((shape, _)) => Some(Rank {
                grade: 5,
                cards: cards
                    .into_iter()
                    .cloned()
                    .filter(|&card| card.shape == shape)
                    .rev()
                    .take(5)
                    .collect::<HashSet<Card>>(),
            }),
            None => None,
        }
    }

    fn is_straight(cards: &[Card]) -> Option<Self> {
        // grade : 4
        let num_cnts = Rank::num_counts(cards);
        let mut cards = cards.to_vec();
        cards.dedup();
        // special case : A2345 (0,1,2,3,12)
        let mut consec_cnt: u8 = 0;
        let mut ace_exist: bool = false;
        for (num, num_cnt) in num_cnts.into_iter().enumerate().rev() {
            if num == 12 {
                ace_exist = true;
            }
            if num_cnt >= 1 {
                consec_cnt += 1;
                if (consec_cnt == 5) || (num == 0 && ace_exist && consec_cnt == 4) {
                    return Some(Rank {
                        grade: 4,
                        cards: cards
                            .into_iter()
                            .filter(|&card| {
                                if ace_exist && num == 0 {
                                    (card.num <= 3) || (card.num == 12)
                                } else {
                                    card.num <= num + 4 && card.num >= num
                                }
                            })
                            .collect::<HashSet<Card>>(),
                    });
                }
            } else {
                consec_cnt = 0;
            }
        }
        return None;
    }

    fn is_three_of_a_kind(cards: &[Card]) -> Option<Self> {
        // grade : 3
        let num_cnts = Rank::num_counts(cards);
        let num_triplet: Option<_> = num_cnts
            .into_iter()
            .enumerate()
            .filter(|&(_num, num_cnt)| num_cnt == 3)
            .nth(0);
        match num_triplet {
            Some((num, _)) => Some(Rank {
                grade: 3,
                cards: cards
                    .into_iter()
                    .cloned()
                    .filter(|&card| card.num == num)
                    .chain(
                        cards
                            .into_iter()
                            .cloned()
                            .rev()
                            .filter(|&card| card.num != num),
                    )
                    .take(5)
                    .collect::<HashSet<Card>>(),
            }),
            None => None,
        }
    }
    fn is_two_pairs(cards: &[Card]) -> Option<Self> {
        // grade : 2
        // (2, 2, 1, 1, 1), (2, 2, 2, 1)
        let num_cnts = Rank::num_counts(cards);
        let num_pairs = num_cnts
            .into_iter()
            .enumerate()
            .filter(|&(_num, num_cnt)| num_cnt == 2)
            .rev()
            .take(3)
            .collect::<Vec<(usize, usize)>>();
        let num_pairs_len = num_pairs.len();
        if num_pairs_len < 2 {
            return None;
        }

        return Some(Rank {
            grade: 2,
            cards: cards
                .iter()
                .cloned()
                .filter(|&card| card.num == num_pairs[0].0 || card.num == num_pairs[1].0)
                .chain(
                    cards
                        .iter()
                        .cloned()
                        .rev()
                        .filter(|&card| card.num != num_pairs[0].0 && card.num != num_pairs[1].0)
                        .take(1),
                )
                .collect::<HashSet<Card>>(),
        });
    }
    fn is_one_pair(cards: &[Card]) -> Option<Rank> {
        // grade : 1
        let num_cnts = Rank::num_counts(cards);
        let num_pair: Option<_> = num_cnts
            .into_iter()
            .enumerate()
            .filter(|&(_num, num_cnt)| num_cnt == 2)
            .nth(0);
        match num_pair {
            Some((num, _)) => Some(Rank {
                grade: 1,
                cards: cards
                    .iter()
                    .cloned()
                    .filter(|&card| card.num == num)
                    .chain(
                        cards
                            .iter()
                            .cloned()
                            .rev()
                            .filter(|&card| card.num != num)
                            .take(3),
                    )
                    .collect::<HashSet<Card>>(),
            }),
            None => None,
        }
    }

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

