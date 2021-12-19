use core::panic;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Lanternfish {
    internal_timer: u8,
}

impl Lanternfish {
    pub fn new() -> Self {
        Lanternfish { internal_timer: 8 }
    }

    pub fn with_internal_timer(internal_timer: u8) -> Self {
        match internal_timer {
            0..=6 => Lanternfish { internal_timer },
            _ => panic!(
                "cannot create a Lanternfish with an internal timer value of `{}`",
                internal_timer
            ),
        }
    }

    pub fn next(&mut self) -> Option<Lanternfish> {
        match self.internal_timer {
            0 => {
                self.internal_timer = 6;
                return Some(Lanternfish::new());
            }
            1..=8 => {
                self.internal_timer -= 1;
                return None;
            }
            _ => panic!("invalid internal timer value: {}", self.internal_timer),
        }
    }
}

pub fn day06a() -> String {
    let values = fs::read_to_string("assets/day06.txt").expect("Could not load file");
    let fishes = read_input(&values);
    simulate_lanternfish(&fishes, 80).to_string()
}

pub fn day06b() -> String {
    let values = fs::read_to_string("assets/day06.txt").expect("Could not load file");
    let mut fishes = read_input(&values);

    // (fish, days left) -> produced fish
    let mut cache: HashMap<(Lanternfish, usize), usize> = HashMap::new();
    let mut count = 0;
    for fish in fishes.iter_mut() {
        count += memoized_simulate(&mut cache, fish, 256)
    }
    count.to_string()
    // simulate_lanternfish(&fishes, 256).to_string()
}

fn read_input(input: &str) -> Vec<Lanternfish> {
    input
        .split(",")
        .filter_map(|s| s.parse::<u8>().ok())
        .map(|f| Lanternfish::with_internal_timer(f))
        .collect::<Vec<Lanternfish>>()
}

fn simulate_lanternfish(input: &[Lanternfish], days: i32) -> usize {
    let mut fishes = input.iter().copied().collect::<Vec<Lanternfish>>();
    for _day in 0..days {
        let mut newborns: Vec<Lanternfish> = Vec::new();
        for fish in fishes.iter_mut() {
            if let Some(f) = fish.next() {
                newborns.push(f);
            }
        }
        fishes.append(&mut newborns);
    }
    fishes.len()
}

fn memoized_simulate(
    cache: &mut HashMap<(Lanternfish, usize), usize>,
    fish: &mut Lanternfish,
    days: usize,
) -> usize {
    if let Some(count) = cache.get(&(*fish, days)) {
        eprintln!("Cache hit: {:?} -> {}", &fish, &count);
        return *count;
    }

    let mut count: usize = 1;
    let cloned_fish = fish.clone();
    for day in 1..=days {
        if let Some(mut f) = fish.next() {
            count += memoized_simulate(cache, &mut f, days - day);
        }
    }
    cache.insert((cloned_fish, days), count);
    count
}

#[cfg(test)]
mod test {
    use super::*;

    const RAW_INPUT: &'static str = "3,4,3,1,2";

    #[test]
    fn test_day06_parta_sample() {
        let fishes = read_input(RAW_INPUT);
        dbg!(&fishes);
        assert_eq!(simulate_lanternfish(&fishes, 80), 5934)
    }

    #[test]
    fn test_day06_partb_sample() {
        let mut fishes = read_input(RAW_INPUT);
        let mut cache: HashMap<(Lanternfish, usize), usize> = HashMap::new();
        let mut count = 0;
        // for fish in fishes.iter_mut() {
        //     count += memoized_simulate(&mut cache, fish, 80)
        // }
        // assert_eq!(count, 5934);
        for fish in fishes.iter_mut() {
            dbg!(&fish);
            count += memoized_simulate(&mut cache, fish, 256)
        }
        assert_eq!(count, 26984457539)
    }

    #[test]
    fn test_day06_parta() {
        let values = fs::read_to_string("assets/day06.txt").expect("Could not load file");
        let fishes = read_input(&values);
        assert_eq!(simulate_lanternfish(&fishes, 80), 386755)
    }
}
