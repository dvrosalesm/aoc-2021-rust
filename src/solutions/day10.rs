use super::utils;

use std::collections::VecDeque;

enum PointsPerIllegalChar {
    Parenthesis = 3,
    Bracket = 57,
    Braces = 1197,
    Arrow = 25137,
}

enum PointsPerCompletionChar {
    Parenthesis = 1,
    Bracket = 2,
    Braces = 3,
    Arrow = 4,
}

pub fn solution() {
    let input = utils::read_from_file("files/day10_input.txt");
    let lines: Vec<&str> = input.split('\n').collect();
    let mut total_score_points = 0;
    let mut autocompletion_scores: Vec<i64> = Vec::new();

    for line in lines {
        let mut chunks: VecDeque<char> = VecDeque::new();
        let mut incorrect_tokens: Vec<char> = Vec::new();

        for c in line.chars() {
            if chunks.front().is_none() || is_starting_token(c) {
                chunks.push_front(c);
            } else {
                let front_c = chunks.pop_front().unwrap();
                if !a_closes_b(front_c, c) {
                    incorrect_tokens.push(c);
                }
            }
        }

        if !incorrect_tokens.is_empty() {
            total_score_points += get_score_for_ictoken(incorrect_tokens[0]);
        } else {
            let mut autocomplete_score: i64 = 0;
            let mut next_char = chunks.pop_front();

            while next_char.is_some() {
                autocomplete_score =
                    (autocomplete_score * 5) + get_score_for_actoken(next_char.unwrap());

                next_char = chunks.pop_front();
            }

            autocompletion_scores.push(autocomplete_score);
        }
    }

    autocompletion_scores.sort();

    println!("Answer to day 10 part 1: {}", total_score_points);
    println!(
        "Answer to day 10 part 2: {}",
        autocompletion_scores[((autocompletion_scores.len() - 1) / 2)]
    );
}

fn is_starting_token(c: char) -> bool {
    c == '(' || c == '{' || c == '[' || c == '<'
}

fn a_closes_b(a: char, b: char) -> bool {
    (a == '(' && b == ')')
        || (a == '[' && b == ']')
        || (a == '{' && b == '}')
        || (a == '<' && b == '>')
}

fn get_score_for_ictoken(c: char) -> i32 {
    match c {
        ')' => PointsPerIllegalChar::Parenthesis as i32,
        ']' => PointsPerIllegalChar::Bracket as i32,
        '}' => PointsPerIllegalChar::Braces as i32,
        '>' => PointsPerIllegalChar::Arrow as i32,
        _ => 0,
    }
}

fn get_score_for_actoken(c: char) -> i64 {
    match c {
        '(' => PointsPerCompletionChar::Parenthesis as i64,
        '[' => PointsPerCompletionChar::Bracket as i64,
        '{' => PointsPerCompletionChar::Braces as i64,
        '<' => PointsPerCompletionChar::Arrow as i64,
        _ => 0,
    }
}
