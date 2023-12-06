use advent_of_code_2023::day_6::race::Race;

#[test]
pub fn get_num_possible_wins_zero_with_no_solutions() {
    let race = Race::new(5, 8);

    assert_eq!(0, race.get_num_possible_wins())
}

#[test]
pub fn get_num_possible_wins_one_with_one_solutions() {
    let race = Race::new(6, 9);

    assert_eq!(1, race.get_num_possible_wins())
}

#[test]
pub fn get_num_possible_wins_returns_parabolic_number() {
    let race = Race::new(7, 9);

    assert_eq!(4, race.get_num_possible_wins())
}

#[test]
pub fn get_num_possible_wins_returns_parabolic_number_case_2() {
    let race = Race::new(30, 200);

    assert_eq!(9, race.get_num_possible_wins())
}
