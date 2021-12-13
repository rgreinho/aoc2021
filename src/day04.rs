use std::fmt;
use std::fs;

const GRID_ROWS: usize = 5;
const GRID_COLS: usize = 5;

#[derive(Debug, Clone, Copy)]
struct Number {
    value: u8,
    called: bool,
}

impl Number {
    pub fn new(value: u8) -> Self {
        Number {
            value,
            called: false,
        }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:>3}", self.value)
    }
}

#[derive(Debug, Clone)]
struct BingoCard {
    grid: Vec<Number>,
    rows: usize,
    columns: usize,
}

impl BingoCard {
    pub fn from_slice(numbers: &[u8], rows: usize, columns: usize) -> Self {
        if rows * columns != numbers.len() {
            panic!("Invalid bingo card shape.")
        }
        BingoCard {
            grid: numbers.iter().map(|&n| Number::new(n)).collect(),
            rows,
            columns,
        }
    }

    pub fn has_full_row(&self) -> bool {
        for row in 0..self.rows {
            if self
                .grid
                .iter()
                .skip(row * self.columns)
                .take(self.columns)
                .all(|n| n.called)
            {
                return true;
            }
        }
        false
    }

    pub fn has_full_column(&self) -> bool {
        for col in 0..self.columns {
            if self
                .grid
                .iter()
                .skip(col)
                .step_by(self.columns)
                .all(|n| n.called)
            {
                return true;
            }
        }
        false
    }

    pub fn wins(&self) -> bool {
        self.has_full_row() || self.has_full_column()
    }

    pub fn calls(&mut self, value: u8) {
        if let Some(n) = self.grid.iter_mut().find(|n| n.value == value) {
            n.called = true
        }
    }

    pub fn unmarked(&self) -> u32 {
        self.grid
            .iter()
            .filter(|n| !n.called)
            .map(|&n| n.value as u32)
            .sum::<u32>()
    }
}

impl fmt::Display for BingoCard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, n) in self.grid.iter().enumerate() {
            if i > 0 && i % self.columns == 0 {
                let _ = writeln!(f);
            }
            let _ = write!(f, "{}", n);
        }
        Ok(())
    }
}

fn read_input(input: &str) -> (Vec<u8>, Vec<BingoCard>) {
    // Prepare the line iterator.
    let mut iter_values = input.lines();

    // Extract the first line as the list of numbers that were called.
    let calls: Vec<u8> = iter_values
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    iter_values.next();

    // Extract the bingo cards.
    let mut bingo_cards: Vec<BingoCard> = Vec::new();
    let mut card: Vec<Vec<u8>> = vec![];
    while let Some(line) = iter_values.next() {
        if line.is_empty() {
            let flat_card: Vec<u8> = card.clone().into_iter().flatten().collect();
            let bingo_card = BingoCard::from_slice(&flat_card, GRID_ROWS, GRID_COLS);
            bingo_cards.push(bingo_card);
            card = vec![];
        } else {
            let card_line: Vec<u8> = line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            card.push(card_line);
        }
    }
    (calls, bingo_cards)
}

fn process_cards(calls: Vec<u8>, mut bingo_cards: Vec<BingoCard>) -> u32 {
    for call in calls {
        for bingo_card in bingo_cards.iter_mut() {
            bingo_card.calls(call);
            if bingo_card.wins() {
                return (call as u32) * bingo_card.unmarked();
            }
        }
    }
    0
}

fn last_winning(calls: Vec<u8>, mut bingo_cards: Vec<BingoCard>) -> u32 {
    let mut all_won = vec![false; bingo_cards.len()];
    for call in calls {
        for (i, bingo_card) in bingo_cards.iter_mut().enumerate() {
            bingo_card.calls(call);
            if bingo_card.wins() {
                all_won[i] = true;
            }
            if all_won.iter().all(|&b| b) {
                dbg!(&call);
                dbg!(&bingo_card.unmarked());
                dbg!(&bingo_card.grid[0]);
                return (call as u32) * bingo_card.unmarked();
            }
        }
    }
    0
}

pub fn day04a() -> String {
    // Read the input as a string.
    let values = fs::read_to_string("assets/day04.txt").expect("Could not load file");
    let (calls, bingo_cards) = read_input(&values);
    let res = process_cards(calls, bingo_cards);
    res.to_string()
}

