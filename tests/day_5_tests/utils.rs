use advent_of_code_2023::day_5::range::Range;

pub fn have_same_items(s1: Vec<Range>, s2: Vec<Range>) -> bool {
    let sorted_1 = sort(s1);
    let sorted_2 = sort(s2);

    return sorted_1.eq(&sorted_2);
}

fn sort(vec: Vec<Range>) -> Vec<Range> {
    let mut result = vec.to_vec();

    result.sort();

    return result;
}
