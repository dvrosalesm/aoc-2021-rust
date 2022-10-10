use super::utils;

// From now on I will start actually optimizing the problems
pub fn solution() {
    let input = utils::read_from_file("files/day6_input.txt");

    let start_fish: Vec<usize> = input
        .split(',')
        .collect::<Vec<&str>>()
        .iter()
        .map(|&x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let fish_after_80_days = fish_after_nth_iter(&start_fish, 80);
    let fish_after_256_days = fish_after_nth_iter(&start_fish, 256);

    println!("Answer to day 6 part 1: {}", fish_after_80_days);
    println!("Answer to day 6 part 2: {}", fish_after_256_days);
}

fn fish_after_nth_iter(start_fish: &Vec<usize>, iterations: i32) -> u64 {
    let mut fish_per_step: [u64; 9] = [0; 9];

    for fish in start_fish {
        fish_per_step[*fish] += 1;
    }

    for _i in 0..iterations {
        let done = fish_per_step[0];
        fish_per_step[0] = fish_per_step[1];
        fish_per_step[1] = fish_per_step[2];
        fish_per_step[2] = fish_per_step[3];
        fish_per_step[3] = fish_per_step[4];
        fish_per_step[4] = fish_per_step[5];
        fish_per_step[5] = fish_per_step[6];
        fish_per_step[6] = fish_per_step[7] + done;
        fish_per_step[7] = fish_per_step[8];
        fish_per_step[8] = done;
    }

    let mut total_fish: u64 = 0;
    fish_per_step.map(|x| total_fish += x);

    total_fish
}
