use crate::cards::card::*;
use rand::{rng, seq::SliceRandom};

const DECK_SIZE: usize = 52;

pub struct Deck {
    pub cards: [u8; DECK_SIZE],
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

impl std::fmt::Display for Deck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for card in &self.cards {
            let res = print_card(*card, f);
            if res.is_err() {
                return res;
            }
            let res = write!(f, "\n");
            if res.is_err() {
                return res;
            }
        }
        Ok(())
    }
}
