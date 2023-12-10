use std::{cmp::Ordering, collections::HashMap, str::FromStr};

use super::rank::Rank;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Hand {
    card_1: Rank,
    card_2: Rank,
    card_3: Rank,
    card_4: Rank,
    card_5: Rank,
}

impl Hand {
    pub fn new(
        card_1: Rank,
        card_2: Rank,
        card_3: Rank,
        card_4: Rank,
        card_5: Rank,
    ) -> Self {
        return Hand {
            card_1,
            card_2,
            card_3,
            card_4,
            card_5,
        };
    }

    pub fn get_type(&self) -> HandType {
        let counts = self.get_counts();

        if is_one_count_of(&counts, 5) {
            return HandType::FiveOfAKind;
        } else if is_one_count_of(&counts, 4) {
            return HandType::FourOfAKind;
        } else if is_one_count_of(&counts, 3) && is_one_count_of(&counts, 2) {
            return HandType::FullHouse;
        } else if is_one_count_of(&counts, 3) {
            return HandType::ThreeOfAKind;
        } else if is_two_counts_of(&counts, 2) {
            return HandType::TwoPair;
        } else if is_one_count_of(&counts, 2) {
            return HandType::OnePair;
        } else {
            return HandType::HighCard;
        }
    }

    pub fn cmp_with_jokers(&self, other: &Self) -> Ordering {
        let t = self.get_type_with_jokers();
        let other_t = other.get_type_with_jokers();

        let ordering = get_first_difference(vec![
            t.cmp(&other_t),
            self.card_1.cmp_with_jokers(&other.card_1),
            self.card_2.cmp_with_jokers(&other.card_2),
            self.card_3.cmp_with_jokers(&other.card_3),
            self.card_4.cmp_with_jokers(&other.card_4),
            self.card_5.cmp_with_jokers(&other.card_5),
        ]);

        return ordering;
    }

    pub fn get_type_with_jokers(&self) -> HandType {
        let mut counts = self.get_counts();
        let num_jokers = count_jokers(*self);

        if num_jokers != 5 && num_jokers != 0 {
            let max_card = counts
                .iter()
                .filter(|(card, _)| **card != Rank::Jack)
                .max_by(|(_, count1), (_, count2)| count1.cmp(count2))
                .unwrap();

            let card = *max_card.0;
            counts.insert(card, max_card.1 + num_jokers);
            counts.remove(&Rank::Jack);
        }

        let number_counts = counts.len();
        let max_count = *counts.values().max().unwrap();

        if number_counts == 1 {
            return HandType::FiveOfAKind;
        }

        if number_counts == 2 {
            if max_count == 4 {
                return HandType::FourOfAKind;
            }
            if max_count == 3 {
                return HandType::FullHouse;
            }
        }

        if number_counts == 3 {
            if max_count == 2 {
                return HandType::TwoPair;
            }
            if max_count == 3 {
                return HandType::ThreeOfAKind;
            }
        }

        if number_counts == 4 {
            return HandType::OnePair;
        } else if number_counts == 5 {
            return HandType::HighCard;
        }

        panic!()
    }

    fn get_counts(&self) -> HashMap<Rank, u32> {
        let mut result = HashMap::<Rank, u32>::new();

        add_card_to_count(&mut result, self.card_1);
        add_card_to_count(&mut result, self.card_2);
        add_card_to_count(&mut result, self.card_3);
        add_card_to_count(&mut result, self.card_4);
        add_card_to_count(&mut result, self.card_5);

        return result;
    }
}

fn count_jokers(hand: Hand) -> u32 {
    let mut count = 0;

    if hand.card_1 == Rank::Jack {
        count += 1;
    }

    if hand.card_2 == Rank::Jack {
        count += 1;
    }

    if hand.card_3 == Rank::Jack {
        count += 1;
    }

    if hand.card_4 == Rank::Jack {
        count += 1;
    }

    if hand.card_5 == Rank::Jack {
        count += 1;
    }

    return count;
}

fn add_card_to_count(counts: &mut HashMap<Rank, u32>, card: Rank) {
    if counts.contains_key(&card) {
        counts.insert(card, counts[&card] + 1);
    } else {
        counts.insert(card, 1);
    }
}

fn is_one_count_of(counts: &HashMap<Rank, u32>, c: u32) -> bool {
    return get_num_of_cards_with_count(counts, c) >= 1;
}

fn is_two_counts_of(counts: &HashMap<Rank, u32>, c: u32) -> bool {
    return get_num_of_cards_with_count(counts, c) == 2;
}

fn get_num_of_cards_with_count(counts: &HashMap<Rank, u32>, c: u32) -> u32 {
    return counts.values().filter(|v| **v == c).count() as u32;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let ordering = get_first_difference(vec![
            self.get_type().cmp(&other.get_type()),
            self.card_1.cmp(&other.card_1),
            self.card_2.cmp(&other.card_2),
            self.card_3.cmp(&other.card_3),
            self.card_4.cmp(&other.card_4),
            self.card_5.cmp(&other.card_5),
        ]);

        return Some(ordering);
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let order = get_order(*self);
        let other_order = get_order(*other);

        return Some(order.cmp(&other_order));
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.partial_cmp(other).unwrap();
    }
}

fn get_order(hand_type: HandType) -> u32 {
    match hand_type {
        HandType::FiveOfAKind => 6,
        HandType::FourOfAKind => 5,
        HandType::FullHouse => 4,
        HandType::ThreeOfAKind => 3,
        HandType::TwoPair => 2,
        HandType::OnePair => 1,
        HandType::HighCard => 0,
    }
}

fn get_first_difference(orderings: Vec<Ordering>) -> Ordering {
    return *orderings
        .iter()
        .find(|o| **o != Ordering::Equal)
        .unwrap_or(&Ordering::Equal);
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.partial_cmp(other).unwrap();
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseHandError;

impl FromStr for Hand {
    type Err = ParseHandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = s
            .trim()
            .chars()
            .map(|c| Rank::from(c))
            .collect::<Vec<Rank>>();

        if cards.len() != 5 {
            return Err(ParseHandError);
        }

        return Ok(Hand::new(cards[0], cards[1], cards[2], cards[3], cards[4]));
    }
}
