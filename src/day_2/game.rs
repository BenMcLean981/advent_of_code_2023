use std::{collections::HashMap, str::FromStr};

use super::{cube::Cube, viewing::Viewing};

#[derive(Debug, PartialEq, Clone)]
pub struct Game {
    pub id: u32,
    pub totals: HashMap<Cube, u32>,
    pub viewings: Vec<Viewing>,
}

impl Game {
    pub fn new(
        id: u32,
        totals: &HashMap<Cube, u32>,
        viewings: &Vec<Viewing>,
    ) -> Self {
        return Game {
            id,
            totals: totals.clone(),
            viewings: viewings.clone(),
        };
    }

    pub fn is_possible(&self) -> bool {
        return self.viewings.iter().all(|v| self.is_viewing_possible(v));
    }

    fn is_viewing_possible(&self, v: &Viewing) -> bool {
        return v
            .counts
            .iter()
            .all(|(cube, count)| *count <= self.totals[cube]);
    }

    pub fn get_minimum_power(&self) -> u32 {
        return self
            .get_minimum_set()
            .iter()
            .map(|(_, c)| c)
            .fold(1, |prod, c| prod * c);
    }

    fn get_minimum_set(&self) -> HashMap<Cube, u32> {
        let cube_counts = self.get_counts_by_cube();

        let mut result = HashMap::<Cube, u32>::new();

        cube_counts.iter().for_each(|(cube, counts)| {
            result.insert(*cube, *counts.iter().max().unwrap());
        });

        return result;
    }

    fn get_counts_by_cube(&self) -> HashMap<Cube, Vec<u32>> {
        let mut result = HashMap::<Cube, Vec<u32>>::new();

        self.viewings.iter().for_each(|v| {
            v.counts.iter().for_each(|(cube, count)| {
                if !result.contains_key(cube) {
                    result.insert(*cube, vec![*count]);
                } else {
                    result.get_mut(cube).unwrap().push(*count);
                }
            });
        });

        return result;
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseGameError;

impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let game =
            Game::new(get_game_id(s), &get_default_totals(), &get_viewings(s));

        return Ok(game);
    }
}

fn get_game_id(s: &str) -> u32 {
    let game_str = s.split(":").nth(0).unwrap();
    let game_id_str = game_str.split(" ").nth(1).unwrap();

    return u32::from_str(game_id_str).unwrap();
}

pub fn get_default_totals() -> HashMap<Cube, u32> {
    let mut result = HashMap::<Cube, u32>::new();

    result.insert(Cube::Red, 12);
    result.insert(Cube::Green, 13);
    result.insert(Cube::Blue, 14);

    return result;
}

pub fn get_viewings(s: &str) -> Vec<Viewing> {
    let viewing_str = s.split(":").nth(1).unwrap().trim();

    if viewing_str.trim().len() == 0 {
        return Vec::<Viewing>::new();
    }

    return viewing_str
        .split(';')
        .map(Viewing::from_str)
        .map(|v| v.unwrap())
        .collect();
}
