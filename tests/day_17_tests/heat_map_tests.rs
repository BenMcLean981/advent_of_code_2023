use advent_of_code_2023::day_17::{
    direction::Direction,
    heat_map::{Edge, HeatMap},
    path::Path,
};

#[test]
pub fn get_edges_top_left_edge_zero_moves_returns_two_options() {
    let heat_map = HeatMap::new(vec![
        vec![1, 2, 3, 4, 5],
        vec![6, 7, 8, 9, 1],
        vec![6, 5, 4, 3, 2],
    ]);

    let path = Path::new();

    let edges = heat_map.get_edges(&path);

    let expected = vec![
        Edge::new(Direction::Right, 2),
        Edge::new(Direction::Down, 6),
    ];

    assert!(have_same_items(expected, edges));
}

#[test]
pub fn get_edges_top_one_move_returns_three_options() {
    let heat_map = HeatMap::new(vec![
        vec![1, 2, 3, 4, 5],
        vec![6, 7, 8, 9, 1],
        vec![6, 5, 4, 3, 2],
    ]);

    let path = Path::new().move_by(Direction::Right);

    let edges = heat_map.get_edges(&path);

    let expected = vec![
        Edge::new(Direction::Right, 3),
        Edge::new(Direction::Down, 7),
    ];

    assert!(have_same_items(expected, edges));
}

#[test]
pub fn get_edges_top_two_moves_returns_three_options() {
    let heat_map = HeatMap::new(vec![
        vec![1, 2, 3, 4, 5],
        vec![6, 7, 8, 9, 1],
        vec![6, 5, 4, 3, 2],
    ]);

    let path = Path::new()
        .move_by(Direction::Right)
        .move_by(Direction::Right);

    let edges = heat_map.get_edges(&path);

    let expected = vec![
        Edge::new(Direction::Right, 4),
        Edge::new(Direction::Down, 8),
    ];

    assert!(have_same_items(expected, edges));
}

#[test]
pub fn get_edges_top_three_moves_returns_two_options() {
    let heat_map = HeatMap::new(vec![
        vec![1, 2, 3, 4, 5],
        vec![6, 7, 8, 9, 1],
        vec![6, 5, 4, 3, 2],
    ]);

    let path = Path::new()
        .move_by(Direction::Right)
        .move_by(Direction::Right)
        .move_by(Direction::Right);

    let edges = heat_map.get_edges(&path);

    let expected = vec![Edge::new(Direction::Down, 9)];

    assert!(have_same_items(expected, edges));
}

pub fn have_same_items(s1: Vec<Edge>, s2: Vec<Edge>) -> bool {
    let sorted_1 = sort(s1);
    let sorted_2 = sort(s2);

    return sorted_1.eq(&sorted_2);
}

fn sort(vec: Vec<Edge>) -> Vec<Edge> {
    let mut result = vec.to_vec();

    result.sort();

    return result;
}
