use super::utils;

pub fn solution() {
    let input = utils::read_from_file("files/day9_input.txt");
    let lines: Vec<&str> = input.split('\n').collect();
    let width = lines[0].len();
    let mut sum_of_low: u32 = 0;

    let locations: Vec<u8> = lines
        .join("")
        .chars()
        .map(|c| c.to_string().as_str().parse::<u8>().unwrap())
        .collect();

    let mut low_points: Vec<usize> = Vec::new();

    for i in 0..locations.len() {
        let mut is_low_point = true;

        if i >= width && locations[i] >= locations[i - width] {
            is_low_point = false;
        }

        if i + width < locations.len() && locations[i] >= locations[i + width] {
            is_low_point = false;
        }

        if i > 0 && i % width != 0 && locations[i] >= locations[i - 1] {
            is_low_point = false;
        }

        if (i + 1) % width != 0 && locations[i] >= locations[i + 1] {
            is_low_point = false;
        }

        if is_low_point {
            low_points.push(i);
            sum_of_low += locations[i] as u32 + 1;
        }
    }

    let mut all_basins: Vec<u32> = Vec::new();

    for low_point in low_points {
        let mut basins: Vec<usize> = vec![low_point];

        let mut on_found_fn = |basin_loc: usize| {
            if !basins.contains(&basin_loc) && locations[basin_loc] != 9 {
                basins.push(basin_loc);
                return true;
            }
            false
        };

        find_basins(&locations, width, low_point, &mut on_found_fn);

        all_basins.push(basins.len() as u32);
    }

    all_basins.sort_by(|a, b| b.cmp(a));

    let mut mul_top_3_basins = 1;
    all_basins[0..3].iter().for_each(|len: &u32| {
        mul_top_3_basins *= len;
    });

    println!("Answer to day 9 part 1: {}", sum_of_low);
    println!("Answer to day 9 part 2: {}", mul_top_3_basins);
}

fn find_basins<F>(locations: &Vec<u8>, width: usize, point: usize, on_found: &mut F)
where
    F: FnMut(usize) -> bool,
{
    // I'm not checking current point since its assumed that was already visited and added to the visited vec
    if point >= width && locations[point] < locations[point - width] && on_found(point - width) {
        find_basins(locations, width, point - width, on_found)
    }

    if point + width < locations.len()
        && locations[point] < locations[point + width]
        && on_found(point + width)
    {
        find_basins(locations, width, point + width, on_found)
    }

    if point > 0
        && point % width != 0
        && locations[point] < locations[point - 1]
        && on_found(point - 1)
    {
        find_basins(locations, width, point - 1, on_found)
    }

    if point + 1 < locations.len()
        && point + 1 % width != 0
        && locations[point] < locations[point + 1]
        && on_found(point + 1)
    {
        find_basins(locations, width, point + 1, on_found)
    }
}
