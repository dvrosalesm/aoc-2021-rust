use std::collections::HashMap;

use super::utils;

pub fn solution() {
    const PART1_ITERATIONS: u8 = 10;
    const PART2_ITERATIONS: u8 = 40;
    let input: String = utils::read_from_file("files/day14_input.txt");
    // sections of the input, section 0 is the template and section 1 defines the rules
    let sections: Vec<&str> = input.split("\n\n").collect();

    // Load rules
    let rules: Vec<&str> = sections[1].split('\n').collect();
    let mut polymer_rules: HashMap<String, String> = HashMap::new();
    load_rules_into_dict(&mut polymer_rules, &rules);

    // Step through the template
    let template = sections[0].to_string();
    let (mut pairs, mut occurrences) = get_initial_pairs(&template);

    for _ in 0..PART1_ITERATIONS {
        pairs = process_polymer(&mut pairs, &mut occurrences, &polymer_rules);
    }

    let mut largest_occurence: u64 = 0;
    let mut smallest_occurrence: u64 = u64::MAX;

    for (_, v) in occurrences {
        if v > largest_occurence {
            largest_occurence = v;
        }

        if v < smallest_occurrence {
            smallest_occurrence = v;
        }
    }

    println!(
        "Answer to day 14 part 1: {}",
        largest_occurence - smallest_occurrence
    );

    let (mut pairs2, mut occurrences2) = get_initial_pairs(&template);

    for _ in 0..PART2_ITERATIONS {
        pairs2 = process_polymer(&mut pairs2, &mut occurrences2, &polymer_rules);
    }

    let mut largest_occurence2: u64 = 0;
    let mut smallest_occurrence2: u64 = u64::MAX;

    for (_, v) in occurrences2 {
        if v > largest_occurence2 {
            largest_occurence2 = v;
        }

        if v < smallest_occurrence2 {
            smallest_occurrence2 = v;
        }
    }

    println!(
        "Answer to day 14 part 2: {}",
        largest_occurence2 - smallest_occurrence2
    );
}

pub fn load_rules_into_dict(dict: &mut HashMap<String, String>, rules: &[&str]) {
    for &rule in rules.iter() {
        let rule_args: Vec<&str> = rule.split(" -> ").collect();
        dict.insert(rule_args[0].to_string(), rule_args[1].to_string());
    }
}

pub fn process_polymer(
    pairs: &mut HashMap<String, u64>,
    ocurrences: &mut HashMap<String, u64>,
    rules: &HashMap<String, String>,
) -> HashMap<String, u64> {
    let mut new_pairs: HashMap<String, u64> = HashMap::new();

    for (pair, _count) in pairs.iter() {
        if rules.contains_key(pair) {
            let output = rules.get(pair).unwrap();
            let new_pair1 = format!("{}{}", pair.chars().next().unwrap(), output);
            let new_pair2 = format!("{}{}", output, pair.chars().nth(1).unwrap());

            *new_pairs.entry(new_pair1).or_insert(0) += _count;
            *new_pairs.entry(new_pair2).or_insert(0) += _count;
            *ocurrences.entry(output.to_string()).or_insert(0) += _count;
        }
    }

    new_pairs
}

fn get_initial_pairs(template: &str) -> (HashMap<String, u64>, HashMap<String, u64>) {
    let mut pairs: HashMap<String, u64> = HashMap::new();
    let mut occurrences: HashMap<String, u64> = HashMap::new();

    for i in 0..template.len() - 1 {
        let curr_char = template.chars().nth(i).unwrap();
        let next_char = template.chars().nth(i + 1).unwrap();
        let pair_key = format!("{}{}", curr_char, next_char);

        *pairs.entry(pair_key).or_insert(0) += 1;
        *occurrences.entry(curr_char.to_string()).or_insert(0) += 1;
    }

    *occurrences
        .entry(
            template
                .chars()
                .nth(template.len() - 1)
                .unwrap()
                .to_string(),
        )
        .or_insert(0) += 1;

    (pairs, occurrences)
}
