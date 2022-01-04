use crate::Point;
use core::panic;
use std::cmp;
use std::collections::HashMap;
use std::fmt;
use std::fs;

#[derive(Debug, Clone, Copy)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    pub fn from_string(input: &str) -> Self {
        match input.split_once(" -> ") {
            Some((start, end)) => Line {
                start: Point::from_string(start),
                end: Point::from_string(end),
            },
            None => panic!("cannot parse the string representing the line: {}", input),
        }
    }

    /// Expands all the points of a line.
    ///
    /// Expands as:
    ///
    /// * a vertical entry like 1,1 -> 1,3 covers points 1,1, 1,2, and 1,3.
    /// * a horizontal entry like 9,7 -> 7,7 covers points 9,7, 8,7, and 7,7.
    /// * diagonal linea at exactly 45 degrees like
    ///     * an entry like 1,1 -> 3,3 covers points 1,1, 2,2, and 3,3.
    ///     * an entry like 9,7 -> 7,9 covers points 9,7, 8,8, and 7,9.
    pub fn expand(self, with_diagonals: bool) -> Vec<Point> {
        let mut expanded_lines: Vec<Point> = vec![];
        if self.start.x == self.end.x {
            expanded_lines.append(
                &mut (cmp::min(self.start.y, self.end.y)..=cmp::max(self.start.y, self.end.y))
                    .map(|i| Point {
                        x: self.start.x,
                        y: i,
                    })
                    .collect::<Vec<Point>>(),
            );
        } else if self.start.y == self.end.y {
            expanded_lines.append(
                &mut (cmp::min(self.start.x, self.end.x)..=cmp::max(self.start.x, self.end.x))
                    .map(|i| Point {
                        x: i,
                        y: self.start.y,
                    })
                    .collect::<Vec<Point>>(),
            );
        } else if with_diagonals {
            let dx = if self.end.x - self.start.x > 0 { 1 } else { -1 };
            let dy = if self.end.y - self.start.y > 0 { 1 } else { -1 };
            let mut x = self.start.x;
            let mut y = self.start.y;
            expanded_lines.push(Point { x, y });
            while x != self.end.x {
                x += dx;
                y += dy;
                expanded_lines.push(Point { x, y });
            }
            assert_eq!(
                y, self.end.y,
                "error while expanding line: {} ({:?})",
                self, expanded_lines
            );
        }
        expanded_lines
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{},{} -> {},{}",
            self.start.x, self.start.y, self.end.x, self.end.y
        )
    }
}

pub fn day05a() -> String {
    let values = fs::read_to_string("assets/day05.txt").expect("Could not load file");
    let lines = read_input(&values);
    let count = process_lines(&lines, false);
    count.to_string()
}

pub fn day05b() -> String {
    let values = fs::read_to_string("assets/day05.txt").expect("Could not load file");
    let lines = read_input(&values);
    let count = process_lines(&lines, true);
    count.to_string()
}

fn read_input(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|l| l.trim())
        .map(Line::from_string)
        .collect::<Vec<Line>>()
}

fn process_lines(lines: &[Line], with_diagonals: bool) -> i32 {
    let mut all_lines: HashMap<Point, i32> = HashMap::new();
    for line in lines {
        for point in line.expand(with_diagonals) {
            *all_lines.entry(point).or_insert(0) += 1;
        }
    }
    all_lines.iter().filter(|p| p.1 >= &2).map(|p| p.1).count() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    const RAW_INPUT: &'static str = "0,9 -> 5,9
    8,0 -> 0,8
    9,4 -> 3,4
    2,2 -> 2,1
    7,0 -> 7,4
    6,4 -> 2,0
    0,9 -> 2,9
    3,4 -> 1,4
    0,0 -> 8,8
    5,5 -> 8,2";

    #[test]
    fn test_day05_parta_sample() {
        let lines = read_input(RAW_INPUT);
        assert_eq!(process_lines(&lines, false), 5)
    }

    #[test]
    fn test_day05_partb_sample() {
        let lines = read_input(RAW_INPUT);
        assert_eq!(process_lines(&lines, true), 12)
    }

    // #[test]
    // fn test_line_expand() {
    //     let line00 = Line::from_string("1,1 -> 1,3");
    //     let line01 = Line::from_string("9,7 -> 7,7");
    //     assert_eq!(
    //         line00.expand(),
    //         vec![
    //             Point { x: 1, y: 1 },
    //             Point { x: 1, y: 2 },
    //             Point { x: 1, y: 3 },
    //         ]
    //     );
    //     assert_eq!(
    //         line01.expand(),
    //         vec![
    //             Point { x: 9, y: 7 },
    //             Point { x: 8, y: 7 },
    //             Point { x: 7, y: 7 },
    //         ]
    //     );
    // }
}
