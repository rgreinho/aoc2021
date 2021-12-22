use crate::read_input_sep;
use std::fs;

pub fn day08a() -> String {
    let data = fs::read_to_string("assets/day08.txt").expect("Could not load file");
    let input = parse_input(&data);
    let res = process_a(&input);
    res.to_string()
}

pub fn day08b() -> String {
    let data = read_input_sep::<&str, i32>("assets/day08.txt", ",");
    "".to_string()
}

fn process_a(input: &[String]) -> usize {
    input.iter().map(|s| count_easy_digits(s)).sum()
}

fn process_b(input: &[String]) -> i32 {
    0
}

fn parse_input(input: &str) -> Vec<String> {
    let input_lines = input.lines().map(String::from).collect::<Vec<String>>();
    input_lines
        .iter()
        .map(|s| s.split("|").last().unwrap())
        .map(String::from)
        .collect::<Vec<String>>()
}

fn count_easy_digits(output: &str) -> usize {
    output
        .split_whitespace()
        .into_iter()
        .filter(|&digit| is_easy_digit(digit))
        .count()
}

// Easy digits are defined as follow:
// * 1 is the only digit that uses two segments
// * 4 is the only digit that uses four segments
// * 7 is the only digit that uses three segments
// * 8 is the only digit that uses seven segments
fn is_easy_digit(digit: &str) -> bool {
    match digit.len() {
        2 | 4 | 3 | 7 => true,
        _ => false,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const RAW_INPUT: &'static str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec |fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef |cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega |efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga |gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf |gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf |cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd |ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg |gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc |fgae cfgab fg bagce";

    #[test]
    fn test_day08_parta_sample() {
        let input = parse_input(RAW_INPUT);
        let res = process_a(&input);
        assert_eq!(res, 26);
    }

    #[test]
    fn test_day08_partb_sample() {
        let input = parse_input(RAW_INPUT);
        let res = process_b(&input);
        assert_eq!(res, 61229);
    }
}
