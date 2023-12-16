use advent_of_code_2023::day_15::hash::hash;

#[test]
pub fn hash_example_computes_hash() {
    assert_eq!(52, hash("HASH"));
}
