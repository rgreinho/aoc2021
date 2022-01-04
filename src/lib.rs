use std::fmt;
use std::fs;
use std::path::Path;
use std::str::FromStr;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;

pub fn read_input<P, T>(input: P) -> Vec<T>
where
    P: AsRef<Path>,
    T: FromStr,
{
    let values = fs::read_to_string(input).expect("Could not load file");
    input_from_string::<T>(&values)
}

pub fn input_from_string<T>(input: &str) -> Vec<T>
where
    T: FromStr,
{
    input
        .lines()
        .filter_map(|s| s.parse::<T>().ok())
        .collect::<Vec<T>>()
}

pub fn read_input_sep<P, T>(input: P, separator: &str) -> Vec<T>
where
    P: AsRef<Path>,
    T: FromStr,
{
    let values = fs::read_to_string(input).expect("Could not load file");
    input_sep_from_string::<T>(&values, separator)
}

pub fn input_sep_from_string<T>(input: &str, separator: &str) -> Vec<T>
where
    T: FromStr,
{
    input
        .trim()
        .split(separator)
        .filter_map(|s| s.parse::<T>().ok())
        .collect::<Vec<T>>()
}

pub fn mean(list: &[i32]) -> i32 {
    list.iter().sum::<i32>() / (list.len() as i32)
}

pub fn median(list: &[i32]) -> i32 {
    let mut v = list.iter().copied().collect::<Vec<i32>>();
    v.sort();
    let len = v.len();
    let mid = len / 2;
    if len % 2 == 0 {
        return *v.get(mid + 1).unwrap();
    } else {
        return v[mid];
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn from_string(input: &str) -> Self {
        match input.split_once(',') {
            Some((x, y)) => Point {
                x: x.parse::<i32>().unwrap(),
                y: y.parse::<i32>().unwrap(),
            },
            None => panic!("cannot parse the string representing the point: {}", input),
        }
    }
}

impl From<&str> for Point {
    fn from(item: &str) -> Self {
        match item.split_once(',') {
            Some((x, y)) => Point {
                x: x.parse::<i32>().unwrap(),
                y: y.parse::<i32>().unwrap(),
            },
            None => panic!("cannot parse the string representing the point: {}", item),
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}
