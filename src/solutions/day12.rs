use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::{ds::Node, utils};

#[allow(dead_code)]
pub fn solution() {
    let input: String = utils::read_from_file("files/day12_input.txt");

    let start_node = input_to_tree(input);

    // trasverse this nonsense p1
    let mut found_paths1: Vec<Vec<String>> = Vec::new();
    trasverse_crazy_tree_p1(&start_node, &mut Vec::new(), &mut found_paths1);

    // trasverse this nonsense p2
    let mut found_paths2: Vec<Vec<String>> = Vec::new();
    trasverse_crazy_tree_p2(&start_node, &mut Vec::new(), &mut found_paths2, false);

    println!("Answer to day 12 part 1: {}", found_paths1.len());
    println!("Answer to day 12 part 2: {}", found_paths2.len());
}

#[allow(dead_code)]
pub fn trasverse_crazy_tree_p1(
    node: &Rc<RefCell<Node<String>>>,
    visited: &mut Vec<String>,
    found_paths: &mut Vec<Vec<String>>,
) {
    visited.push((*node).borrow().value.to_string());

    if (*node).borrow().value == "end" {
        found_paths.push(visited.to_vec());
        return;
    }

    for i in 0..(*node).borrow().children.len() {
        let next_path = (*(*node).borrow().children[i]).borrow().value.to_string();
        let times_found = visited.iter().filter(|&s| *s == next_path).count();

        if next_path.chars().next().unwrap().is_uppercase()
            || (next_path != "start" && times_found == 0)
        {
            trasverse_crazy_tree_p1(
                &(*node).borrow().children[i],
                &mut visited.to_vec(),
                found_paths,
            );
        }
    }
}

pub fn trasverse_crazy_tree_p2(
    node: &Rc<RefCell<Node<String>>>,
    visited: &mut Vec<String>,
    found_paths: &mut Vec<Vec<String>>,
    has_done_sc_twice: bool,
) {
    visited.push((*node).borrow().value.to_string());

    if (*node).borrow().value == "end" {
        found_paths.push(visited.to_vec());
        return;
    }

    for i in 0..(*node).borrow().children.len() {
        let next_path = (*(*node).borrow().children[i]).borrow().value.to_string();
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

            trasverse_crazy_tree_p2(
                &(*node).borrow().children[i],
                &mut visited.to_vec(),
                found_paths,
                has_dt,
            );
        }
    }
}

pub fn input_to_tree(input: String) -> Rc<RefCell<Node<String>>> {
    let lines: Vec<&str> = input.split('\n').collect();
    let mut nodes: HashMap<String, Rc<RefCell<Node<String>>>> = HashMap::new();

    // Create nodes
    for curr_line in lines.iter() {
        let new_nodes: Vec<&str> = curr_line.split('-').collect();

        for &curr_node in new_nodes.iter() {
            if !nodes.contains_key(curr_node) {
                nodes.insert(
                    curr_node.to_string(),
                    Rc::new(RefCell::new(Node {
                        value: curr_node.to_string(),
                        children: Vec::new(),
                    })),
                );
            }
        }
    }

    // Create roads
    for &curr_line in lines.iter() {
        let new_nodes: Vec<&str> = curr_line.split('-').collect();
        let left_node = nodes.get(new_nodes[0]).unwrap().clone();
        let right_node = nodes.get(new_nodes[1]).unwrap().clone();

        (*left_node).borrow_mut().children.push(right_node.clone());
        (*right_node).borrow_mut().children.push(left_node.clone());
    }

    // Connect nodes
    return nodes.get("start").unwrap().clone();
}
