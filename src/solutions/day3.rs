use super::utils;

pub fn solution() {
    let input = utils::read_from_file("files/day3_input.txt");
    let splitted_input: Vec<&str> = input.split('\n').collect();

    let mut gamma_rate_str = "".to_string();
    let mut epsilon_rate_str = "".to_string();

    for i in 0..splitted_input[0].chars().count() {
        let mut one_count = 0;

        for &curr_input in splitted_input.iter() {
            if curr_input.chars().nth(i).unwrap() == '1' {
                one_count += 1
            }
        }

        if one_count > splitted_input.len() / 2 {
            gamma_rate_str.push('1');
            epsilon_rate_str.push('0');
        } else {
            gamma_rate_str.push('0');
            epsilon_rate_str.push('1');
        }
    }

    let gamma_rate = u32::from_str_radix(gamma_rate_str.as_str(), 2).unwrap();
    let sigma_rate = u32::from_str_radix(epsilon_rate_str.as_str(), 2).unwrap();

    println!("Answer to day 3 part 1: {}", gamma_rate * sigma_rate);

    let mut oxy_rating = splitted_input.to_vec();
    let mut scr_rating = splitted_input.to_vec();

    for i in 0..oxy_rating[0].chars().count() {
        let mut zeroes = Vec::<&str>::new();
        let mut ones = Vec::<&str>::new();
        let mut char_ones = 0;

        for &curr_rating in oxy_rating.iter() {
            if curr_rating.chars().nth(i).unwrap() == '1' {
                ones.push(curr_rating);
                char_ones += 1;
            } else {
                zeroes.push(curr_rating);
            }
        }

        if oxy_rating.len() == 2 {
            oxy_rating = match ones.len() {
                0 => zeroes,
                _ => ones,
            };

            break;
        }

        if char_ones >= oxy_rating.len() / 2 {
            oxy_rating = ones;
        } else {
            oxy_rating = zeroes;
        }

        if oxy_rating.len() == 1 {
            break;
        }
    }

    for i in 0..scr_rating[0].chars().count() {
        let mut zeroes = Vec::<&str>::new();
        let mut ones = Vec::<&str>::new();
        let mut char_ones = 0;

        for &curr_rating in scr_rating.iter() {
            if curr_rating.chars().nth(i).unwrap() == '1' {
                ones.push(curr_rating);
                char_ones += 1;
            } else {
                zeroes.push(curr_rating);
            }
        }

        if scr_rating.len() == 2 {
            scr_rating = match zeroes.len() {
                0 => ones,
                _ => zeroes,
            };

            break;
        }

        if char_ones >= scr_rating.len() / 2 {
            scr_rating = zeroes;
        } else {
            scr_rating = ones;
        }

        if scr_rating.len() == 1 {
            break;
        }
    }

    let oxygen_rating = u32::from_str_radix(oxy_rating[0], 2).unwrap();
    let scrubber_rating = u32::from_str_radix(scr_rating[0], 2).unwrap();

    println!(
        "Answer to day 3 part 2: {}",
        oxygen_rating * scrubber_rating
    );
}
