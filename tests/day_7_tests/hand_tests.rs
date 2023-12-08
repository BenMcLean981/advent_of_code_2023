use std::{cmp::Ordering, str::FromStr};

use advent_of_code_2023::day_7::{
    hand::{Hand, HandType},
    rank::Rank,
};

#[test]
pub fn from_str_makes_hand() {
    let actual = Hand::from_str("32T3K").unwrap();
    let expected =
        Hand::new(Rank::Three, Rank::Two, Rank::Ten, Rank::Three, Rank::King);

    assert_eq!(expected, actual);
}

#[test]
pub fn get_type_five_of_a_kind() {
    let hand = Hand::new(Rank::Ace, Rank::Ace, Rank::Ace, Rank::Ace, Rank::Ace);

    assert_eq!(HandType::FiveOfAKind, hand.get_type());
}

#[test]
pub fn get_type_4_of_a_kind() {
    let hand =
        Hand::new(Rank::Ace, Rank::Ace, Rank::Queen, Rank::Ace, Rank::Ace);

    assert_eq!(HandType::FourOfAKind, hand.get_type());
}

#[test]
pub fn get_type_full_house() {
    let hand =
        Hand::new(Rank::Ace, Rank::Ace, Rank::Queen, Rank::Queen, Rank::Ace);

    assert_eq!(HandType::FullHouse, hand.get_type());
}

#[test]
pub fn get_type_3_of_a_kind() {
    let hand =
        Hand::new(Rank::Ace, Rank::Ace, Rank::Queen, Rank::Two, Rank::Ace);

    assert_eq!(HandType::ThreeOfAKind, hand.get_type());
}

#[test]
pub fn get_type_2_pair() {
    let hand =
        Hand::new(Rank::Ace, Rank::Ace, Rank::Queen, Rank::Two, Rank::Queen);

    assert_eq!(HandType::TwoPair, hand.get_type());
}

#[test]
pub fn get_type_pair() {
    let hand =
        Hand::new(Rank::Ace, Rank::Ace, Rank::Queen, Rank::Two, Rank::Three);

    assert_eq!(HandType::OnePair, hand.get_type());
}

#[test]
pub fn get_type_high_card() {
    let hand =
        Hand::new(Rank::Ace, Rank::Jack, Rank::Queen, Rank::Two, Rank::Three);

    assert_eq!(HandType::HighCard, hand.get_type());
}

#[test]
pub fn orders_by_hand_type() {
    let high_card =
        Hand::new(Rank::Ace, Rank::Jack, Rank::Queen, Rank::Two, Rank::Three);
    let one_pair =
        Hand::new(Rank::Ace, Rank::Ace, Rank::Queen, Rank::Two, Rank::Three);
    let two_pair =
        Hand::new(Rank::Ace, Rank::Ace, Rank::Queen, Rank::Two, Rank::Queen);
    let three_of_a_kind =
        Hand::new(Rank::Ace, Rank::Queen, Rank::Queen, Rank::Two, Rank::Queen);
    let full_house =
        Hand::new(Rank::Ace, Rank::Queen, Rank::Queen, Rank::Ace, Rank::Queen);
    let four_of_a_kind = Hand::new(
        Rank::Ace,
        Rank::Queen,
        Rank::Queen,
        Rank::Queen,
        Rank::Queen,
    );
    let five_of_a_kind = Hand::new(
        Rank::Queen,
        Rank::Queen,
        Rank::Queen,
        Rank::Queen,
        Rank::Queen,
    );

    let mut hands = vec![
        five_of_a_kind,
        four_of_a_kind,
        full_house,
        three_of_a_kind,
        two_pair,
        one_pair,
        high_card,
        high_card,
        one_pair,
        two_pair,
        three_of_a_kind,
        full_house,
        four_of_a_kind,
        five_of_a_kind,
    ];

    let expected = vec![
        high_card,
        high_card,
        one_pair,
        one_pair,
        two_pair,
        two_pair,
        three_of_a_kind,
        three_of_a_kind,
        full_house,
        full_house,
        four_of_a_kind,
        four_of_a_kind,
        five_of_a_kind,
        five_of_a_kind,
    ];

    hands.sort();

    assert_eq!(expected, hands);
}

