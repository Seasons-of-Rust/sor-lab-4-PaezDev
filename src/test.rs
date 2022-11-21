use crate::{Card, FightResult, Shop};

#[test]
fn test_can_create_card() {
    // Create a card
    let card = Card {
        price: 10,
        health: 10,
        damage: 10,
    };
    assert_eq!(card.price, 10);
}

#[test]
fn test_can_create_shop() {
    let shop = Shop {
        cards: vec![
            Card {
                price: 10,
                health: 10,
                damage: 10,
            },
            Card {
                price: 10,
                health: 10,
                damage: 10,
            },
            Card {
                price: 10,
                health: 10,
                damage: 10,
            },
        ],
    };
    assert_eq!(shop.most_expensive(), 10);
}

#[test]
fn test_can_fight_tie() {
    let card1 = Card {
        price: 10,
        health: 10,
        damage: 10,
    };

    let card2 = Card {
        price: 10,
        health: 10,
        damage: 10,
    };

    assert!(matches!(card1.fight(&card2), FightResult::Tie));
}

#[test]
fn test_can_fight_draw() {
    let card1 = Card {
        price: 10,
        health: 10,
        damage: 5,
    };

    let card2 = Card {
        price: 10,
        health: 10,
        damage: 2,
    };

    assert!(matches!(card1.fight(&card2), FightResult::Draw));
}

#[test]
fn test_this_card_can_fight_win() {
    let card1 = Card {
        price: 10,
        health: 10,
        damage: 50,
    };

    let card2 = Card {
        price: 10,
        health: 10,
        damage: 2,
    };

    assert!(matches!(card1.fight(&card2), FightResult::Win));
}

#[test]
fn test_other_card_can_fight_win() {
    let card1 = Card {
        price: 10,
        health: 10,
        damage: 2,
    };

    let card2 = Card {
        price: 10,
        health: 10,
        damage: 50,
    };

    assert!(matches!(card1.fight(&card2), FightResult::Loss));
}

#[test]
fn test_most_expensive_card() {
    let shop = Shop {
        cards: vec![
            Card {
                price: 12314,
                health: 0,
                damage: 0,
            },
            Card {
                price: 1241,
                health: 0,
                damage: 0,
            },
            Card {
                price: 312340,
                health: 0,
                damage: 0,
            },
        ],
    };
    assert_eq!(shop.most_expensive(), 312340);
}

#[test]
fn test_total_damage() {
    let shop = Shop {
        cards: vec![
            Card {
                price: 0,
                health: 0,
                damage: 12,
            },
            Card {
                price: 0,
                health: 0,
                damage: 52,
            },
            Card {
                price: 0,
                health: 0,
                damage: 21,
            },
        ],
    };
    assert_eq!(shop.total_damage(), 85);
}

#[test]
fn test_total_health() {
    let shop = Shop {
        cards: vec![
            Card {
                price: 0,
                health: 12,
                damage: 0,
            },
            Card {
                price: 0,
                health: 52,
                damage: 0,
            },
            Card {
                price: 0,
                health: 21,
                damage: 0,
            },
        ],
    };
    assert_eq!(shop.total_health(), 85);
}

#[test]
fn test_shops_can_fight() {
    let shop1 = Shop { cards: vec![] };

    let shop2 = Shop { cards: vec![] };

    assert!(matches!(shop1.fight_store(&shop2), FightResult::Tie));
}

#[test]
fn test_shop_can_win_fight() {
    let shop1 = Shop {
        cards: vec![
            Card {
                price: 10,
                health: 100,
                damage: 50,
            },
            Card {
                price: 10,
                health: 100,
                damage: 2,
            },
        ],
    };

    let shop2 = Shop {
        cards: vec![
            Card {
                price: 10,
                health: 10,
                damage: 2,
            },
            Card {
                price: 10,
                health: 10,
                damage: 50,
            },
        ],
    };

    assert!(matches!(shop1.fight_store(&shop2), FightResult::Win));
}

#[test]
fn test_shop_can_lose_fight() {
    let shop1 = Shop {
        cards: vec![
            Card {
                price: 10,
                health: 1,
                damage: 50,
            },
            Card {
                price: 10,
                health: 1,
                damage: 2,
            },
        ],
    };

    let shop2 = Shop {
        cards: vec![
            Card {
                price: 10,
                health: 10,
                damage: 2,
            },
            Card {
                price: 10,
                health: 10,
                damage: 50,
            },
        ],
    };

    assert!(matches!(shop1.fight_store(&shop2), FightResult::Loss));
}

#[test]
fn test_shops_can_tie() {
    let shop1 = Shop {
        cards: vec![
            Card {
                price: 10,
                health: 1,
                damage: 50,
            },
            Card {
                price: 10,
                health: 1,
                damage: 2,
            },
        ],
    };

    let shop2 = Shop {
        cards: vec![
            Card {
                price: 10,
                health: 1,
                damage: 2,
            },
            Card {
                price: 10,
                health: 1,
                damage: 50,
            },
        ],
    };

    assert!(matches!(shop1.fight_store(&shop2), FightResult::Tie));
}

#[test]
fn test_large_battle() {
    let mut cards_1 = Vec::new();
    let mut cards_2 = Vec::new();

    for i in 0..1_000 {
        cards_1.push(Card {
            price: 10,
            health: i % 8,
            damage: i % 10,
        });

        cards_2.push(Card {
            price: 10,
            health: 10 % 6,
            damage: 10 % 11,
        });
    }

    let shop1 = Shop { cards: cards_1 };
    let shop2 = Shop { cards: cards_2 };

    assert!(matches!(shop1.fight_store(&shop2), FightResult::Loss));
}
