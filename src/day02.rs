use core::panic;

use crate::read_input;
use std::cmp::PartialEq;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Forward,
    Down,
    Up,
}

#[derive(Debug, Clone, Copy)]
struct Command {
    direction: Direction,
    unit: u8,
}

impl Command {
    pub fn from_string(command: &str) -> Self {
        let parts: Vec<&str> = command.split(' ').collect();
        let direction = match parts[0] {
            "forward" => Direction::Forward,
            "up" => Direction::Up,
            "down" => Direction::Down,
            _ => panic!("unknown direction"),
        };
        let unit = parts[1].parse::<u8>().unwrap();
        Command { direction, unit }
    }
}

pub fn day02a() -> String {
    let data = read_input::<&str, String>("assets/day02.txt");
    let commands = data
        .iter()
        .map(|d| Command::from_string(d))
        .collect::<Vec<Command>>();
    let horizontal = commands
        .iter()
        .filter(|c| c.direction == Direction::Forward)
        .map(|c| c.unit as u32)
        .sum::<u32>();
    let depth = commands
        .iter()
        .filter(|c| c.direction != Direction::Forward)
        .map(|c| match c.direction {
            Direction::Up => -(c.unit as i32),
            Direction::Down => c.unit as i32,
            _ => panic!("invalid direction"),
        })
        .sum::<i32>();
    let planned_course = (horizontal as i32) * depth;
    planned_course.to_string()
}

pub fn day02b() -> String {
    let data = read_input::<&str, String>("assets/day02.txt");
    let commands = data
        .iter()
        .map(|d| Command::from_string(d))
        .collect::<Vec<Command>>();
    let horizontal_commands = commands
        .iter()
        .filter(|c| c.direction == Direction::Forward)
        .map(|&c| c.clone())
        .collect::<Vec<Command>>();
    let horizontal = horizontal_commands
        .iter()
        .map(|c| c.unit as u32)
        .sum::<u32>();
    let mut depth: i64 = 0;
    let mut aim: i64 = 0;
    for command in commands {
        match command.direction {
            Direction::Up => aim -= command.unit as i64,
            Direction::Down => aim += command.unit as i64,
            Direction::Forward => depth += aim * (command.unit as i64),
        }
    }
    let planned_course = (horizontal as i64) * depth;
    planned_course.to_string()
}
