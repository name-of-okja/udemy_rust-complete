use rand;
use rand::seq::SliceRandom;

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        let suits = ["hearts", "diamonds", "clubs", "spades"];
        let values = ["Ace", "two", "three"];

        let mut cards = vec![];
        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        Deck { cards }
    }

    fn shuffle(&mut self) {
        let mut rng = rand::rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_cards)
    }
}

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();

    let cards = deck.deal(10);
    println!("Heres your hands: {:#?}", cards);
    println!("Heres your deck: {:#?}", deck);
}
