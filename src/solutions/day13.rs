use super::{ds::Grid, utils};

pub fn solution() {
    const INITIAL_GRID_WIDTH: u32 = 1500;
    let input: String = utils::read_from_file("files/day13_input.txt");
    let sections: Vec<&str> = input.split("\n\n").collect::<Vec<&str>>();

    let mut grid: Grid<u32> = Grid::new(INITIAL_GRID_WIDTH, 0);

    let positions_text = sections[0];
    let positions: Vec<&str> = positions_text.split('\n').collect();

    for &curr_position in positions.iter() {
        let position: Vec<&str> = curr_position.split(',').collect();
        let x = position[0].parse::<usize>().unwrap();
        let y = position[1].parse::<usize>().unwrap();
        grid.set(x, y, 1);
    }

    // folds!
    let folds: Vec<&str> = sections[1].split('\n').collect();

    let mut is_first_iter = false;
    for &curr_grouped_fold in folds.iter() {
        let curr_fold: Vec<&str> = curr_grouped_fold.split('=').collect();
        let axis = curr_fold[0]
            .chars()
            .nth(curr_fold[0].chars().count() - 1)
            .unwrap();
        let fold_at = curr_fold[1].parse::<usize>().unwrap();

        if axis.eq(&'x') {
            fold_grid_along_x(&mut grid, fold_at);
        } else {
            fold_grid_along_y(&mut grid, fold_at);
        }

        if !is_first_iter {
            let dots = grid.positions.iter().filter(|&x| *x == 1).count();
            println!("Answer to day 13 part 1: {}", dots);
            is_first_iter = true;
        }
    }

    println!("Answer to day 13 part 2:");
    grid.dump();
}

pub fn fold_grid_along_y(grid: &mut Grid<u32>, at: usize) {
    let rows = grid.positions.len() / grid.width as usize;
    if at > rows {
        panic!("Fold along x at Invalid position");
    }

    for cut_position in 1..(rows - at) {
        // This means that the next positions will out of the current grid
        if cut_position > at {
            break;
        }

        let below_y_position = at + cut_position;
        let above_y_position = at - cut_position;

        for x in 0..grid.width as usize {
            let value = grid.get(x, below_y_position) | grid.get(x, above_y_position);
            grid.set(x, above_y_position, value);
        }
    }

    grid.positions = grid.positions[..(grid.width as usize * at)].to_vec();
}

pub fn fold_grid_along_x(grid: &mut Grid<u32>, at: usize) {
    let columns = grid.width as usize;
    let rows = grid.positions.len() / grid.width as usize;
    if at > grid.width as usize {
        panic!("Fold along y at Invalid position");
    }

    for cut_position in 1..(columns - at) {
        // This means that the next positions will out of the current grid
        if cut_position > at {
            break;
        }

        let left_x_position = at + cut_position;
        let right_x_position = at - cut_position;

        for y in 0..rows {
            let value = grid.get(left_x_position, y) | grid.get(right_x_position, y);
            grid.set(right_x_position, y, value);
        }
    }

    let mut new_pos: Vec<u32> = Vec::new();
    // Re-adjust grid positions
    for i in 0..rows {
        let start = i * (grid.width as usize);
        new_pos.append(&mut grid.positions[start..start + at].to_vec());
    }

    grid.positions = new_pos;
    grid.width = at as u32;
}
