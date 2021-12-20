use crate::read_input_sep;
use std::collections::HashSet;

// 351901
pub fn day07a() -> String {
    let data = read_input_sep::<&str, i32>("assets/day07.txt", ",");
    let res = process_a(&data);
    res.to_string()
}

pub fn day07b() -> String {
    let data = read_input_sep::<&str, i32>("assets/day07.txt", ",");
    let res = process_b(&data);
    res.to_string()
}

fn process_a(input: &[i32]) -> i32 {
    let mut results = HashSet::new();
    let min_alignment = *input.iter().min().unwrap();
    let max_alignment = *input.iter().max().unwrap();
    for alignment in min_alignment..=max_alignment {
        let res = input
            .iter()
            .map(|&position| (position - alignment).abs())
            .sum::<i32>();
        results.insert(res);
    }
    results.into_iter().min().unwrap()
}

fn process_b(input: &[i32]) -> i32 {
    let mut results = HashSet::new();
    let min_alignment = *input.iter().min().unwrap();
    let max_alignment = *input.iter().max().unwrap();
    for alignment in min_alignment..=max_alignment {
        let res = input
            .iter()
            .enumerate()
            // .map(|position| (*position.1 - alignment).abs() * (position.0 as i32 + 1))
            // Not so sure about this formula. Got the solution from https://youtu.be/lyUxFTOCk6w?t=300.
            .map(|position| {
                (*position.1 - alignment).abs() * ((*position.1 - alignment).abs() + 1) / 2
            })
            .sum::<i32>();
        results.insert(res);
    }
    results.into_iter().min().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    const RAW_INPUT: &'static str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_day07_parta_sample() {
        let input = RAW_INPUT
            .trim()
            .split(',')
            .filter_map(|s| s.parse::<i32>().ok())
            .collect::<Vec<i32>>();
        assert_eq!(process_a(&input), 37);
    }

    #[test]
    fn test_day07_partb_sample() {
        let input = RAW_INPUT
            .trim()
            .split(',')
            .filter_map(|s| s.parse::<i32>().ok())
            .collect::<Vec<i32>>();
        assert_eq!(process_b(&input), 168);
    }
}
