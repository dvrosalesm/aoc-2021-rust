use std::collections::HashMap;

use super::utils;

#[derive(Debug)]
struct BingoCardBox {
    value: String,
    marked: bool,
}

#[derive(Debug)]
struct BingoCard {
    positions: Vec<BingoCardBox>,
}

impl Default for BingoCardBox {
    fn default() -> BingoCardBox {
        BingoCardBox {
            value: "".to_string(),
            marked: false,
        }
    }
}

impl BingoCard {
    pub fn has_won(&self) -> bool {
        for i in 0..5 {
            // check columns
            if self.positions[i].marked
                && self.positions[i + 5].marked
                && self.positions[i + 10].marked
                && self.positions[i + 15].marked
                && self.positions[i + 20].marked
            {
                return true;
            }

            // check row
            let current_row = i * 5;
            if self.positions[current_row].marked
                && self.positions[current_row + 1].marked
                && self.positions[current_row + 2].marked
                && self.positions[current_row + 3].marked
                && self.positions[current_row + 4].marked
            {
                return true;
            }
        }

        false
    }

    pub fn sum_of_unmarked(&self) -> i32 {
        let mut sum = 0;
        for i in 0..self.positions.len() {
            if !self.positions[i].marked {
                sum += self.positions[i].value.parse::<i32>().unwrap();
            }
        }
        sum
    }
}

pub fn solution() {
    let input = utils::read_from_file("files/day4_input.txt");

    let splitted_string: Vec<&str> = input.split("\n\n").collect();

    // Moves
    let numbers: Vec<&str> = splitted_string[0].split(',').collect();

    // Create bingo cards
    let mut cards: Vec<BingoCard> = vec![];

    for &card in splitted_string[1..splitted_string.len()].iter() {
        let lines: Vec<&str> = card.split('\n').collect();
        let mut positions: Vec<BingoCardBox> = vec![];

        for line in lines.iter() {
            positions.push(BingoCardBox {
                value: line[0..2].trim().to_string(),
                marked: false,
            });
            positions.push(BingoCardBox {
                value: line[3..5].trim().to_string(),
                marked: false,
            });
            positions.push(BingoCardBox {
                value: line[6..8].trim().to_string(),
                marked: false,
            });
            positions.push(BingoCardBox {
                value: line[9..11].trim().to_string(),
                marked: false,
            });
            positions.push(BingoCardBox {
                value: line[12..14].trim().to_string(),
                marked: false,
            });
        }

        cards.push(BingoCard { positions })
    }

    let mut fw_idx = usize::MAX;
    let mut fw_sum = 0;
    let mut fw_winning_number = 0;

    let mut winners = HashMap::<usize, bool>::new();

    let mut lw_sum = 0;
    let mut lw_winning_number = 0;

    for number in numbers {
        for (i, card) in cards.iter_mut().enumerate() {
            for j in 0..card.positions.len() {
                if card.positions[j].value == number {
                    card.positions[j].marked = true;

                    if card.has_won() {
                        let winning_number = number.parse::<i32>().unwrap();

                        if fw_idx == usize::MAX {
                            fw_idx = i;
                            fw_winning_number = winning_number;
                            fw_sum = card.sum_of_unmarked();
                        }

                        if !winners.contains_key(&i) {
                            lw_sum = card.sum_of_unmarked();
                            lw_winning_number = winning_number;
                        }

                        winners.insert(i, true);
                    }
                }
            }
        }
    }

    println!("Answer to day 4 part 1: {}", fw_sum * fw_winning_number);
    println!("Answer to day 4 part 2: {}", lw_sum * lw_winning_number);
}
