use crate::table::Card;
use std::cmp::Ordering;
use std::fmt;

#[derive(Debug)]
pub enum Grade {
    StraightFlush([Card; 5]),
    FourOfAKind([Card; 5]),
    FullHouse([Card; 5]),
    Flush([Card; 5]),
    Straight([Card; 5]),
    ThreeOfAKind([Card; 5]),
    TwoPairs([Card; 5]),
    OnePair([Card; 5]),
    Top([Card; 5]),
}

impl PartialOrd for Grade {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_value().partial_cmp(&other.as_value())
    }
}

impl PartialEq for Grade {
    fn eq(&self, other: &Self) -> bool {
        self.as_value().eq(&other.as_value())
    }

    fn ne(&self, other: &Self) -> bool {
        self.as_value().ne(&other.as_value())
    }
}

impl fmt::Display for Grade {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl FromIterator<Card> for [Card; 5] {
    fn from_iter<I: IntoIterator<Item = Card>>(iter: I) -> Self {
        let mut arr = [Card { shape: 5, num: 13 }; 5];
        iter.into_iter()
            .enumerate()
            .for_each(|(i, card)| arr[i] = card);
        arr
    }
}

impl Grade {
    pub fn to_string(&self) -> String {
        match self {
            Self::StraightFlush(cards) => format!("Straight Flush	: {:?}", cards),
            Self::FourOfAKind(cards) => format!("Four Of A Kind	: {:?}", cards),
            Self::FullHouse(cards) => format!("Full House		: {:?}", cards),
            Self::Flush(cards) => format!("Flush		: {:?}", cards),
            Self::Straight(cards) => format!("Straight		: {:?}", cards),
            Self::ThreeOfAKind(cards) => format!("Three Of A Kind	: {:?}", cards),
            Self::TwoPairs(cards) => format!("TwoPairs		: {:?}", cards),
            Self::OnePair(cards) => format!("One Pair		: {:?}", cards),
            Self::Top(cards) => format!("Top			: {:?}", cards),
        }
    } // end :: to_string()