pub fn day04b() -> String {
    // Read the input as a string.
    let values = fs::read_to_string("assets/day04.txt").expect("Could not load file");
    let (calls, bingo_cards) = read_input(&values);
    let res = last_winning(calls, bingo_cards);
    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const RAW_INPUT: &'static str =
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19

3 15  0  2 22
9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7

";

    #[test]
    pub fn test_day04_parta() {
        let (calls, bingo_cards) = read_input(RAW_INPUT);
        assert_eq!(4512, process_cards(calls, bingo_cards))
    }

    #[test]
    pub fn test_day04_partb() {
        let (calls, bingo_cards) = read_input(RAW_INPUT);
        assert_eq!(1924, last_winning(calls, bingo_cards))
    }

    #[test]
    pub fn test_bingo_card_full_row() {
        let full_row: Vec<Number> = vec![
            Number {
                value: 1,
                called: true,
            },
            Number {
                value: 2,
                called: true,
            },
            Number {
                value: 3,
                called: true,
            },
            Number {
                value: 4,
                called: false,
            },
            Number {
                value: 5,
                called: false,
            },
            Number {
                value: 6,
                called: false,
            },
        ];
        let full_card = BingoCard {
            grid: full_row,
            rows: 2,
            columns: 3,
        };
        assert_eq!(full_card.has_full_row(), true);
        assert_eq!(full_card.has_full_column(), false);
        assert_eq!(full_card.wins(), true);
    }

    #[test]
    pub fn test_bingo_card_partial_row() {
        let partial_row: Vec<Number> = vec![
            Number {
                value: 1,
                called: true,
            },
            Number {
                value: 2,
                called: true,
            },
            Number {
                value: 3,
                called: false,
            },
            Number {
                value: 4,
                called: false,
            },
            Number {
                value: 5,
                called: false,
            },
            Number {
                value: 6,
                called: false,
            },
        ];
        let partial_card = BingoCard {
            grid: partial_row,
            rows: 2,
            columns: 3,
        };
        assert_eq!(partial_card.has_full_row(), false);
        assert_eq!(partial_card.has_full_column(), false);
        assert_eq!(partial_card.wins(), false);
    }

    #[test]
    pub fn test_bingo_card_full_column() {
        let full_column: Vec<Number> = vec![
            Number {
                value: 1,
                called: true,
            },
            Number {
                value: 2,
                called: false,
            },
            Number {
                value: 3,
                called: true,
            },
            Number {
                value: 4,
                called: false,
            },
            Number {
                value: 5,
                called: true,
            },
            Number {
                value: 6,
                called: false,
            },
        ];
        let full_card = BingoCard {
            grid: full_column,
            rows: 3,
            columns: 2,
        };
        println!("{}", full_card);
        assert_eq!(full_card.has_full_row(), false);
        assert_eq!(full_card.has_full_column(), true);
        assert_eq!(full_card.wins(), true);
    }

    #[test]
    pub fn test_bingo_card_partial_column() {
        let full_row: Vec<Number> = vec![
            Number {
                value: 1,
                called: true,
            },
            Number {
                value: 2,
                called: true,
            },
            Number {
                value: 3,
                called: false,
            },
        ];
        let full_card = BingoCard {
            grid: full_row,
            rows: 3,
            columns: 1,
        };
        assert_eq!(full_card.has_full_row(), true);
        assert_eq!(full_card.has_full_column(), false);
        assert_eq!(full_card.wins(), true);
    }

    #[test]
    pub fn test_bingo_card_unmarked() {
        let full_row: Vec<Number> = vec![
            Number {
                value: 1,
                called: true,
            },
            Number {
                value: 2,
                called: true,
            },
            Number {
                value: 3,
                called: false,
            },
            Number {
                value: 4,
                called: false,
            },
            Number {
                value: 5,
                called: true,
            },
            Number {
                value: 6,
                called: false,
            },
        ];
        let full_card = BingoCard {
            grid: full_row,
            rows: 3,
            columns: 2,
        };
        assert_eq!(full_card.unmarked(), 13);
    }
}
