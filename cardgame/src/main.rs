use rand::seq::SliceRandom;
use rand::{prelude::ThreadRng, thread_rng};
// you can do multiple imports the way it is handled above
#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    // these are methods or in this case, this is a method that is copies a static method run on the class itself
    // you can declare what you expect the function to return with a ->
    // you can declare what you want the binding to hold when it is being declared
    fn new() -> Self {
        let shapes: [&str; 4] = ["Spade", "Hearts", "Diamonds", "Clubs"];
        let values: [&str; 12] = [
            "Ace", "2", "3", "4", "5", "6", "7", "8", "9", "King", "Queen", "Jack   ",
        ];
        let mut cards: Vec<String> = Vec::new();
        for shape in shapes {
            for value in values {
                let card: String = format!("{} of {}", value, shape);
                cards.push(card);
            }
        }
        return Deck { cards };
    }

    // these are instance methods or methods that are run on instances of a class
    fn shuffle(&mut self) {
        let mut rng: ThreadRng = thread_rng();
        self.cards.shuffle(&mut rng);
    }
    fn deal(&mut self, mut no_of_cards: usize) -> Vec<String> {
        if no_of_cards > self.cards.len() {
            no_of_cards = self.cards.len();
            println!(
                "You have exceeded the max number of cards, dealing only {}  cards",
                no_of_cards
            );
        }
        return self.cards.split_off(self.cards.len() - no_of_cards);
    }
}

fn main() {
    //mut is used to declare that a binding can be changed
    let mut deck: Deck = Deck::new();
    deck.shuffle();

    // you can use the # to ensure it is formatted to your taste
    println!("You have a deck of cards: {:#?}", deck);
    // you can use the :# to print out struct without having to choose how it looks when printed out
    println!("You have been dealt a few cards {:#?}", deck.deal(4));
}