    fn as_value(&self) -> (u8, usize, usize, usize, usize, usize) {
        match self {
            Self::StraightFlush([a, b, c, d, e]) => (8, a.num, b.num, c.num, d.num, e.num),
            Self::FourOfAKind([a, b, c, d, e]) => (7, a.num, b.num, c.num, d.num, e.num),
            Self::FullHouse([a, b, c, d, e]) => (6, a.num, b.num, c.num, d.num, e.num),
            Self::Flush([a, b, c, d, e]) => (5, a.num, b.num, c.num, d.num, e.num),
            Self::Straight([a, b, c, d, e]) => (4, a.num, b.num, c.num, d.num, e.num),
            Self::ThreeOfAKind([a, b, c, d, e]) => (3, a.num, b.num, c.num, d.num, e.num),
            Self::TwoPairs([a, b, c, d, e]) => (2, a.num, b.num, c.num, d.num, e.num),
            Self::OnePair([a, b, c, d, e]) => (1, a.num, b.num, c.num, d.num, e.num),
            Self::Top([a, b, c, d, e]) => (0, a.num, b.num, c.num, d.num, e.num),
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

    fn extract_top5(cards: &mut [Card]) -> [Card; 5] {
        cards.sort();
        Self::take_highest_5_cards(cards)
    }

    fn shape_counts(cards: &[Card]) -> [usize; 4] {
        let mut cnts: [usize; 4] = [0; 4];
        cards.iter().for_each(|&card| cnts[card.shape] += 1);
        return cnts;
    }

    fn get_major_shape(cards: &[Card], threshold: usize) -> Option<usize> {
        let shape_cnts = Self::shape_counts(&cards);
        return shape_cnts
            .iter()
            .enumerate()
            .filter(|&(_shape, shape_cnt)| *shape_cnt >= threshold)
            .map(|(shape, _shape_cnt)| shape)
            .nth(0);
    }

    fn filter_by_shape(cards: &[Card], shape: usize) -> Vec<Card> {
        return cards
            .into_iter()
            .cloned()
            .filter(|&card| card.shape == shape)
            .collect::<Vec<Card>>();
    }

    fn num_counts(cards: &[Card]) -> [usize; 13] {
        let mut cnts: [usize; 13] = [0; 13];
        cards.iter().for_each(|&card| cnts[card.num] += 1);
        return cnts;
    }

    fn get_major_num(cards: &[Card], num_of_cards: usize) -> Option<usize> {
        let num_cnts = Self::num_counts(&cards);
        return num_cnts
            .iter()
            .enumerate()
            .filter(|&(_num, num_cnt)| *num_cnt == num_of_cards)
            .map(|(num, _num_cnt)| num)
            .nth(0);
    }

    fn take_highest_5_cards(cards: &[Card]) -> [Card; 5] {
        return cards.iter().cloned().rev().take(5).collect::<[Card; 5]>();
    }

    pub fn is_straight_flush(cards: &mut [Card]) -> Option<Self> {
        // search a major shape of the cards.

        let opt_major_shape: Option<_> = Self::get_major_shape(&cards, 5);
        if opt_major_shape.is_none() {
            return None;
        }
        let major_shape = opt_major_shape.unwrap();

        // filter the cards by the major shape.
        let mut shape_filtered_cards: Vec<_> = Self::filter_by_shape(&cards, major_shape);
        // reuse a code for checking if it is straight.
        match Self::is_straight(&mut shape_filtered_cards) {
            Some(Self::Straight(same_shaped_straight_cards)) => {
                Some(Self::StraightFlush(same_shaped_straight_cards))
            }
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
                cards.sort_by_key(|&card| {
                    if card.num == major_num {
                        13 + card.num
                    } else {
                        card.num
                    }
                });
                let made_cards = Self::take_highest_5_cards(&cards);
                return Some(Self::FourOfAKind(made_cards));
            }
            None => None,
        } // end of match
    } // end of is_four_of_a_kind

    fn is_full_house(cards: &mut [Card]) -> Option<Self> {
        // search major nums of the cards.
        //let triplets : Option<_> = Self::get_major_num(&cards, 3);
        let num_cnts = Self::num_counts(&cards);
        // (3, 2, 1, 1) or (3, 2, 2) or (3, 3, 1) => full house
        let triplets = num_cnts
            .iter()
            .cloned()
            .enumerate()
            .filter(|&(_num, num_cnt)| num_cnt == 3)
            .map(|(num, _num_cnt)| num)
            .take(2)
            .collect::<Vec<usize>>();
        if triplets.len() == 0 {
            return None;
        }
        let twins = num_cnts
            .iter()
            .cloned()
            .enumerate()
            .filter(|&(_num, num_cnt)| num_cnt == 2)
            .take(2)
            .map(|(num, _num_cnt)| num)
            .collect::<Vec<usize>>();
        if triplets.len() + twins.len() <= 1 {
            return None;
        }

        cards.sort_by_key(|&card| {
            if triplets.contains(&card.num) {
                card.num + 26
            } else if twins.contains(&card.num) {
                card.num + 13
            } else {
                card.num
            }
        });
        let made_cards = Self::take_highest_5_cards(&cards);
        return Some(Self::FullHouse(made_cards));
    }

    fn is_flush(cards: &mut [Card]) -> Option<Self> {
        // grade : 5
        let opt_major_shape: Option<usize> = Self::get_major_shape(cards, 5);
        match opt_major_shape {
            Some(major_shape) => {
                cards.sort_by_key(|&card| {
                    if card.shape == major_shape {
                        card.num + 13
                    } else {
                        card.num
                    }
                });
                let made_cards = Self::take_highest_5_cards(&cards);
                Some(Self::Flush(made_cards))
            }
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
                        cards.sort_by_key(|&card| {
                            if card.num >= num && card.num <= num + 4 {
                                card.num + 20
                            } else {
                                card.num
                            }
                        });
                        let made_cards = Self::take_highest_5_cards(&cards);
                        return Some(Self::Straight(made_cards));
                    }
                    4 if num == 0 && ace_exist => {
                        cards.sort_by_key(|&card| {
                            if card.num == 12 {
                                card.num + 20 - 13
                            } else if card.num <= 3 {
                                card.num + 20
                            } else {
                                card.num
                            }
                        });

                        let made_cards = Self::take_highest_5_cards(&cards);
                        return Some(Self::Straight(made_cards));
                    } //Some(Self::Straight((3, 2, 1, 0, 0))),
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
                cards.sort_by_key(|&card| {
                    if card.num == num {
                        card.num + 13
                    } else {
                        card.num
                    }
                });

                let made_cards = Self::take_highest_5_cards(&cards);
                return Some(Self::ThreeOfAKind(made_cards));
            }
            None => {
                return None;
            }
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
            .map(|(num, _num_cnt)| num)
            .collect::<Vec<usize>>();
        let num_pairs_len = num_pairs.len();
        match num_pairs_len {
            0..=1 => {
                return None;
            }
            2..=3 => {
                cards.sort_by_key(|&card| {
                    if num_pairs.contains(&card.num) {
                        card.num + 13
                    } else {
                        card.num
                    }
                });
                let made_cards = Self::take_highest_5_cards(&cards);
                return Some(Self::TwoPairs(made_cards));
            }
            _ => {
                return None;
            }
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
            None => {
                return None;
            }
            Some(num) => {
                cards.sort_by_key(|&card| {
                    if card.num == num {
                        card.num + 13
                    } else {
                        card.num
                    }
                });
                let made_cards = Self::take_highest_5_cards(&cards);
                return Some(Self::OnePair(made_cards));
            }
        }
    }
}

#[cfg(test)]
mod test_for_grade {
    use super::*;
    use crate::table::*;
    use std::collections::HashSet;

