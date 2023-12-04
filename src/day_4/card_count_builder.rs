use std::collections::HashMap;

use super::card::Card;

// processed is not needed for AOC 2023 but this seems like good
// software design considering this process is meant to be one way.
pub struct CardCountBuilder {
    cards: HashMap<u32, Card>,
    counted: bool,
}

impl CardCountBuilder {
    pub fn new(cards: Vec<Card>) -> Self {
        let mut hash_map = HashMap::new();

        cards.iter().for_each(|c| {
            hash_map.insert(c.card_num, c.clone());
        });

        return CardCountBuilder {
            cards: hash_map,
            counted: false,
        };
    }

    pub fn get_count(&self) -> Result<u32, &str> {
        if !self.counted {
            return Err("Not counted, call .build() first.");
        }

        return Ok(self.cards.values().map(|c| c.quantity).sum());
    }

    pub fn build(&mut self) {
        self.get_ordered_keys().iter().for_each(|k| {
            let card = self.cards.get(k).unwrap();
            let num_wins = card.get_num_winning_numbers();
            let num_new_cards = card.quantity;

            self.add_cards(k + 1, num_wins, num_new_cards)
        });

        self.counted = true;
    }

    fn add_cards(&mut self, from: u32, num: u32, quantity: u32) {
        for k in from..from + num {
            self.increment(k, quantity);
        }
    }

    fn increment(&mut self, card_num: u32, quantity: u32) {
        let card = self.cards.get(&card_num);

        if let Some(card) = card {
            self.cards.insert(card_num, card.add(quantity));
        }

        // otherwise there is no card and we are done.
        // this can happen at end of list.
    }

    fn get_ordered_keys(&self) -> Vec<u32> {
        let mut keys = self.cards.keys().copied().collect::<Vec<u32>>();
        keys.sort();

        return keys;
    }
}
