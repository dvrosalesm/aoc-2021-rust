use super::utils;

pub fn solution() {
    let input: String = utils::read_from_file("files/day11_input.txt");
    let input_lines: Vec<&str> = input.split('\n').collect();

    let octopus_pos: Vec<u8> = input_lines
        .join("")
        .chars()
        .into_iter()
        .map(|x| x.to_string().as_str().parse::<u8>().unwrap())
        .collect();

    let width = input_lines[0].len();

    let (total_flashed1, _) = total_flashes(octopus_pos.to_vec(), width, 100, false);
    let (_, all_synched2) = total_flashes(octopus_pos.to_vec(), width, 100, true);

    println!("Answer to day 11 part 1: {}", total_flashed1);
    println!("Answer to day 11 part 2: {}", all_synched2);
}

fn total_flashes(
    mut state: Vec<u8>,
    width: usize,
    count_to: u32,
    wait_for_synched: bool,
) -> (u32, u32) {
    let mut total_flashed: u32 = 0;
    let mut all_synched: u32 = 0;
    let mut step_counter: u32 = 0;

    loop {
        let mut flashed: Vec<usize> = Vec::new();
        for i in 0..state.len() {
            step(&mut state, width, i, &mut flashed);
        }

        if step_counter <= count_to {
            //utils::display_grid(&state, width);
            total_flashed += flashed.len() as u32;
        }

        if flashed.len() == state.len() && all_synched == 0 {
            all_synched = step_counter + 1;
        }

        step_counter += 1;

        if wait_for_synched {
            if all_synched != 0 {
                break;
            }
        } else if step_counter >= count_to {
            break;
        }
    }

    (total_flashed, all_synched)
}

fn step(state: &mut Vec<u8>, width: usize, pos: usize, flashed: &mut Vec<usize>) {
    if flashed.contains(&pos) {
        return;
    }

    state[pos] += 1;

    if state[pos] == 10 {
        flashed.push(pos);
        state[pos] = 0;

        // Top part
        if pos >= width {
            let top = pos - width;
            let top_right = top + 1;
            step(state, width, top, flashed);

            if (top + 1) % width != 0 {
                step(state, width, top_right, flashed);
            }

            if pos > 0 && top % width != 0 {
                let top_left = top - 1;
                step(state, width, top_left, flashed);
            }
        }

        if (pos + 1) % width != 0 && (pos + 1) < state.len() {
            let right = pos + 1;
            step(state, width, right, flashed);
        }

        if pos % width != 0 && pos > 0 {
            let left = pos - 1;
            step(state, width, left, flashed);
        }

        if pos + width < state.len() {
            let bottom = pos + width;
            step(state, width, bottom, flashed);

            if bottom + 1 < state.len() && (pos + 1) % width != 0 {
                let bottom_right = bottom + 1;
                step(state, width, bottom_right, flashed);
            }

            if pos % width != 0 {
                let bottom_left = bottom - 1;
                step(state, width, bottom_left, flashed);
            }
        }
    }
}