    #[test]
    fn test_straight_flush() {
        let mut deck = Deck::new();
        deck.deal_cards(13);
        deck.deal_cards(13);
        deck.deal_cards(13);
        deck.deal_cards(5);
        let mut cards: Vec<Card> = deck.deal_cards(7);
        let sf = Grade::is_straight_flush(&mut cards);
        assert!(sf.is_some());
        let sf = sf.unwrap();
        if let Grade::StraightFlush(cards) = sf {
            assert_eq!(
                HashSet::from(cards),
                HashSet::from([
                    Card { shape: 3, num: 11 },
                    Card { shape: 3, num: 10 },
                    Card { shape: 3, num: 9 },
                    Card { shape: 3, num: 8 },
                    Card { shape: 3, num: 7 }
                ])
            );
        }
    }

    #[test]
    fn test_four_of_a_kind() {
        let mut deck = Deck::new();
        let mut cards: Vec<Card> = Vec::new();
        cards.append(&mut deck.deal_cards(3));
        deck.deal_cards(9);
        cards.append(&mut deck.deal_cards(1));
        deck.deal_cards(12);
        cards.append(&mut deck.deal_cards(1));
        deck.deal_cards(12);
        cards.append(&mut deck.deal_cards(1));
        deck.deal_cards(12);
        cards.append(&mut deck.deal_cards(1));

        let fk = Grade::is_four_of_a_kind(&mut cards);
        assert!(fk.is_some());
        let fk = fk.unwrap();
        if let Grade::FourOfAKind(cards) = fk {
            assert_eq!(
                HashSet::from(cards),
                HashSet::from([
                    Card { shape: 0, num: 12 },
                    Card { shape: 1, num: 12 },
                    Card { shape: 2, num: 12 },
                    Card { shape: 3, num: 12 },
                    Card { shape: 0, num: 2 }
                ])
            );
        }
    }

    #[test]
    fn test_full_house() {
        let mut cards: Vec<Card> = vec![
            Card { shape: 0, num: 9 },
            Card { shape: 1, num: 9 },
            Card { shape: 2, num: 5 },
            Card { shape: 3, num: 9 },
            Card { shape: 2, num: 8 },
            Card { shape: 1, num: 5 },
            Card { shape: 0, num: 5 },
        ];
        let fh = Grade::is_full_house(&mut cards);
        assert!(fh.is_some());
        let fh = fh.unwrap();
        if let Grade::FullHouse(cards) = fh {
            assert_eq!(
                HashSet::from(cards),
                HashSet::from([
                    Card { shape: 0, num: 9 },
                    Card { shape: 1, num: 9 },
                    Card { shape: 3, num: 9 },
                    Card { shape: 0, num: 5 },
                    Card { shape: 1, num: 5 }
                ])
            );
        }
    }

    #[test]
    fn test_flush() {
        let mut cards: Vec<Card> = vec![
            Card { shape: 3, num: 1 },
            Card { shape: 1, num: 2 },
            Card { shape: 2, num: 3 },
            Card { shape: 3, num: 4 },
            Card { shape: 3, num: 6 },
            Card { shape: 3, num: 9 },
            Card { shape: 3, num: 12 },
        ];
        let fl = Grade::is_flush(&mut cards);
        assert!(fl.is_some());
        let fl = fl.unwrap();
        if let Grade::Flush(cards) = fl {
            assert_eq!(
                HashSet::from(cards),
                HashSet::from([
                    Card { shape: 3, num: 1 },
                    Card { shape: 3, num: 4 },
                    Card { shape: 3, num: 6 },
                    Card { shape: 3, num: 9 },
                    Card { shape: 3, num: 12 }
                ])
            );
        }
    }

