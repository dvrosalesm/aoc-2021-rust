use super::utils;

pub fn solution() {
    let input = utils::read_from_file("files/day2_input.txt");

    let splitted_input: Vec<&str> = input.split('\n').collect();

    let mut depth = 0;
    let mut position = 0;

    splitted_input.iter().for_each(|&x| {
        let movement: Vec<&str> = x.split_whitespace().collect();
        let amount = movement[1].parse::<u32>().unwrap();

        match movement[0] {
            "forward" => position += amount,
            "down" => depth += amount,
            "up" => depth -= amount,
            _ => println!("Unkown case"),
        }
    });

    println!("Answer to day 2 part 1: {}", depth * position);

    let mut depth2 = 0;
    let mut position2 = 0;
    let mut aim2 = 0;

    splitted_input.iter().for_each(|&x| {
        let movement: Vec<&str> = x.split_whitespace().collect();
        let amount = movement[1].parse::<u32>().unwrap();

        match movement[0] {
            "up" => aim2 -= amount,
            "down" => aim2 += amount,
            "forward" => {
                position2 += amount;
                depth2 += aim2 * amount;
            }
            _ => println!("Unkown case"),
        }
    });

    println!("Answer to day 2 part 2: {}", depth2 * position2)
}
