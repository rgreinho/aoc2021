// I used this puzzle more for learning this time.
//
// I got some inspiration from:
// * https://github.com/seamusriordan/advent-of-code-2021-day-12-rust/blob/part2/src/lib.rs
// * https://github.com/kamioftea/advent-of-code-2021/blob/main/src/day_12.rs
//
// Graph theory:
// * https://github.com/nrc/r4cppp/blob/master/graphs/README.md
// * https://depth-first.com/articles/2020/02/03/graphs-in-rust-an-introduction-to-petgraph/
//
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum CaveType {
    Start,
    End,
    Small,
    Large,
}

// https://doc.rust-lang.org/rust-by-example/conversion/from_into.html
impl From<&str> for CaveType {
    fn from(item: &str) -> Self {
        match item {
            "start" => CaveType::Start,
            "end" => CaveType::End,
            label if label.to_uppercase() == label => CaveType::Large,
            _ => CaveType::Small,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Cave {
    label: String,
    cave_type: CaveType,
    visited: bool,
    // connected_caves: HashSet<Cave>,
    edges: Vec<Rc<RefCell<Cave>>>,
}

impl From<&str> for Cave {
    fn from(item: &str) -> Self {
        Cave {
            label: item.into(),
            cave_type: CaveType::from(item),
            visited: false,
            edges: Vec::new(),
        }
    }
}

impl Cave {
    fn new(label: &str) -> Rc<RefCell<Cave>> {
        Rc::new(RefCell::new(Cave {
            label: label.into(),
            cave_type: CaveType::from(label),
            visited: false,
            edges: Vec::new(),
        }))
    }

    pub fn add(&mut self, cave: &Rc<RefCell<Cave>>) {
        self.edges.push(cave.clone());
    }

    pub fn can_visit(&self) -> bool {
        !(self.visited && self.cave_type == CaveType::Small)
    }

    fn traverse(&mut self) {
        if !self.can_visit() {
            return;
        }
        self.visited = true;
        // for n in &self.edges {
        //     n.borrow().traverse();
        // }
    }
}

pub fn parse_input(input: &str) -> HashMap<&str, Rc<RefCell<Cave>>> {
    let mut caves: HashMap<&str, Rc<RefCell<Cave>>> = HashMap::new();
    input
        .lines()
        .flat_map(|l| l.split_once("-"))
        .for_each(|(from, to)| {
            // Insert new caves.
            caves.entry(from).or_insert(Cave::new(from));
            caves.entry(to).or_insert(Cave::new(to));

            // Update caves.
            let from_cave = caves.get(from).unwrap();
            let to_cave = caves.get(to).unwrap();
            from_cave.borrow_mut().add(to_cave);
            to_cave.borrow_mut().add(from_cave);
        });
    caves
}

pub fn day12a() -> String {
    let data = fs::read_to_string("assets/day12.txt").expect("Could not load file");
    "".to_string()
}

pub fn day12b() -> String {
    let data = fs::read_to_string("assets/day12.txt").expect("Could not load file");
    "".to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const RAW_INPUT: &'static str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end
";

    #[test]
    fn test_day12_parta_sample() {
        let input = parse_input(RAW_INPUT);
        assert_eq!(input.len(), 6);
        dbg!(&input.keys());
        assert_eq!(0, 10);
    }

    #[test]
    fn test_day12_partb_sample() {
        assert_eq!(0, 195);
    }
}
