use advent_of_code_2023::day_1;

#[test]
fn get_calibration_value_returns_sum_when_first_and_last() {
    assert_eq!(12, day_1::solution::get_calibration_value("1abc2"));
}

#[test]
fn get_calibration_value_returns_sum_when_not_first_and_last() {
    assert_eq!(38, day_1::solution::get_calibration_value("pqr3stu8vwx"));
}

#[test]
fn get_calibration_value_returns_sum_when_many() {
    assert_eq!(15, day_1::solution::get_calibration_value("a1b2c3d4e5f"));
}

#[test]
fn get_calibration_value_returns_sum_when_one() {
    assert_eq!(77, day_1::solution::get_calibration_value("treb7uchet"));
}

#[test]
fn get_calibration_value_returns_first_spelt_digit() {
    assert_eq!(17, day_1::solution::get_calibration_value("trebone7uchet"));
}

#[test]
fn get_calibration_value_returns_last_spelt_digit() {
    assert_eq!(72, day_1::solution::get_calibration_value("treb7uctwohet"));
}

#[test]
fn get_calibration_value_returns_first_spelt_digit_with_overlap() {
    assert_eq!(87, day_1::solution::get_calibration_value("trebeightwo7uchet"));
}

#[test]
fn add_lines_sums_calibration_numbers_of_lines() {
    let lines = vec![
        "1abc2".to_string(),
        "pqr3stu8vwx".to_string(),
        "a1b2c3d4e5f".to_string(),
        "treb7uchet".to_string()
    ];

    assert_eq!(142, day_1::solution::add_lines(lines));
}