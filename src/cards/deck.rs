use crate::cards::card::*;
use rand::{rng, seq::SliceRandom};

const DECK_SIZE: usize = 52;

pub struct Deck {
    pub cards: Vec<u8>
}

impl Deck {
    pub fn gen_deck() -> Deck {
        let mut cards: Vec<u8> = Vec::with_capacity(DECK_SIZE);

        for &suit in &SUITS {
            for &val in &VALUES {
                cards.push(val | suit);
            }
        }

        cards.shuffle(&mut rng());
        Deck {
            cards: cards.try_into().unwrap(),
        }
    }
}

impl ToString for Deck {
    fn to_string(&self) -> String {
        let mut res = String::new();
        for card in &self.cards {
            res.push_str(&cart_to_str(*card));
            res.push_str(" ");
        }
        res
    } 
}
