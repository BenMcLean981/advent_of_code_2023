use crate::{day_14::platform::Platform, utils::file_utils::read_lines};

const NUM_CYCLES: usize = 1_000_000_000;

pub fn solve() {
    let filename = "src/day_14/input.txt";

    let binding = read_lines(filename);
    let lines = binding.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
    let platform = Platform::from_lines(lines);

    let north_load = platform.tilt_north().compute_north_load();
    let spun = spin(platform);
    let spun_load = spun.compute_north_load();

    println!("Day 14");
    println!("The load on the north beam is {north_load}.");
    println!("The load on the north beam is after {NUM_CYCLES} spin cycles is {spun_load}.");
}

fn spin(platform: Platform) -> Platform {
    let mut spins: Vec<Platform> = vec![platform];

    while find_cycle(&spins).is_none() {
        spins.push(spins.last().unwrap().spin());
    }

    let (offset, period) = find_cycle(&spins).unwrap();

    let index = (NUM_CYCLES - offset) % period + offset;

    return spins[index].clone();
}

/**
 * Gets the offset and period of the cycle.
 * Period does not count both ends, just one end of the cycle.
 */
fn find_cycle(spins: &Vec<Platform>) -> Option<(usize, usize)> {
    let last = spins.last().unwrap();

    let mut period: Option<usize> = None;

    for (i, other) in spins.iter().rev().skip(1).enumerate() {
        if other == last {
            period = Some(i);
            break;
        }
    }

    if period.is_none() {
        return None;
    }

    let period = period.unwrap();
    let offset = spins.len() - period - 1;

    return Some((offset, period));
}
