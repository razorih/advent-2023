use advent::read_input;
use grid::Grid;


fn parse<const GALAXY_SIZE: usize>(s: &str) -> Vec<(usize, usize)> {
    let mut grid: Vec<char> = Vec::new();

    let cols = s.lines().nth(0).unwrap().len();
    for line in s.trim().lines() {
        grid.extend(line.chars())
    }

    let grid = Grid::from_vec(grid, cols);

    fn galaxy_filter<'a, T: Iterator<Item = &'a char> + Clone>(
        (col, content): (usize, T)
    ) -> Option<usize> {
        if content.clone().all(|&ch| ch == '.') {
            Some(col)
        } else {
            None
        }
    }

    // Find all empty column and row indices.
    // Resulting index arrays are sorted.
    let empty_cols: Vec<usize> = grid.iter_cols()
        .enumerate().filter_map(galaxy_filter).collect();
    let empty_rows: Vec<usize> = grid.iter_rows()
        .enumerate().filter_map(galaxy_filter).collect();

    println!("empty cols: {empty_cols:?}, rows: {empty_rows:?}");

    // Collect a list of unexpanded galaxies
    let mut galaxies = Vec::from_iter(
        grid.indexed_iter()
            .filter_map(|(coords, &ch)| if ch == '#' { Some(coords) } else { None })
    );

    // Expand galaxies using a "scanline".
    // Columns and rows are expanded separately.
    //
    // Loop through all empty column/row and check if any galaxies appears after them.
    // If so, move it forward accordingly.
    // 
    // Since moving a galaxy also moves following empty galaxies, we have to keep
    // a counter (`fix`), which is used to calculate the "true" position
    // where galaxy should be expanded.
    
    for (empty, fix) in empty_cols.into_iter().zip(0_usize..) {
        for galaxy in &mut galaxies {
            if galaxy.1 >= empty + fix*GALAXY_SIZE {
                galaxy.1 = galaxy.1 + GALAXY_SIZE;
            }
        }
    }

    for (empty, fix) in empty_rows.into_iter().zip(0_usize..) {
        for galaxy in &mut galaxies {
            if galaxy.0 >= empty + fix*GALAXY_SIZE {
                galaxy.0 = galaxy.0 + GALAXY_SIZE;
            }
        }
    }

    galaxies
}

/// Distance function measuring distance between two galaxies.
/// In this case, L_1 norm.
fn dist((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> usize {
    x1.abs_diff(x2) + y1.abs_diff(y2)
}

fn main() -> anyhow::Result<()> {
    let input = read_input()?;
    let galaxies = parse::<999_999>(&input);

    let mut sum = 0;
    for i in 0..galaxies.len() {
        for j in i..galaxies.len() {
            if i == j {
                continue;
            }

            let dist = dist(galaxies[i], galaxies[j]);
            sum += dist;
            // println!("{i} -> {j} dist: {}", dist)
        }
    }

    println!("Gold: {:?}", sum);

    Ok(())
}
