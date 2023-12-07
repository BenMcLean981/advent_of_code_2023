use advent_of_code_2023::day_7::rank::Rank;

#[test]
pub fn from_char_makes_rank() {
    assert_eq!(Rank::Ace, Rank::from('A'));
    assert_eq!(Rank::King, Rank::from('K'));
    assert_eq!(Rank::Queen, Rank::from('Q'));
    assert_eq!(Rank::Jack, Rank::from('J'));
    assert_eq!(Rank::Ten, Rank::from('T'));
    assert_eq!(Rank::Nine, Rank::from('9'));
    assert_eq!(Rank::Eight, Rank::from('8'));
    assert_eq!(Rank::Seven, Rank::from('7'));
    assert_eq!(Rank::Six, Rank::from('6'));
    assert_eq!(Rank::Five, Rank::from('5'));
    assert_eq!(Rank::Four, Rank::from('4'));
    assert_eq!(Rank::Three, Rank::from('3'));
    assert_eq!(Rank::Two, Rank::from('2'));
}

#[test]
#[should_panic]
pub fn from_char_invalid_panics() {
    let _ = Rank::from('f');
}

#[test]
pub fn sort_puts_in_ascending_order() {
    let mut ranks = vec![
        Rank::Ace,
        Rank::King,
        Rank::Queen,
        Rank::Jack,
        Rank::Ten,
        Rank::Nine,
        Rank::Eight,
        Rank::Seven,
        Rank::Six,
        Rank::Five,
        Rank::Four,
        Rank::Three,
        Rank::Two,
        Rank::Two,
        Rank::Three,
        Rank::Four,
        Rank::Five,
        Rank::Six,
        Rank::Seven,
        Rank::Eight,
        Rank::Nine,
        Rank::Ten,
        Rank::Jack,
        Rank::Queen,
        Rank::King,
        Rank::Ace,
    ];

    let expected = vec![
        Rank::Two,
        Rank::Two,
        Rank::Three,
        Rank::Three,
        Rank::Four,
        Rank::Four,
        Rank::Five,
        Rank::Five,
        Rank::Six,
        Rank::Six,
        Rank::Seven,
        Rank::Seven,
        Rank::Eight,
        Rank::Eight,
        Rank::Nine,
        Rank::Nine,
        Rank::Ten,
        Rank::Ten,
        Rank::Jack,
        Rank::Jack,
        Rank::Queen,
        Rank::Queen,
        Rank::King,
        Rank::King,
        Rank::Ace,
        Rank::Ace,
    ];

    ranks.sort();

    assert_eq!(expected, ranks);
}
