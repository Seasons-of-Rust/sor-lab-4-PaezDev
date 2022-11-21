use core::time;
use std::thread;

use rand::Rng;

use crate::{card::Card, shop::Shop};

mod card;
mod shop;
mod strings;

pub enum FightResult {
    // TODO: Add variants for win, loss, tie, and draw
}

fn main() {
    println!("{}", strings::TITLE);

    println!("> Welcome to The Comic Book Shoppe's Realtime and Updated Shop Tender!\n");

    println!("> Now that we have two stores, we can test our fighting systems");
    println!("> For starters, let's watch some card battles!");

    let strong_card = Card {
        price: 10,
        health: 10,
        damage: 50,
    };

    let weak_card = Card {
        price: 10,
        health: 10,
        damage: 2,
    };

    let fight_result = strong_card.print_fight(&weak_card);
    assert!(matches!(fight_result, FightResult::Win));

    println!("> Next, what will happen when a card fights itself?");

    let fight_result = strong_card.print_fight(&strong_card);
    assert!(matches!(fight_result, FightResult::Tie));

    println!("> And does it work the other way?");

    let fight_result = weak_card.print_fight(&strong_card);
    assert!(matches!(fight_result, FightResult::Loss));

    println!("> Now, on to the store battle! 🦹‍♀️");

    // Create two vecs for the cards of each store
    let mut cards_1 = Vec::new();
    let mut cards_2 = Vec::new();

    // Set up randomness
    let mut rng = rand::thread_rng();

    // Add 5 cards to each store
    for _ in 0..5 {
        cards_1.push(Card {
            price: 0,
            health: rng.gen_range(1..10),
            damage: rng.gen_range(1..10),
        });

        cards_2.push(Card {
            price: 0,
            health: rng.gen_range(1..10),
            damage: rng.gen_range(1..10),
        });
    }

    // Set up the shops
    let shop1 = Shop { cards: cards_1 };
    let shop2 = Shop { cards: cards_2 };

    // Make them fight!
    let fight_result = shop1.fight_store(&shop2);

    // Print the results
    println!("> And the winner is... 🏆");
    thread::sleep(time::Duration::from_millis(300));
    println!("> ...");
    thread::sleep(time::Duration::from_millis(300));
    println!("> ...");
    thread::sleep(time::Duration::from_millis(300));

    match fight_result {
        FightResult::Win => println!("SHOP 1 WINS!"),
        FightResult::Loss => println!("SHOP 2 WINS!"),
        FightResult::Tie => println!("IT'S A TIE!"),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod test;
