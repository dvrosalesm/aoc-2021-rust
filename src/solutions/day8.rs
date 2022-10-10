use super::utils;

const SEGMENTS_IN_1: u8 = 2;
const SEGMENTS_IN_4: u8 = 4;
const SEGMENTS_IN_7: u8 = 3;
const SEGMENTS_IN_8: u8 = 7;

enum NoToSegments {
    A = 0,
    // B = 1,
    C = 2,
    D = 3,
    // E = 4,
    F = 5,
    // G = 6,
}

pub fn solution() {
    let input = utils::read_from_file("files/day8_input.txt");
    let lines: Vec<&str> = input.split('\n').collect();
    let mut count_known_digits = 0;
    let mut sum_decoded_numbers = 0;

    for &line in &lines {
        for sequence in line.split(" | ").collect::<Vec<&str>>()[1]
            .split(' ')
            .collect::<Vec<&str>>()
        {
            let current_len = sequence.len() as u8;
            if current_len == SEGMENTS_IN_1
                || current_len == SEGMENTS_IN_4
                || current_len == SEGMENTS_IN_7
                || current_len == SEGMENTS_IN_8
            {
                count_known_digits += 1;
            }
        }
    }

    for line in &lines {
        let mut digits: Vec<String> = Vec::new();
        let mut segments: Vec<String> = Vec::new();
        let sequences = line.split(" | ").collect::<Vec<&str>>()[0]
            .split(' ')
            .collect::<Vec<&str>>();

        digits.resize(10, String::from(""));
        segments.resize(10, String::from(""));

        // Find 1, 4, 7 and 8
        sequences.iter().copied().for_each(|seq| match seq.len() {
            2 => {
                digits[1] = order_string(seq);
            }
            4 => {
                digits[4] = order_string(seq);
            }
            3 => {
                digits[7] = order_string(seq);
            }
            7 => {
                digits[8] = order_string(seq);
            }
            _ => {}
        });

        // Find segment a doing 1 <-> 7
        for i in 0..digits[7].len() {
            let current_seg = digits[7].chars().nth(i).unwrap();
            if !digits[1].contains(current_seg) {
                segments[NoToSegments::A as usize] = current_seg.to_string();
            }
        }

        // Find 6 by comparing its length and to 1
        // We can also find segments C and F
        sequences.iter().copied().for_each(|seq| {
            if seq.len() == 6 {
                if seq.contains(digits[1].chars().next().unwrap())
                    && !seq.contains(digits[1].chars().nth(1).unwrap())
                {
                    digits[6] = order_string(seq);
                    segments[NoToSegments::C as usize] =
                        digits[1].chars().nth(1).unwrap().to_string();
                    segments[NoToSegments::F as usize] =
                        digits[1].chars().next().unwrap().to_string();
                } else if seq.contains(digits[1].chars().nth(1).unwrap())
                    && !seq.contains(digits[1].chars().next().unwrap())
                {
                    digits[6] = order_string(seq);
                    segments[NoToSegments::C as usize] =
                        digits[1].chars().next().unwrap().to_string();
                    segments[NoToSegments::F as usize] =
                        digits[1].chars().nth(1).unwrap().to_string();
                }
            }
        });

        // Find 5 by comparing length = 5 and missing segment C
        // Find 2 by comparing length = 5 and missing segment F
        // Find 3 by comparing length = 5 and not missing C nor F
        sequences.iter().copied().into_iter().for_each(|seq| {
            if seq.len() == 5 {
                if !seq.contains(segments[NoToSegments::C as usize].as_str()) {
                    digits[5] = order_string(seq);
                }

                if !seq.contains(segments[NoToSegments::F as usize].as_str()) {
                    digits[2] = order_string(seq);
                }

                if seq.contains(segments[NoToSegments::C as usize].as_str())
                    && seq.contains(segments[NoToSegments::F as usize].as_str())
                {
                    digits[3] = order_string(seq);
                }
            }
        });

        // Find out which is segment D from 1, 2 and 4
        for i in 0..digits[4].len() {
            let current_segment = digits[4].chars().nth(i).unwrap();

            if !digits[1].contains(current_segment) && digits[2].contains(current_segment) {
                // We got segment d!
                segments[NoToSegments::D as usize] = current_segment.to_string();
            }
        }

        // Only 0 and 9 left...
        sequences.iter().copied().for_each(|seq| {
            if seq.len() == 6 && order_string(seq) != digits[6] {
                // we don't want to overwrite 6
                if seq.contains(segments[NoToSegments::D as usize].as_str()) {
                    digits[9] = order_string(seq);
                } else {
                    digits[0] = order_string(seq);
                }
            }
        });

        // Calculating the numbers
        let numbers = line.split(" | ").collect::<Vec<&str>>()[1]
            .split(' ')
            .collect::<Vec<&str>>();
        let mut decoded_number = String::new();

        for number in numbers {
            let formatted_number = order_string(number);

            for (i, curr_digit) in (&digits).iter().enumerate() {
                if curr_digit == &formatted_number {
                    decoded_number = format!("{}{}", decoded_number, i);
                }
            }
        }

        sum_decoded_numbers += decoded_number.parse::<i32>().unwrap();
    }

    println!("Answer to day 8 part 1: {}", count_known_digits);
    println!("Answer to day 8 part 2: {}", sum_decoded_numbers);
}

fn order_string(to_sort: &str) -> String {
    let mut chars = to_sort.chars().collect::<Vec<char>>();
    chars.sort();
    String::from_iter(chars)
}
