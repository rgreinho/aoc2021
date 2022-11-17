use itertools::Itertools;
use std::{collections::HashMap, fs};

pub fn day14a() -> String {
    let data = fs::read_to_string("assets/day14.txt").expect("Could not load file");
    let (polymer_template, rules) = parse_input(&data);
    let polymer = polymerization(&polymer_template, &rules, 10);
    let freq = frequency(polymer);
    let (most, least) = most_least_char(freq);
    let diff = most - least;
    diff.to_string()
}

pub fn day14b() -> String {
    let data = fs::read_to_string("assets/day14.txt").expect("Could not load file");
    "".to_string()
}

pub fn parse_input(input: &str) -> (String, HashMap<&str, &str>) {
    let polymer_template = input.trim().lines().take(1).collect::<String>();
    let rules = input
        .trim()
        .lines()
        .skip(2)
        .map(|l| l.split_once(" -> "))
        .map(|rule| (rule.unwrap().0, rule.unwrap().1))
        .collect::<HashMap<&str, &str>>();
    (polymer_template, rules)
}

pub fn polymerization(
    polymer_template: &str,
    rules: &HashMap<&str, &str>,
    steps: usize,
) -> Vec<char> {
    let mut template = polymer_template.chars().collect::<Vec<char>>();
    for step in 0..steps {
        dbg!(&step);
        // Generate insertions.
        let insertions = template
            .windows(2)
            .map(String::from_iter)
            .filter_map(|rule| rules.get(rule.as_str()))
            .map(|&c| c.to_string())
            .collect::<String>();
        let insertion_vec = insertions.chars().map(|c| c).collect::<Vec<char>>();

        // Merge the results.
        template = template
            .into_iter()
            .interleave(insertion_vec)
            .map(|c| c)
            .collect();
    }
    template
}

pub fn frequency(polymer: Vec<char>) -> HashMap<char, i64> {
    let mut letter_counts: HashMap<char, i64> = HashMap::new();
    for c in polymer {
        *letter_counts.entry(c).or_insert(0) += 1;
    }
    letter_counts
}

pub fn most_least_char(frequency_map: HashMap<char, i64>) -> (i64, i64) {
    let most_common_element = frequency_map.values().max().unwrap().clone();
    let least_common_element = frequency_map.values().min().unwrap().clone();
    (most_common_element, least_common_element)
}

pub fn memoized_polymerization(
    polymer_template: &str,
    rules: &HashMap<&str, &str>,
    steps: usize,
) -> Vec<char> {
    let template = polymer_template.chars().collect::<Vec<char>>();
    let mut cache: HashMap<(String, usize), Vec<char>> = HashMap::new();
    for window in template.windows(2) {
        let mut partial_template = window.iter().map(|&c| c.clone()).collect::<Vec<char>>();
        let cache_key = window.iter().collect::<String>();
        for step in 0..steps {
            let insertions = partial_template
                .windows(2)
                .map(String::from_iter)
                .filter_map(|rule| rules.get(rule.as_str()))
                .map(|&c| c.to_string())
                .collect::<String>();
            let insertion_vec = insertions.chars().map(|c| c).collect::<Vec<char>>();
            partial_template = partial_template
                .into_iter()
                .interleave(insertion_vec)
                .map(|c| c)
                .collect();
            cache.insert((cache_key.clone(), step), partial_template.clone());
        }
    }
    vec![]
}

#[cfg(test)]
mod test {
    use super::*;

    const RAW_INPUT: &'static str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
";

    #[test]
    fn test_day14_parta_sample() {
        let (polymer_template, rules) = parse_input(RAW_INPUT);
        let polymer = polymerization(&polymer_template, &rules, 10);
        assert_eq!(polymer.len(), 3073);
        let freq = frequency(polymer);
        let (most, least) = most_least_char(freq);
        assert_eq!(most - least, 1588);
    }

    #[test]
    fn test_day14_partb_sample() {
        let (polymer_template, rules) = parse_input(RAW_INPUT);
        let polymer = polymerization(&polymer_template, &rules, 40);
        assert_eq!(polymer.len(), 3073);
        let freq = frequency(polymer);
        let (most, least) = most_least_char(freq);
        assert_eq!(most, 2192039569602);
        assert_eq!(least, 3849876073);
        assert_eq!(most - least, 2188189693529);
    }
}
