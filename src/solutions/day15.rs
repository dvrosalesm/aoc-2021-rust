use std::{cmp::Ordering, collections::BinaryHeap};

use super::utils;

// Djikstra's Power!
#[derive(Copy, Clone, Eq, PartialEq)]
struct StateCost {
    cost: usize,
    position: usize,
}

impl Ord for StateCost {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for StateCost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn solution() {
    const RESCALE_TIMES: u8 = 5;
    let input: String = utils::read_from_file("files/day15_input.txt");
    let width: usize = input.split('\n').next().unwrap().len();
    let grid: String = input.split('\n').collect::<Vec<&str>>().join("");
    let numeric_grid = str_to_grid(&grid);

    if let Some(best_path) = get_lowest_cost(&numeric_grid, width) {
        println!("Answer to day 15 part 1: {}", best_path);
    }

    let augmented_grid = rescale_grid(&numeric_grid, width, RESCALE_TIMES);

    if let Some(best_pat_p2) = get_lowest_cost(&augmented_grid, width * RESCALE_TIMES as usize) {
        println!("Answer to day 15 part 2: {}", best_pat_p2);
    }
}

fn str_to_grid(grid: &str) -> Vec<u8> {
    let mut numeric_grid: Vec<u8> = Vec::new();

    for c in grid.chars() {
        numeric_grid.push(c.to_string().parse::<u8>().unwrap())
    }

    numeric_grid
}

fn get_lowest_cost(numeric_grid: &[u8], width: usize) -> Option<usize> {
    let mut p_queue: BinaryHeap<StateCost> = BinaryHeap::new();
    let mut cost_grid: Vec<usize> = vec![usize::MAX; numeric_grid.len()];
    let numeric_grid_len = numeric_grid.len();

    cost_grid[0] = 0;
    p_queue.push(StateCost {
        cost: 0,
        position: 0,
    });

    while let Some(StateCost { cost, position }) = p_queue.pop() {
        if position == numeric_grid.len() - 1 {
            return Some(cost);
        }

        if cost > cost_grid[position] {
            continue;
        }

        for n in get_neighbors(position, numeric_grid_len, width) {
            let next = StateCost {
                cost: numeric_grid[n] as usize + cost,
                position: n,
            };

            if next.cost < cost_grid[next.position] {
                p_queue.push(next);

                cost_grid[next.position] = next.cost;
            }
        }
    }

    None
}

fn rescale_grid(numeric_grid: &[u8], width: usize, times: u8) -> Vec<u8> {
    let new_width = width * times as usize;
    let mut reescaled_grid: Vec<u8> = vec![0; numeric_grid.len() * (times * times) as usize];

    // fill all first row
    for (curr_idx, curr) in numeric_grid.iter().enumerate() {
        for time in 0..times {
            let next_idx =
                (curr_idx % width) + (width * time as usize) + ((curr_idx / width) * new_width);
            let mut next_val = curr.to_owned();

            if time != 0 {
                next_val += time;

                if next_val > 9 {
                    next_val -= 9;
                }
            }

            reescaled_grid[next_idx] = next_val;
        }
    }

    let first_block_size = numeric_grid.len() * times as usize;
    // then cascade downwards
    for curr_idx in 0..first_block_size {
        for time in 1..times as usize {
            let next_idx = (time as usize * first_block_size) + curr_idx;

            let mut next_val = reescaled_grid[curr_idx] + time as u8;

            if next_val > 9 {
                next_val -= 9;
            }

            reescaled_grid[next_idx] = next_val;
        }
    }

    reescaled_grid
}

fn get_neighbors(curr: usize, total_length: usize, width: usize) -> Vec<usize> {
    let mut neighbors: Vec<usize> = Vec::new();

    if curr % width != 0 {
        neighbors.push(curr - 1);
    }

    if (curr + 1) % width != 0 {
        neighbors.push(curr + 1);
    }

    if curr >= width {
        neighbors.push(curr - width);
    }

    if curr < (total_length - width) {
        neighbors.push(curr + width);
    }

    neighbors
}

// PREVIOUS DJIKSTRA IMPLEMENTATION
// fn get_lowest_cost(numeric_grid: &[u8], width: usize) -> u32 {
//     let mut cost_grid: Vec<u32> = vec![u32::MAX; numeric_grid.len()];
//     cost_grid[0] = 0;

//     for cidx in 0..cost_grid.len() - 1 {
//         // Neighbor to the left
//         if cidx % width > 0 {
//             let new_cost = cost_grid[cidx] + numeric_grid[cidx - 1] as u32;
//             if cost_grid[cidx - 1] > new_cost {
//                 cost_grid[cidx - 1] = cost_grid[cidx] + numeric_grid[cidx - 1] as u32;
//             }
//         }

//         // Neighbor to the right
//         if cidx + 1 % width != 0 {
//             let new_cost = cost_grid[cidx] + numeric_grid[cidx + 1] as u32;
//             if cost_grid[cidx + 1] > new_cost {
//                 cost_grid[cidx + 1] = cost_grid[cidx] + numeric_grid[cidx + 1] as u32;
//             }
//         }

//         // Upstairs Neighbor
//         if cidx > width {
//             let new_cost = cost_grid[cidx] + numeric_grid[cidx - width] as u32;
//             if cost_grid[cidx - width] > new_cost {
//                 cost_grid[cidx - width] = cost_grid[cidx] + numeric_grid[cidx - width] as u32;
//             }
//         }

//         // Downstairs Neighbor
//         if cidx + width < cost_grid.len() {
//             let new_cost = cost_grid[cidx] + numeric_grid[cidx + width] as u32;
//             if cost_grid[cidx + width] > new_cost {
//                 cost_grid[cidx + width] = cost_grid[cidx] + numeric_grid[cidx + width] as u32;
//             }
//         }
//     }

//     cost_grid[cost_grid.len() - 1] - cost_grid[0]
// }
