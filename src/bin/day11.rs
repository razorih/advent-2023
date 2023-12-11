use advent::read_input;
use grid::Grid;


fn parse(s: &str) -> Vec<(usize, usize)> {
    let mut grid: Vec<char> = Vec::new();

    let cols = s.lines().nth(0).unwrap().len();
    for line in s.trim().lines() {
        grid.extend(line.chars())
    }

    let mut grid = Grid::from_vec(grid, cols);

    fn galaxy_filter<'a, T: Iterator<Item = &'a char> + Clone>(
        (col, content): (usize, T)
    ) -> Option<usize> {
        if content.clone().all(|&ch| ch == '.') {
            Some(col)
        } else {
            None
        }
    }
    // Expand columns and rows
    let empty_cols: Vec<usize> = grid.iter_cols()
        .enumerate().filter_map(galaxy_filter).collect();

    let empty_rows: Vec<usize> = grid.iter_rows()
        .enumerate().filter_map(galaxy_filter).collect();

    println!("empty cols: {empty_cols:?}, rows: {empty_rows:?}");

    // The `fix` term below accounts for the fact that inserting a column/row
    // pushes later columns'/rows' indices by one

    let n_rows = grid.rows();
    for (col, fix) in empty_cols.into_iter().zip(0_usize..) {
        grid.insert_col(col+fix, vec!['.'; n_rows]);
    }

    let n_cols = grid.cols(); // changed from expanding columns
    for (row, fix) in empty_rows.into_iter().zip(0_usize..) {
        grid.insert_row(row+fix, vec!['.'; n_cols]);
    }

    // Galaxy has been expanded, return galaxy coordinates
    Vec::from_iter(
        grid.indexed_iter()
            .filter_map(|(coords, &ch)| if ch == '#' { Some(coords) } else { None })
    )
}

/// Distance function measuring distance between two galaxies.
/// In this case, L_1 norm.
fn dist((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> usize {
    x1.abs_diff(x2) + y1.abs_diff(y2)
}

fn main() -> anyhow::Result<()> {
    let input = read_input()?;
    let galaxies = parse(&input);

    let mut silver_sum = 0;
    for i in 0..galaxies.len() {
        for j in i..galaxies.len() {
            if i == j {
                continue;
            }

            let dist = dist(galaxies[i], galaxies[j]);
            silver_sum += dist;
            // println!("{i} -> {j} dist: {}", dist)
        }
    }

    println!("Silver: {:?}", silver_sum);

    Ok(())
}
