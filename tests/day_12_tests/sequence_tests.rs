use advent_of_code_2023::day_12::sequence::Sequence;

#[test]
pub fn is_partial_empty_returns_true() {
    let partial = Sequence::make_clear(vec![]);
    let sequence = Sequence::make_clear(vec![5, 4, 8]);

    assert!(partial.is_partial(&sequence));
}

#[test]
pub fn is_partial_larger_returns_false() {
    let partial = Sequence::make_clear(vec![5, 4, 8, 2]);
    let sequence = Sequence::make_clear(vec![5, 4, 8]);

    assert!(!partial.is_partial(&sequence));
}

#[test]
pub fn is_partial_partial_returns_true() {
    let partial = Sequence::make_clear(vec![5, 4]);
    let sequence = Sequence::make_clear(vec![5, 4, 8]);

    assert!(partial.is_partial(&sequence));
}

#[test]
pub fn is_partial_non_partial_returns_false() {
    let partial = Sequence::make_clear(vec![5, 5]);
    let sequence = Sequence::make_clear(vec![5, 4, 8]);

    assert!(!partial.is_partial(&sequence));
}

#[test]
pub fn is_partial_unclear_partial_returns_true() {
    let partial = Sequence::make_unclear(vec![5, 4]);
    let sequence = Sequence::make_clear(vec![5, 4, 8]);

    assert!(partial.is_partial(&sequence));
}

#[test]
pub fn is_partial_unclear_partial_too_small_returns_true() {
    // the 3 could be about to turn into a 4.
    let partial = Sequence::make_unclear(vec![5, 3]);
    let sequence = Sequence::make_clear(vec![5, 4, 8]);

    assert!(partial.is_partial(&sequence));
}
