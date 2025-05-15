use crate::cards::card::cart_to_str;
use crate::cards::deck;

pub struct Hand {
    pub cards: Vec<u8>,
}

impl Hand {
    pub fn new() -> Hand {
        Hand { cards: Vec::new() }
    }

    pub fn hit(&mut self, deck: &mut deck::Deck, t: u8) {
        for _ in 0..t {
            self.cards.push(deck.cards[0]);
            deck.cards.remove(0);
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
