use std::{collections::HashMap, fs};

#[derive(Clone, Debug)]
pub struct Octopus {
    energy_level: u32,
}

impl Octopus {
    pub fn new(energy_level: u32) -> Self {
        Octopus { energy_level }
    }

    /// inc returns True if an Octopus flashes.
    pub fn inc(&mut self) {
        self.energy_level += 1;
    }

    pub fn flashed(&self) -> bool {
        match self.energy_level {
            10 => true,
            _ => false,
        }
    }

    pub fn reset(&mut self) {
        if self.energy_level > 9 {
            self.energy_level = 0;
        }
    }
}

pub fn day11a() -> String {
    let data = fs::read_to_string("assets/day11.txt").expect("Could not load file");
    let mut map: HashMap<(i32, i32), Octopus> = read_input(&data);
    let (flashes, _synced_cycle) = flash_cycles(&mut map, 100);
    flashes.to_string()
}

pub fn flash(map: &mut HashMap<(i32, i32), Octopus>, x: i32, y: i32) -> i64 {
    let mut flashes = 1;
    for (dx, dy) in [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (1, 1),
        (-1, 1),
        (1, -1),
    ] {
        if let Some(octopus) = map.get_mut(&(x + dx, y + dy)) {
            // eprint!("flash:: {} at {},{} ", &octopus.energy_level, &x, &y);
            octopus.inc();
            // eprint!("increased to {} ", &octopus.energy_level);
            if octopus.flashed() {
                // eprintln!("and flashed ");
                flashes += flash(map, x + dx, y + dy);
            }
            // eprintln!()
        }
    }
    flashes
}

pub fn flash_cycles(map: &mut HashMap<(i32, i32), Octopus>, cycles: u32) -> (i64, Option<u32>) {
    let mut flashes: i64 = 0;
    let mut first_flash: Option<u32> = None;

    for cycle in 1..=cycles {
        for y in 0..10 {
            for x in 0..10 {
                if let Some(octopus) = map.get_mut(&(x, y)) {
                    // eprint!("flash_cycles:: {} at {},{} ", &octopus.energy_level, &x, &y);
                    octopus.inc();
                    // eprintln!("increased to {}", &octopus.energy_level);
                    if octopus.flashed() {
                        flashes += flash(map, x, y)
                    }
                }
            }
        }

        // Reset the octopuses that flashed.
        for y in 0..10 {
            for x in 0..10 {
                if let Some(octopus) = map.get_mut(&(x, y)) {
                    octopus.reset();
                }
            }
        }

        if map.iter().all(|(_, octopus)| octopus.energy_level == 0) {
            eprintln!("Synchronized flash during cycle #{}", cycle);
            if first_flash.is_none() {
                first_flash = Some(cycle);
                return (flashes, first_flash);
            }
        }
    }

    (flashes, first_flash)
}

pub fn day11b() -> String {
    let data = fs::read_to_string("assets/day11.txt").expect("Could not load file");
    let mut map: HashMap<(i32, i32), Octopus> = read_input(&data);
    let (_flashes, synced_cycle) = flash_cycles(&mut map, 1000);
    match synced_cycle {
        None => "No synced cycle".to_owned(),
        Some(c) => c.to_string(),
    }
}

pub fn read_input(input: &str) -> HashMap<(i32, i32), Octopus> {
    let mut map: HashMap<(i32, i32), Octopus> = HashMap::new();
    for (y, line) in input.trim().lines().enumerate() {
        for (x, value) in line.chars().enumerate() {
            map.insert(
                (x as i32, y as i32),
                Octopus::new(value.to_digit(10).unwrap()),
            );
        }
    }
    map
}

pub fn print_map(map: &mut HashMap<(i32, i32), Octopus>) {
    for y in 0..10 {
        for x in 0..10 {
            if let Some(octopus) = map.get_mut(&(x, y)) {
                eprint!("{}", octopus.energy_level)
            }
        }
        eprintln!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const RAW_INPUT: &'static str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
";

    #[test]
    fn test_day11_parta_sample() {
        let mut map: HashMap<(i32, i32), Octopus> = read_input(RAW_INPUT);
        let (flashes, _) = flash_cycles(&mut map, 100);
        assert_eq!(flashes, 1656);
    }

    #[test]
    fn test_day11_partb_sample() {
        let mut map: HashMap<(i32, i32), Octopus> = read_input(RAW_INPUT);
        let (_, synced_cycle) = flash_cycles(&mut map, 200);
        assert!(synced_cycle.is_some());
        assert_eq!(synced_cycle.unwrap(), 195);
    }

    #[test]
    fn test_day11_flashes() {
        let input: &str = "11111
19991
19191
19991
11111";
        let mut map: HashMap<(i32, i32), Octopus> = read_input(input);
        print_map(&mut map);
        let (flashes, _) = flash_cycles(&mut map, 1);
        print_map(&mut map);
        assert_eq!(flashes, 9);
    }
}
