use aoc2021::day01::{day01a, day01b};
use aoc2021::day02::{day02a, day02b};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let problem = args.get(1).map(|s| s.as_str()).unwrap_or("None");
    let result = match problem {
        "day01a" => day01a(),
        "day01b" => day01b(),
        "day02a" => day02a(),
        "day02b" => day02b(),

        _ => "We haven't solved that yet".to_string(),
    };
    println!("{}", result);
}
