use super::utils;

pub fn solution() {
    let input = utils::read_from_file("files/day1_input.txt");

    let splitted_input: Vec<&str> = input.split('\n').collect();

    let mut increase_count = 0;

    for index in 1..splitted_input.len() {
        if splitted_input[index].parse::<u32>().unwrap()
            > splitted_input[index - 1].parse::<u32>().unwrap()
        {
            increase_count += 1;
        }
    }

    println!("Answer to day 1 part 1: {}", increase_count);

    let mut p2_increase_amout = 0;
    let mut p2_last_window = sum_array(&splitted_input[0..3]);

    for idx in 1..splitted_input.len() - 2 {
        let current_window = sum_array(&splitted_input[idx..idx + 3]);

        if current_window > p2_last_window {
            p2_increase_amout += 1;
        }

        p2_last_window = current_window;
    }

    println!("Answer to day 1 part 2: {}", p2_increase_amout);
}

fn sum_array(arr: &[&str]) -> i32 {
    let mut sum = 0;
    for cs in arr {
        sum += cs.parse::<i32>().unwrap();
    }

    sum
}
