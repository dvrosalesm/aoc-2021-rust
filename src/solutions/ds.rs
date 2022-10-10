use std::{cell::RefCell, fmt::Debug, fmt::Display, mem::replace, rc::Rc, str::FromStr};

use super::utils::display_grid;

pub struct Node<T> {
    pub value: T,
    pub children: Vec<Rc<RefCell<Node<T>>>>,
}

pub struct Grid<T> {
    pub width: u32,
    pub positions: Vec<T>,
}

#[allow(dead_code)]
impl<T: FromStr> Grid<T> {
    pub fn new(width: u32, default_value: T) -> Self
    where
        T: Clone,
    {
        let mut positions: Vec<T> = Vec::new();
        positions.resize((width * width) as usize, default_value);

        Grid { width, positions }
    }

    pub fn from_str(positions: &str, width: u32) -> Self
    where
        <T as FromStr>::Err: Debug,
    {
        let single_positions: Vec<&str> = positions.split(',').collect();
        let mut positions_vec: Vec<T> = Vec::new();

        for curr_pos in single_positions.iter() {
            positions_vec.push(FromStr::from_str(curr_pos).unwrap());
        }

        Grid {
            width,
            positions: positions_vec,
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) -> T {
        replace(&mut self.positions[(y * self.width as usize) + x], value)
    }

    pub fn get(&self, x: usize, y: usize) -> &T {
        &self.positions[(y * self.width as usize) + x]
    }

    pub fn dump(&self)
    where
        T: Display,
    {
        display_grid(&self.positions, &(self.width as usize))
    }
}
