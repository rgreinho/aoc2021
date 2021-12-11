use std::fs;
use std::path::Path;
use std::str::FromStr;

pub mod day01;
pub mod day02;

pub fn read_input<P, T>(input: P) -> Vec<T>
where
    P: AsRef<Path>,
    T: FromStr,
{
    let values = fs::read_to_string(input).expect("Could not load file");
    values
        .lines()
        .filter_map(|s| s.parse::<T>().ok())
        .collect::<Vec<T>>()
}
