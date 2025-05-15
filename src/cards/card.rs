// first 4 bits are the value
pub const ACE: u8 = 0b00000001;
pub const TWO: u8 = 0b00000010;
pub const THREE: u8 = 0b00000011;
pub const FOUR: u8 = 0b00000100;
pub const FIVE: u8 = 0b00000101;
pub const SIX: u8 = 0b00000110;
pub const SEVEN: u8 = 0b00000111;
pub const EIGHT: u8 = 0b00001000;
pub const NINE: u8 = 0b00001001;
pub const TEN: u8 = 0b00001010;
pub const JACK: u8 = 0b00001011;
pub const QUEEN: u8 = 0b00001100;
pub const KING: u8 = 0b00001101;

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
    c & 0b00001111
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
