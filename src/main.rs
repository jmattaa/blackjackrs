mod cards;
use cards::deck;

fn main() {
    let deck = deck::Deck::gen_deck();
    println!("{}", deck);
}
