use std::str::FromStr;

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
pub fn get_type_four_of_a_kind() {
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
pub fn get_type_three_of_a_kind() {
    let hand =
        Hand::new(Rank::Ace, Rank::Ace, Rank::Queen, Rank::Two, Rank::Ace);

    assert_eq!(HandType::ThreeOfAKind, hand.get_type());
}

#[test]
pub fn get_type_two_pair() {
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
