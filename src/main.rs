mod cards;
use cards::deck;
use cards::hand;

fn main() {
    let mut deck = deck::Deck::gen_deck();
    let mut dealer = hand::Hand::new();
    let mut player = hand::Hand::new();

    // both start with one card
    dealer.hit(&mut deck, 2);
    player.hit(&mut deck, 2);

    println!("Dealer: {}", dealer.to_string());
    println!("Player: {}", player.to_string());
}
