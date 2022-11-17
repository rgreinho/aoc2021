use crate::Point;
use regex::Regex;
use std::{collections::HashSet, fs};

#[derive(Debug, PartialEq)]
pub enum Axis {
    X,
    Y,
}

impl From<&str> for Axis {
    fn from(item: &str) -> Self {
        match item {
            "x" => Axis::X,
            "y" => Axis::Y,
            _ => panic!("Cannot parse axis: `{}`.", item),
        }
    }
}

#[derive(Debug)]
pub struct Fold {
    axis: Axis,
    value: i32,
}

impl From<&str> for Fold {
    fn from(item: &str) -> Self {
        let re = Regex::new(r"\s([xy])=(\d{0,5})$").unwrap();
        let caps = re.captures(item).unwrap();
        Fold {
            axis: Axis::from(&caps[1]),
            value: caps[2].parse::<i32>().unwrap(),
        }
    }
}

pub fn day13a() -> String {
    let data = fs::read_to_string("assets/day13.txt").expect("Could not load file");
    let (points, folds) = parse_input(&data);
    let point_count = fold_paper(&points, &folds, 1);
    point_count.to_string()
}

pub fn day13b() -> String {
    let data = fs::read_to_string("assets/day13.txt").expect("Could not load file");
    let (points, folds) = parse_input(&data);
    let point_count = fold_paper(&points, &folds, folds.len());
    point_count.to_string()
}

pub fn parse_input(input: &str) -> (Vec<Point>, Vec<Fold>) {
    let points = input
        .trim()
        .lines()
        .filter(|&l| !l.starts_with("fold"))
        .filter(|&l| !l.is_empty())
        .map(Point::from)
        .collect::<Vec<Point>>();
    let folds = input
        .lines()
        .filter(|&l| l.starts_with("fold"))
        .map(Fold::from)
        .collect::<Vec<Fold>>();
    (points, folds)
}

pub fn fold_paper(points: &Vec<Point>, folds: &Vec<Fold>, fold_count: usize) -> usize {
    let mut folded_points: Vec<Point> = points.iter().map(|&p| p).collect();
    for fold in folds.iter().take(fold_count) {
        // dbg!(&fold);
        for point in folded_points.iter_mut() {
            match fold.axis {
                Axis::X => {
                    if point.x > fold.value {
                        point.x = 2 * fold.value - point.x;
                    }
                }
                Axis::Y => {
                    if point.y > fold.value {
                        point.y = 2 * fold.value - point.y;
                    }
                }
            }
        }
        // dbg!(&folded_points);
    }
    let point_set = folded_points.iter().map(|&p| p).collect::<HashSet<Point>>();

    // Print the paper.
    let max_x = folded_points.iter().map(|p| p.x).max().unwrap();
    let max_y = folded_points.iter().map(|p| p.y).max().unwrap();
    for y in 0..=max_y {
        for x in 0..=max_x {
            if let Some(_p) = point_set.get(&Point { x, y }) {
                print!("#");
            } else {
                print!(".")
            }
        }
        println!();
    }

    // Return the nuumber of points.
    point_set.len()
}

#[cfg(test)]
mod test {
    use super::*;

    const RAW_INPUT: &'static str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
";

    #[test]
    fn test_day13_parta_sample() {
        let (points, folds) = parse_input(RAW_INPUT);
        assert_eq!(points.len(), 18);
        let point_count = fold_paper(&points, &folds, 2);
        assert_eq!(point_count, 16);
    }

    #[test]
    fn test_day13_partb_sample() {
        assert_eq!(0, 195);
    }
}
