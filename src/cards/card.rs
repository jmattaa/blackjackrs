// first 4 bits are the value
pub const ACE: u8 = 1;
pub const TWO: u8 = 2;
pub const THREE: u8 = 3;
pub const FOUR: u8 = 4;
pub const FIVE: u8 = 5;
pub const SIX: u8 = 6;
pub const SEVEN: u8 = 7;
pub const EIGHT: u8 = 8;
pub const NINE: u8 = 9;
pub const TEN: u8 = 10;
pub const JACK: u8 = 11;
pub const QUEEN: u8 = 12;
pub const KING: u8 = 13;

// last 3 bits are the suit
pub const CLUBS: u8 = 0b00100000;
pub const DIAMONDS: u8 = 0b01000000;
pub const HEARTS: u8 = 0b01100000;
pub const SPADES: u8 = 0b11100000;

pub const VALUES: [u8; 13] = [
    ACE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE, TEN, JACK, QUEEN, KING,
];
pub const SUITS: [u8; 4] = [CLUBS, DIAMONDS, HEARTS, SPADES];

pub fn suit(c: u8) -> u8 {
    c & 0b11100000
}
pub fn value(c: u8) -> u8 {
    // highest value is 10
    let v = c & 0b00001111;
    if v > 10 { 10 } else { v }
}

pub fn cart_to_str(c: u8) -> String {
    format!(
        "{}{}",
        match value(c) {
            ACE => "A",
            TWO => "2",
            THREE => "3",
            FOUR => "4",
            FIVE => "5",
            SIX => "6",
            SEVEN => "7",
            EIGHT => "8",
            NINE => "9",
            TEN => "10",
            JACK => "J",
            QUEEN => "Q",
            KING => "K",
            _ => "?",
        },
        match suit(c) {
            CLUBS => "♣",
            DIAMONDS => "♦",
            HEARTS => "♥",
            SPADES => "♠",
            _ => "?",
        },
    )
}
