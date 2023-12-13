use advent_of_code_2023::day_12::sequence::Sequence;

#[test]
pub fn is_partial_empty_returns_true() {
    let partial = Sequence::new(vec![]);
    let sequence = Sequence::new(vec![5, 4, 8]);

    assert!(partial.is_partial(&sequence));
}

#[test]
pub fn is_partial_larger_returns_false() {
    let partial = Sequence::new(vec![5, 4, 8, 2]);
    let sequence = Sequence::new(vec![5, 4, 8]);

    assert!(!partial.is_partial(&sequence));
}

#[test]
pub fn is_partial_partial_returns_true() {
    let partial = Sequence::new(vec![5, 4]);
    let sequence = Sequence::new(vec![5, 4, 8]);

    assert!(partial.is_partial(&sequence));
}

#[test]
pub fn is_partial_non_partial_returns_false() {
    let partial = Sequence::new(vec![5, 5]);
    let sequence = Sequence::new(vec![5, 4, 8]);

    assert!(!partial.is_partial(&sequence));
}
