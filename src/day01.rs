use crate::read_input;

pub fn day01a() -> String {
    let data = read_input::<&str, i64>("assets/day01.txt");
    data.windows(2)
        .map(|i| if i[0] < i[1] { 1 } else { 0 })
        .sum::<i64>()
        .to_string()
}

pub fn day01b() -> String {
    let data = read_input::<&str, i64>("assets/day01.txt");
    let measurement_windows = data
        .windows(3)
        .map(|w| w.iter().into_iter().sum::<i64>())
        .collect::<Vec<i64>>();
    measurement_windows
        .windows(2)
        .map(|i| if i[0] < i[1] { 1 } else { 0 })
        .sum::<i64>()
        .to_string()
}
