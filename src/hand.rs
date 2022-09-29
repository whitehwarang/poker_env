use std::cmp;
use std::fmt;
use crate::deck::Card;

#[derive(fmt::Debug, Clone)]
pub struct Hand {
    cards_cnt: usize,
    pub cards: Vec<Card>,
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.cards)
    }
}

impl Hand {
    pub fn new() -> Self {
        Self {
            cards_cnt: 0,
	    cards: Vec::<Card>::new(),
        }
    }
    pub fn clone(&self) -> Self {
	let cards_cnt = self.cards_cnt;
	let cards = self.cards.clone();
	Self {cards_cnt, cards}
    }

    pub fn add_card(&mut self, card: Card) -> () {
        if self.cards_cnt < 7 {
	    self.cards.push(card);
	    self.cards_cnt += 1;
	}
    }

    pub fn add_cards(&mut self, cards: &[Card]) -> () {
        //    if self.cards_cnt < 7 {
        cards.into_iter().for_each(|card| {
            if self.cards_cnt < 7 {
                self.cards.push(*card);
                self.cards_cnt += 1;
            }
        })
    }
    pub fn len(&self) -> usize {
        self.cards_cnt
    }
}

#[cfg(test)]
mod test_for_deck {
    use crate::deck::*;
    use crate::hand::*;
    #[test]
    fn hand_get_cards_from_deck() {
        let mut deck = Deck::new();
        let mut p1 = Hand::new();
        p1.add_cards(&deck.deal_cards(7));
        assert_eq!(p1.len(), 7);
        assert_eq!(p1.cards[6], Card{shape:0, num:6});
    }
}
