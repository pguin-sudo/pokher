mod engine;

use engine::Deck;

fn main() {
    let deck = Deck::new(true);
    println!("{:}", deck);
}
