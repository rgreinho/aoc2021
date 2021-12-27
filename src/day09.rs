use std::fs;

#[derive(Debug, Clone)]
pub struct Point {
    index: usize,
    value: u32,
}

impl Point {
    pub fn new(index: usize, value: u32) -> Self {
        Point { index, value }
    }
}

#[derive(Debug, Clone)]
pub struct Heightmap {
    shape: (usize, usize),
    values: Vec<u32>,
}

impl Heightmap {
    pub fn from_string(input: &str) -> Self {
        let lines = input.lines().collect::<Vec<&str>>();
        let x = lines.get(0).unwrap().len();
        let y = lines.len();
        let values = input
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect::<Vec<u32>>();
        if x * y != values.len() {
            panic!("The number of values does not match the shape.")
        }
        Heightmap {
            shape: (x, y),
            values,
        }
    }

    pub fn find_neighbors(&self, index: usize) -> Vec<Option<Point>> {
        let length = self.shape.0;
        let width = self.shape.1;

        // First column has no left value.
        let left = if index % length == 0 {
            None
        } else {
            match self.values.get(index - 1) {
                None => None,
                Some(v) => Some(Point::new(index, v.clone())),
            }
        };

        // Last column has no right value.
        let right = if index % length == (length - 1) {
            None
        } else {
            match self.values.get(index + 1) {
                None => None,
                Some(v) => Some(Point::new(index, v.clone())),
            }
        };

        // First line has no up value.
        let up = if index < length {
            None
        } else {
            match self.values.get(index - length) {
                None => None,
                Some(v) => Some(Point::new(index, v.clone())),
            }
        };

        // Last line has no down value.
        let down = if index > (width * length - length) {
            None
        } else {
            match self.values.get(index + length) {
                None => None,
                Some(v) => Some(Point::new(index, v.clone())),
            }
        };

        let neighbors = vec![left, right, up, down];
        neighbors
    }

    pub fn is_low_points(self, index: usize) -> Option<(usize, u32)> {
        let point = self.values.get(index).unwrap();
        let neighbors = self.find_neighbors(index);

        if neighbors
            .into_iter()
            .filter_map(|neighbor| neighbor)
            .all(|neighbor| &neighbor.value > point)
        {
            return Some((index, point.to_owned()));
        }

        None
    }
}

pub fn find_low_points(hm: &Heightmap) -> Vec<(usize, u32)> {
    hm.values
        .iter()
        .enumerate()
        .filter_map(|p| hm.clone().is_low_points(p.0))
        .map(|p| (p.0, p.1 + 1))
        .collect()
}

pub fn risk_level(hm: &Heightmap) -> u32 {
    find_low_points(hm).iter().map(|p| p.1).sum::<u32>()
}

pub fn find_bassin_size(hm: &Heightmap, index: usize) -> u32 {
    let value = hm.values.get(index).unwrap().clone();
    let neighbors = hm.find_neighbors(index);
    let higher_neighbors: Vec<Point> = neighbors
        .iter()
        .filter_map(|p| p.as_ref())
        .filter(|p| p.value >= value)
        .map(|p| p.clone())
        .collect();

    // for neighbor in neighbors.iter {
    //   let hi
    // }
    0
}

// 530 is too low.
pub fn day09a() -> String {
    let data = fs::read_to_string("assets/day09.txt").expect("Could not load file");
    let heightmap = Heightmap::from_string(&data);
    let risk_level = risk_level(&heightmap);
    risk_level.to_string()
}

pub fn day09b() -> String {
    let data = fs::read_to_string("assets/day09.txt").expect("Could not load file");
    let heightmap = Heightmap::from_string(&data);
    let low_points = find_low_points(&heightmap);
    "".to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const RAW_INPUT: &'static str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn test_day09_parta_sample() {
        let heightmap = Heightmap::from_string(RAW_INPUT);
        let risk_level = risk_level(&heightmap);
        assert_eq!(risk_level, 15);
    }

    #[test]
    fn test_day09_partb_sample() {
        assert_eq!(0, 1134);
    }
}
