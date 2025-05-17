use crate::cards::card::{cart_to_str, value};
use crate::cards::deck;

pub struct Hand {
    pub cards: Vec<u8>,
}

impl Hand {
    pub fn new(deck: &mut deck::Deck) -> Hand {
        let mut hand = Hand { cards: Vec::new() };
        // a hand starts with 2 cards
        hand.hit(deck, 2);
        hand
    }

    // hits t amount of cards
    // if there are less than t cards in the deck, the deck is refilled
    pub fn hit(&mut self, deck: &mut deck::Deck, t: u8) {
        if deck.cards.len() < t as usize {
            *deck = deck::Deck::gen_deck();
        }

        for _ in 0..t {
            self.cards.push(deck.cards[0]);
            deck.cards.remove(0);
        }
    }

    // count the value of the hand
    pub fn count(&self) -> u8 {
        let mut res = 0;
        let mut got_ace = false;
        for card in &self.cards {
            res += value(*card);
            if value(*card) == 1 {
                got_ace = true;
            }
        }
        // check for ace and add if we want 11 or 1
        // we be already added 1 in the loop
        if got_ace && res + 11 < 21 {
            res += 10;
        }

        res
    }

    pub fn dealercount(&self, isplayerturn: bool) -> u8 {
        if isplayerturn {
            value(self.cards[0])
        } else {
            self.count()
        }
    }

    pub fn dealerstr(&self, isplayerturn: bool) -> String {
        if isplayerturn {
            format!("{} ??", cart_to_str(self.cards[0]))
        } else {
            self.to_string()
        }
    }
}

impl ToString for Hand {
    fn to_string(&self) -> String {
        let mut res = String::new();
        for card in &self.cards {
            res.push_str(&cart_to_str(*card));
            res.push_str(" ");
        }
        res
    }
}