    #[test]
    fn test_straight() {
        let mut cards: Vec<Card> = vec![
            Card { shape: 3, num: 1 },
            Card { shape: 1, num: 10 },
            Card { shape: 2, num: 8 },
            Card { shape: 3, num: 4 },
            Card { shape: 3, num: 7 },
            Card { shape: 3, num: 9 },
            Card { shape: 3, num: 11 },
        ];
        let st = Grade::is_straight(&mut cards);
        assert!(st.is_some());
        if let Some(Grade::Straight(cards)) = st {
            assert_eq!(
                HashSet::from(cards),
                HashSet::from([
                    Card { shape: 1, num: 10 },
                    Card { shape: 3, num: 7 },
                    Card { shape: 2, num: 8 },
                    Card { shape: 3, num: 9 },
                    Card { shape: 3, num: 11 }
                ])
            );
        }
    }

    #[test]
    fn test_three_of_a_kind() {
        let mut cards: Vec<Card> = vec![
            Card { shape: 0, num: 3 },
            Card { shape: 0, num: 11 },
            Card { shape: 1, num: 2 },
            Card { shape: 2, num: 12 },
            Card { shape: 3, num: 3 },
            Card { shape: 2, num: 3 },
            Card { shape: 0, num: 10 },
        ];
        let tk = Grade::is_three_of_a_kind(&mut cards);
        assert!(tk.is_some());
        if let Some(Grade::ThreeOfAKind(cards)) = tk {
            assert_eq!(
                HashSet::from(cards),
                HashSet::from([
                    Card { shape: 0, num: 3 },
                    Card { shape: 2, num: 3 },
                    Card { shape: 3, num: 3 },
                    Card { shape: 2, num: 12 },
                    Card { shape: 0, num: 11 }
                ])
            );
        }
    }

    #[test]
    fn test_two_pairs() {
        let mut cards: Vec<Card> = vec![
            Card { shape: 0, num: 1 },
            Card { shape: 0, num: 12 },
            Card { shape: 1, num: 7 },
            Card { shape: 1, num: 12 },
            Card { shape: 2, num: 7 },
            Card { shape: 2, num: 10 },
            Card { shape: 3, num: 10 },
        ];
        let tp = Grade::is_two_pairs(&mut cards);
        assert!(tp.is_some());
        if let Some(Grade::TwoPairs(cards)) = tp {
            assert_eq!(
                HashSet::from(cards),
                HashSet::from([
                    Card { shape: 3, num: 10 },
                    Card { shape: 2, num: 10 },
                    Card { shape: 2, num: 7 },
                    Card { shape: 0, num: 12 },
                    Card { shape: 1, num: 12 }
                ])
            );
        }
    }
    #[test]
    fn test_one_pair() {
        let mut cards: Vec<Card> = vec![
            Card { shape: 0, num: 1 },
            Card { shape: 0, num: 11 },
            Card { shape: 1, num: 7 },
            Card { shape: 1, num: 12 },
            Card { shape: 2, num: 7 },
            Card { shape: 2, num: 2 },
            Card { shape: 3, num: 10 },
        ];
        let op = Grade::is_one_pair(&mut cards);
        assert!(op.is_some());
        if let Some(Grade::OnePair(cards)) = op {
            assert_eq!(
                HashSet::from(cards),
                HashSet::from([
                    Card { shape: 1, num: 12 },
                    Card { shape: 0, num: 11 },
                    Card { shape: 2, num: 7 },
                    Card { shape: 1, num: 7 },
                    Card { shape: 3, num: 10 }
                ])
            );
        }
    }

    #[test]
    fn test_top() {
        let mut cards: Vec<Card> = vec![
            Card { shape: 0, num: 1 },
            Card { shape: 0, num: 3 },
            Card { shape: 1, num: 5 },
            Card { shape: 1, num: 7 },
            Card { shape: 2, num: 8 },
            Card { shape: 2, num: 10 },
            Card { shape: 3, num: 11 },
        ];
        let top = Grade::new(&mut cards);
        match top {
            Grade::Top(cards) => assert_eq!(
                HashSet::from(cards),
                HashSet::from([
                    Card { shape: 3, num: 11 },
                    Card { shape: 2, num: 10 },
                    Card { shape: 2, num: 8 },
                    Card { shape: 1, num: 7 },
                    Card { shape: 1, num: 5 }
                ])
            ),
            _ => panic!("cards do not constitute top."),
        }
    }
}
