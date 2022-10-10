use std::cmp;
use std::fmt;

use super::utils;

struct Grid {
    width: i32,
    height: i32,
    cells: Vec<i32>,
}

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for i in 0..self.width as usize {
            for j in 0..self.height as usize {
                if self.cells[(i * self.width as usize) + j] == 0 {
                    s.push('.');
                } else {
                    s.push_str(
                        self.cells[(i * self.width as usize) + j]
                            .to_string()
                            .as_str(),
                    );
                }
            }
            s.push('\n');
        }

        write!(f, "{}", s)
    }
}

impl Grid {
    fn new(width: i32, height: i32) -> Grid {
        let mut grid = Grid {
            width,
            height,
            cells: Vec::new(),
        };
        grid.cells.resize((grid.height * grid.width) as usize, 0);

        grid
    }

    fn draw_fixed_axis_line(&mut self, a: &Point, b: &Point) {
        if a.x == b.x {
            let distance = (a.y - b.y).abs() + 1;
            let min = cmp::min(a.y, b.y);

            for i in min..min + distance {
                let idx = ((i * self.width) + a.x) as usize;
                self.cells[idx] += 1;
            }
        } else if a.y == b.y {
            let distance = (a.x - b.x).abs() + 1;
            let min = cmp::min(a.x, b.x);

            for i in min..(min + distance) {
                let idx = ((a.y * self.width) + i) as usize;
                self.cells[idx] += 1;
            }
        }
    }

    fn draw_line_with_diagonal(&mut self, a: &Point, b: &Point) {
        if a.x == b.x || a.y == b.y {
            self.draw_fixed_axis_line(a, b)
        } else {
            // Only diagonal needed! (Meaning right triangle)
            let start_point;
            let end_point;

            if a.x < b.x {
                start_point = a;
                end_point = b;
            } else {
                start_point = b;
                end_point = a;
            }

            for i in 0..(end_point.x - start_point.x) + 1 {
                let next_x = start_point.x + i;
                let next_y = if start_point.y > end_point.y {
                    start_point.y - i
                } else {
                    start_point.y + i
                };

                self.cells[((next_y * self.width) + next_x) as usize] += 1;
            }
        }
    }

    fn overlapped_lines(&self) -> i32 {
        let mut sum = 0;

        for i in 0..self.width as usize {
            for j in 0..self.height as usize {
                if self.cells[(i * self.width as usize) + j] > 1 {
                    sum += 1;
                }
            }
        }

        sum
    }
}

impl Point {
    fn from_str(desc: &str) -> Point {
        let points: Vec<&str> = desc.split(',').collect();
        Point {
            x: points[0].parse::<i32>().unwrap(),
            y: points[1].parse::<i32>().unwrap(),
        }
    }
}

pub fn solution() {
    let input = utils::read_from_file("files/day5_input.txt");
    let splitted_lines: Vec<&str> = input.split('\n').collect();

    // Looks like my input won't go over 1000
    // TODO: Add dynamic init for the grid?
    let mut grid = Grid::new(1000, 1000);
    let mut grid_w_diagonal = Grid::new(1000, 1000);

    for line in splitted_lines {
        let points: Vec<&str> = line.split(" -> ").collect();
        let point_a = Point::from_str(points[0]);
        let point_b = Point::from_str(points[1]);

        grid.draw_fixed_axis_line(&point_a, &point_b);
        grid_w_diagonal.draw_line_with_diagonal(&point_a, &point_b);
    }

    println!("Answer to day 5 part 1: {}", grid.overlapped_lines());
    println!(
        "Answer to day 5 part 2: {}",
        grid_w_diagonal.overlapped_lines()
    );
}
