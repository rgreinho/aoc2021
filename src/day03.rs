use crate::read_input;

pub fn day03a() -> String {
    let data = read_input::<&str, String>("assets/day03.txt");
    let res = part_a(&data);
    res.to_string()
}

pub fn day03b() -> String {
    let data = read_input::<&str, String>("assets/day03.txt");
    let res = part_b(&data);
    res.to_string()
}

fn generate_matrix(data: &[String]) -> (usize, usize, Vec<Vec<u32>>) {
    let nrows = data.len();
    let ncols = data[0].len();
    let mut matrix = vec![vec![0u32; nrows]; ncols];
    for row in data.iter().enumerate() {
        for col in row.1.char_indices() {
            let value: u32 = match col.1 {
                '0' => 0,
                '1' => 1,
                _ => panic!("invalid character"),
            };
            matrix[col.0][row.0] = value;
        }
    }
    (nrows, ncols, matrix)
}

fn part_a(data: &[String]) -> u32 {
    let (nrows, _ncols, matrix) = generate_matrix(data);
    let mut gamma: String = String::new();
    let mut epsilon: String = String::new();
    for row in matrix.iter() {
        let row_sum = row.iter().sum::<u32>();
        let half = (nrows / 2) as u32;
        if row_sum > half {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }

    let gamma_rate = u32::from_str_radix(&gamma, 2).unwrap();
    let espilon_rate = u32::from_str_radix(&epsilon, 2).unwrap();
    gamma_rate * espilon_rate
}

fn filtering(data: &[String], most_common_criteria: bool) -> u32 {
    // Clone the data to manipulate it later.
    let mut data_clone = data.to_vec();

    // Start the filtering process.
    let result;
    for col in 0..data[0].len() {
        // Prepare the matrix to filter out the ratings.
        let (nrows, _ncols, matrix) = generate_matrix(data_clone.as_slice());

        // Determine the most common value for the current column.
        let row_sum = matrix[col].iter().sum::<u32>();
        let half = (nrows as f32 / 2.0).ceil() as u32;
        dbg!(&row_sum);
        dbg!(&half);
        let most_common = if (row_sum >= half) == most_common_criteria {
            '1'
        } else {
            '0'
        };
        dbg!(&most_common);

        // Remove the numbers which DON'T start with the most common value.
        data_clone = data_clone
            .iter()
            .filter(|n| n.chars().nth(col).unwrap() == most_common)
            .map(|n| n.clone())
            .collect::<Vec<String>>();
        dbg!(&data_clone);

        if data_clone.len() == 1 {
            break;
        }
    }
    result = data_clone[0].clone();
    u32::from_str_radix(&result, 2).unwrap()
}

fn part_b(data: &[String]) -> u32 {
    let oxygen_generator_rating = filtering(data, true);
    // let co2_scrubber_rating = 0;
    let co2_scrubber_rating = filtering(data, false);
    dbg!(&oxygen_generator_rating);
    // dbg!(&co2_scrubber_rating);
    oxygen_generator_rating * co2_scrubber_rating
}

#[cfg(test)]
mod tests {
    use super::*;

    const RAW_INPUT: &'static str = "00100
    11110
    10110
    10111
    10101
    01111
    00111
    11100
    10000
    11001
    00010
    01010";

    #[test]
    fn day_03_part_a_test() {
        let input = RAW_INPUT
            .lines()
            .map(|l| l.trim().to_owned())
            .collect::<Vec<String>>();
        assert_eq!(198, part_a(&input));
    }

    #[test]
    fn day_03_part_b_test() {
        let input = RAW_INPUT
            .lines()
            .map(|l| l.trim().to_owned())
            .collect::<Vec<String>>();
        assert_eq!(230, part_b(&input));
    }
}
