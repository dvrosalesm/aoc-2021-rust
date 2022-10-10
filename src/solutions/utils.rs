use std::{
    fmt::{Debug, Display},
    fs,
};

pub trait Extensions {
    fn to_vec<T: num_traits::Num>(&self, sep: &str) -> Vec<T>
    where
        <T as num_traits::Num>::FromStrRadixErr: Debug;
}

impl Extensions for str {
    fn to_vec<T: num_traits::Num>(&self, sep: &str) -> Vec<T>
    where
        <T as num_traits::Num>::FromStrRadixErr: Debug,
    {
        return self
            .split(sep)
            .collect::<Vec<&str>>()
            .iter()
            .map(|&x| T::from_str_radix(x, 10).unwrap())
            .collect::<Vec<T>>();
    }
}

pub fn read_from_file(filename: &str) -> String {
    fs::read_to_string(filename).expect("Something went wrong while reading the file")
}

#[allow(dead_code)]
pub fn display_grid<T>(grid: &[T], width: &usize)
where
    T: Display,
{
    for (i, pos) in grid.iter().enumerate() {
        if i % width == 0 {
            println!();
        }
        print!("{}", pos);
    }
    println!();
}
