use advent_of_code_2023::day_17::{
    dijkstra_solver::{DijsktraSolver, State},
    heat_map::HeatMap,
    path::Path,
    position::Position,
};

#[test]
pub fn pop_min_gets_initial_state() {
    let mut solver = make_solver();

    let actual = solver.pop_min();
    let expected = State {
        position: Position::new(0, 0),
        loss: 0,
        path: Path::from_positions(vec![Position::new(0, 0)]),
    };

    assert_eq!(expected, actual);
}

#[test]
#[should_panic]
pub fn pop_min_removes_initial_state() {
    let mut solver = make_solver();

    solver.pop_min();
    solver.pop_min();
}

fn make_solver() -> DijsktraSolver {
    return DijsktraSolver::new(make_heat_map());
}

fn make_heat_map() -> HeatMap {
    return HeatMap::from_lines(vec![
        "2413432311323",
        "3215453535623",
        "3255245654254",
        "3446585845452",
        "4546657867536",
        "1438598798454",
        "4457876987766",
        "3637877979653",
        "4654967986887",
        "4564679986453",
        "1224686865563",
        "2546548887735",
        "4322674655533",
    ]);
}
