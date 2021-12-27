use std::collections::HashMap;
use std::fs;

pub fn day10a() -> String {
    let data = fs::read_to_string("assets/day10.txt").expect("Could not load file");
    let input = read_input(&data);
    let res = process_lines(&input);
    res.to_string()
}

pub fn day10b() -> String {
    let data = fs::read_to_string("assets/day10.txt").expect("Could not load file");
    let input = read_input(&data);
    let res = repair_lines(&input);
    res.to_string()
}

pub fn read_input(input: &str) -> Vec<String> {
    input.trim().lines().map(String::from).collect()
}

pub fn process_line(line: &str) -> u32 {
    let mut pairs = HashMap::new();
    pairs.insert('(', ')');
    pairs.insert('[', ']');
    pairs.insert('{', '}');
    pairs.insert('<', '>');

    let mut points = HashMap::new();
    points.insert(')', 3);
    points.insert(']', 57);
    points.insert('}', 1197);
    points.insert('>', 25137);

    let mut stack: Vec<char> = Vec::new();
    let mut score = 0;

    for c in line.chars() {
        if pairs.contains_key(&c) {
            stack.push(c);
        } else if let Some(closing) = stack.pop() {
            // dbg!(&closing);
            let expected = pairs.get(&closing).unwrap();
            // dbg!(&expected);
            if c != *expected {
                // The line is corrupted.
                score += points.get(&c).unwrap();
                break;
            }
        } else {
            panic!("There is an invalid character.")
        }
    }

    score
}

pub fn process_lines(lines: &[String]) -> u32 {
    lines.iter().map(|l| process_line(l)).sum()
}

pub fn repair_line(line: &str) -> Option<u64> {
    let mut pairs = HashMap::new();
    pairs.insert('(', ')');
    pairs.insert('[', ']');
    pairs.insert('{', '}');
    pairs.insert('<', '>');

    let mut points = HashMap::new();
    points.insert(')', 1);
    points.insert(']', 2);
    points.insert('}', 3);
    points.insert('>', 4);

    let mut stack: Vec<char> = Vec::new();

    for c in line.chars() {
        if pairs.contains_key(&c) {
            stack.push(c);
        } else if let Some(closing) = stack.pop() {
            let expected = pairs.get(&closing).unwrap();
            if c != *expected {
                return None;
            }
        } else {
            panic!("There is an invalid character.")
        }
    }

    // Repair the corrupted line.
    let mut score = 0;

    stack.reverse();
    let completion = stack
        .into_iter()
        .filter(|c| pairs.contains_key(c))
        .map(|c| pairs.get(&c).unwrap().clone())
        .collect::<String>();
    // dbg!(&completion);

    // Count the points.
    for c in completion.chars() {
        score = score * 5 + points.get(&c).unwrap();
    }

    Some(score)
}

pub fn repair_lines(lines: &[String]) -> u64 {
    let mut scores = lines
        .iter()
        .filter_map(|l| repair_line(l))
        .collect::<Vec<u64>>();
    scores.sort_unstable();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod test {
    use super::*;

    const RAW_INPUT: &'static str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
";

    #[test]
    fn test_day10_parta_sample() {
        let input = read_input(RAW_INPUT);
        assert_eq!(process_lines(&input), 26397);
    }

    #[test]
    fn test_day09_partb_sample() {
        let input = read_input(RAW_INPUT);
        assert_eq!(repair_lines(&input), 288957);
    }

    #[test]
    fn test_repair_line() {
        let line = "<{([{{}}[<[[[<>{}]]]>[]]";
        let score = repair_line(line).unwrap();
        assert_eq!(score, 294);
    }
}
