use num_traits::abs;

use super::utils::{self, Extensions};

pub fn solution() {
    let mut input = utils::read_from_file("files/day7_input.txt")
        .as_str()
        .to_vec::<i64>(",");
    input.sort();

    let median: i64 = input[(input.len() as i64 / 2) as usize];
    let mean: i64 = (input.iter().sum::<i64>() as f64 / input.len() as f64).floor() as i64;

    println!("Answer to day 7 part 1: {}", calculate_fuel(&input, median));
    println!(
        "Answer to day 7 part 2: {}",
        calculate_variable_fuel(&input, mean)
    );
}

fn calculate_fuel(xs: &Vec<i64>, to: i64) -> i64 {
    let mut total_amount = 0;

    for x in xs {
        total_amount += abs(x - to);
    }

    total_amount
}

fn calculate_variable_fuel(xs: &Vec<i64>, to: i64) -> i64 {
    let mut total_amount = 0;

    for x in xs {
        let sum_to = abs(x - to);

        total_amount += sum_to * (1 + sum_to) / 2;
    }

    total_amount
}