#[test]
pub fn ord_same_type_goes_by_rank() {
    let hand_1 =
        Hand::new(Rank::Ace, Rank::Ace, Rank::Two, Rank::Two, Rank::Ace);
    let hand_2 =
        Hand::new(Rank::Ace, Rank::Ace, Rank::Three, Rank::Three, Rank::Ace);

    assert_eq!(Ordering::Less, hand_1.cmp(&hand_2));
}

#[test]
pub fn get_type_with_jokers_5_jokers_5_of_a_kind() {
    let hand =
        Hand::new(Rank::Jack, Rank::Jack, Rank::Jack, Rank::Jack, Rank::Jack);

    assert_eq!(HandType::FiveOfAKind, hand.get_type_with_jokers());
}

#[test]
pub fn get_type_with_jokers_4_jokers_5_of_a_kind() {
    let hand =
        Hand::new(Rank::Jack, Rank::Ace, Rank::Jack, Rank::Jack, Rank::Jack);

    assert_eq!(HandType::FiveOfAKind, hand.get_type_with_jokers());
}

#[test]
pub fn get_type_with_jokers_3_jokers_one_pair_5_of_a_kind() {
    let hand =
        Hand::new(Rank::Jack, Rank::Ace, Rank::Ace, Rank::Jack, Rank::Jack);

    assert_eq!(HandType::FiveOfAKind, hand.get_type_with_jokers());
}

#[test]
pub fn get_type_with_jokers_3_jokers_4_of_a_kind() {
    let hand =
        Hand::new(Rank::Jack, Rank::Ace, Rank::Two, Rank::Jack, Rank::Jack);

    assert_eq!(HandType::FourOfAKind, hand.get_type_with_jokers());
}

#[test]
pub fn get_type_with_jokers_2_jokers_3_dupes_5_of_a_kind() {
    let hand =
        Hand::new(Rank::Ace, Rank::Ace, Rank::Ace, Rank::Jack, Rank::Jack);

    assert_eq!(HandType::FiveOfAKind, hand.get_type_with_jokers());
}

#[test]
pub fn get_type_with_jokers_2_jokers_2_dupes_4_of_a_kind() {
    let hand =
        Hand::new(Rank::Ace, Rank::Ace, Rank::Two, Rank::Jack, Rank::Jack);

    assert_eq!(HandType::FourOfAKind, hand.get_type_with_jokers());
}

#[test]
pub fn get_type_with_jokers_2_jokers_no_dupes_3_of_a_kind() {
    let hand =
        Hand::new(Rank::Ace, Rank::Three, Rank::Two, Rank::Jack, Rank::Jack);

    assert_eq!(HandType::FourOfAKind, hand.get_type_with_jokers());
}

#[test]
pub fn get_type_with_jokers_1_jokers_4_dupes_5_of_a_kind() {
    let hand =
        Hand::new(Rank::Ace, Rank::Ace, Rank::Ace, Rank::Ace, Rank::Jack);

    assert_eq!(HandType::FiveOfAKind, hand.get_type_with_jokers());
}

#[test]
pub fn get_type_with_jokers_1_jokers_3_dupes_4_of_a_kind() {
    let hand =
        Hand::new(Rank::Ace, Rank::Ace, Rank::Ace, Rank::Two, Rank::Jack);

    assert_eq!(HandType::FourOfAKind, hand.get_type_with_jokers());
}

#[test]
pub fn get_type_with_jokers_1_jokers_2_pair_full_house() {
    let hand =
        Hand::new(Rank::Ace, Rank::Ace, Rank::Two, Rank::Two, Rank::Jack);

    assert_eq!(HandType::FullHouse, hand.get_type_with_jokers());
}

#[test]
pub fn get_type_with_jokers_1_jokers_2_dupes_three_of_a_kind() {
    let hand =
        Hand::new(Rank::Ace, Rank::Ace, Rank::Two, Rank::Three, Rank::Jack);

    assert_eq!(HandType::ThreeOfAKind, hand.get_type_with_jokers());
}

#[test]
pub fn get_type_with_jokers_1_jokers_no_dupes_one_pair() {
    let hand =
        Hand::new(Rank::Ace, Rank::Four, Rank::Two, Rank::Three, Rank::Jack);

    assert_eq!(HandType::OnePair, hand.get_type_with_jokers());
}
