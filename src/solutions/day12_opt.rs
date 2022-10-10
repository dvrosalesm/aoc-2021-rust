use std::{collections::HashMap, time::Instant};

use super::utils;

// The optimization was on the implementation time, not actually on the execution time :( Although it helped a little bit, I think it still runs like shit

#[allow(dead_code)]
pub fn solution() {
    let now_p1 = Instant::now();
    let input: String = utils::read_from_file("files/day12_input.txt");

    let nodes = input_to_tree(input);

    // trasverse this nonsense p1
    let mut found_paths1: Vec<Vec<String>> = Vec::new();
    trasverse_crazy_tree_p1(
        String::from("start"),
        &nodes,
        &mut Vec::new(),
        &mut found_paths1,
    );

    println!(
        "Answer to day 12 part 1: {} {:.2?}",
        found_paths1.len(),
        now_p1.elapsed()
    );

    let now_p2 = Instant::now();
    // trasverse this nonsense p2
    let mut found_paths2: Vec<Vec<String>> = Vec::new();
    trasverse_crazy_tree_p2(
        String::from("start"),
        &nodes,
        &mut Vec::new(),
        &mut found_paths2,
        false,
    );

    println!(
        "Answer to day 12 part 2: {} {:.2?}",
        found_paths2.len(),
        now_p2.elapsed()
    );
}

pub fn trasverse_crazy_tree_p1(
    current: String,
    nodes: &HashMap<String, Vec<String>>,
    visited: &mut Vec<String>,
    found_paths: &mut Vec<Vec<String>>,
) {
    let paths = nodes.get(&current).unwrap();

    visited.push(current.to_string());

    if current == "end" {
        found_paths.push(visited.to_vec());
        return;
    }

    for curr_path in paths.iter() {
        let next_path = curr_path.to_string();
        let times_found = visited.iter().filter(|&s| *s == next_path).count();

        if next_path.chars().next().unwrap().is_uppercase()
            || (next_path != "start" && times_found == 0)
        {
            trasverse_crazy_tree_p1(next_path, nodes, &mut visited.to_vec(), found_paths);
        }
    }
}

pub fn trasverse_crazy_tree_p2(
    current: String,
    nodes: &HashMap<String, Vec<String>>,
    visited: &mut Vec<String>,
    found_paths: &mut Vec<Vec<String>>,
    has_done_sc_twice: bool,
) {
    let paths = nodes.get(&current).unwrap();
    visited.push(current.to_string());

    if current == "end" {
        found_paths.push(visited.to_vec());
        return;
    }

    for curr_path in paths.iter() {
        let next_path = curr_path.to_string();
        let times_found = visited.iter().filter(|&s| *s == next_path).count();

        if next_path.chars().next().unwrap().is_uppercase()
            || (next_path != "start"
                && (times_found == 0 || (times_found == 1 && !has_done_sc_twice)))
        {
            let mut has_dt = has_done_sc_twice;

            if !next_path.chars().next().unwrap().is_uppercase()
                && !has_done_sc_twice
                && times_found == 1
            {
                has_dt = true;
            }

            trasverse_crazy_tree_p2(next_path, nodes, &mut visited.to_vec(), found_paths, has_dt);
        }
    }
}

pub fn input_to_tree(input: String) -> HashMap<String, Vec<String>> {
    let lines: Vec<&str> = input.split('\n').collect();
    let mut nodes: HashMap<String, Vec<String>> = HashMap::new();

    // Create nodes
    for curr_line in lines.iter() {
        let new_nodes: Vec<&str> = curr_line.split('-').collect();

        for &curr_node in new_nodes.iter() {
            if !nodes.contains_key(curr_node) {
                nodes.insert(curr_node.to_string(), Vec::new());
            }
        }

        let left_node = nodes.get_mut(new_nodes[0]).unwrap();
        left_node.push(new_nodes[1].to_string());

        let right_node = nodes.get_mut(new_nodes[1]).unwrap();
        right_node.push(new_nodes[0].to_string());
    }

    // Connect nodes
    nodes
}
