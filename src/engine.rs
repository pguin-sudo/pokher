use rand::thread_rng;
use rand::seq::SliceRandom;
use std::fmt;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Clone, EnumIter)]
enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Clone, EnumIter)]
enum Suit {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
}

#[derive(Clone)]
struct Card {
    rank: Rank,
    suit: Suit,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rank_str = match self.rank {
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Ace => "A",
        };

        let suit_str = match self.suit {
            Suit::Spades => "♠",
            Suit::Hearts => "♥",
            Suit::Clubs => "♣",
            Suit::Diamonds => "♦",
        };

        write!(f, "{}{}", rank_str, suit_str)
    }
}

#[derive(Clone)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new(shuffled: bool) -> Deck {
        let mut cards = Vec::new();

        for rank in Rank::iter() {
            for suit in Suit::iter() {
                cards.push(Card { rank: rank.clone(), suit: suit.clone() });
            }
        }

        if shuffled {
            let mut rng = thread_rng();
            cards.shuffle(&mut rng);
        }

        Deck { cards }
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for card in &self.cards {
            write!(f, "{} ", card)?;
        }
        Ok(())
    }
}
