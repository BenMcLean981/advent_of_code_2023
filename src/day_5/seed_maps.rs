use super::{map::Map, seed_data::SeedData};

pub struct SeedMaps {
    seed_to_soil: Map,
    soil_to_fertilizer: Map,
    fertilizer_to_water: Map,
    water_to_light: Map,
    light_to_temperature: Map,
    temperature_to_humidity: Map,
    humidity_to_location: Map,
}

impl SeedMaps {
    pub fn from_lines(lines: Vec<&str>) -> SeedMaps {
        let split = split_by_empty_line(lines);

        let seed_to_soil = Map::from_lines(&split[0]);
        let soil_to_fertilizer = Map::from_lines(&split[1]);
        let fertilizer_to_water = Map::from_lines(&split[2]);
        let water_to_light = Map::from_lines(&split[3]);
        let light_to_temperature = Map::from_lines(&split[4]);
        let temperature_to_humidity = Map::from_lines(&split[5]);
        let humidity_to_location = Map::from_lines(&split[6]);

        return SeedMaps {
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        };
    }

    pub fn get_data(&self, seed: u32) -> SeedData {
        let soil = self.seed_to_soil.map(seed);
        let fertilizer = self.soil_to_fertilizer.map(soil);
        let water = self.fertilizer_to_water.map(fertilizer);
        let light = self.water_to_light.map(water);
        let temperature = self.light_to_temperature.map(light);
        let humidity = self.temperature_to_humidity.map(temperature);
        let location = self.humidity_to_location.map(humidity);

        return SeedData {
            seed,
            soil,
            fertilizer,
            water,
            light,
            temperature,
            humidity,
            location,
        };
    }
}

fn split_by_empty_line(lines: Vec<&str>) -> Vec<Vec<&str>> {
    let mut results: Vec<Vec<&str>> = vec![];
    let mut next_result: Vec<&str> = vec![];

    lines.iter().for_each(|l| {
        if l.trim().is_empty() {
            results.push(next_result.clone());
            next_result = vec![];
        } else {
            next_result.push(*l);
        }
    });

    if !next_result.is_empty() {
        results.push(next_result);
    }

    return results;
}
