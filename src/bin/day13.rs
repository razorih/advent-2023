use advent::read_input;
use grid::Grid;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile { Ash, Rock }

#[derive(Debug)]
enum Reflection {
    Column(usize),
    Row(usize),
}

fn solve(pattern: &Grid<Tile>) -> Reflection {
    let (rows, cols) = pattern.size();

    // Look at all neighboring columns and rows and check if we cant start mirror there
    // Returned `pivot` here is already corrected for the 1-based indexing

    for (i, j) in (0..cols-1).zip(1..cols) {
        if let Some(pivot) = check_expanding(|col| pattern.iter_col(col), i, j, pattern.cols()-1) {
            println!("found mirror at col pair {i}-{j}, pivot: {pivot:?}");
            return Reflection::Column(pivot)
        }

    }

    for (i, j) in (0..rows-1).zip(1..rows) {
        if let Some(pivot) = check_expanding(|row| pattern.iter_row(row), i, j, pattern.rows()-1) {
            println!("found mirror at row pair {i}-{j}, pivot: {pivot:?}");
            return Reflection::Row(pivot)
        }
    }

    unreachable!("all patterns should have one mirror");
}

fn main() -> anyhow::Result<()> {
    let input = read_input()?;
    let patterns = parse_patterns(&input);

    let mut sum: usize = 0;
    for pattern in patterns {
        print(&pattern);
        match solve(&pattern) {
            Reflection::Column(n) => sum += n,
            Reflection::Row(n)    => sum += n*100,
        }
    }

    println!("Silver: {}", sum);

    Ok(())
}

fn parse_patterns(s: &str) -> Vec<Grid<Tile>> {
    let mut out: Vec<Grid<Tile>> = Vec::new();
    let mut builder: Vec<Tile> = Vec::new();


    let mut cols = s.lines().next().expect("empty input").len();
    for line in s.trim().lines() {
        if line.is_empty() {
            out.push(Grid::from_vec(builder.clone(), cols));
            builder.clear();
        }
        cols = line.len();
        builder.extend(line.chars().filter_map(Tile::from_char));
    }

    if !builder.is_empty() {
        out.push(Grid::from_vec(builder, cols));
    }

    out
}

fn print(pattern: &Grid<Tile>) {
    use std::io::Write;
    let mut lock = std::io::stdout().lock();

    let cols = pattern.cols();

    let _ = write!(lock, "╭{:─<cols$}╮\n", "");
    for row in pattern.iter_rows() {
        let _ = write!(lock, "│");
        for ch in row {
            let _ = write!(lock, "{}", ch);
        }
        let _ = write!(lock, "│");
        let _ = write!(lock, "\n");
    }
    let _ = write!(lock, "╰{:─<cols$}╯\n", "");
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        match self {
            Tile::Ash => f.write_char('.'),
            Tile::Rock => f.write_char('#'),
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        match self {
            Tile::Ash => f.write_char(' '),
            Tile::Rock => f.write_char('█'),
        }
    }
}

impl Tile {
    fn from_char(ch: char) -> Option<Self> {
        match ch {
            '.' => Some(Self::Ash),
            '#' => Some(Self::Rock),
            _   => None
        }
    }
}

/// Check for reflection by iteratively expanding two indices.
/// 
/// Example how `i` and `j` move, each column (or similarly a row) must match.
/// in order for the iterator to continue. If either `i` or `j` reach array 
/// bounds, the array has a mirror which's pivot is at the original `i` index.
/// ```not_rust
///     ij
/// #.##..##.
/// ---------
///    i  j
/// #.##..##.
/// ---------
///   i    j
/// #.##..##.
/// ```
fn check_expanding<F, I>(
    source: F,
    mut i: usize,
    mut j: usize,
    max_j: usize,
) -> Option<usize>
where
    F: Fn(usize) -> I,
    I: Iterator,
    I::Item: PartialEq,
{
    let pivot = i;
    let res = loop {
        // Check for reflection between arrays given by i and j
        if !source(i).eq(source(j)) {
            break None
        }

        // If we would go out-of-bounds next iteration, this is a mirror
        if i == 0 || j == max_j {
            break Some(pivot + 1)
        }

        // Otherwise, expand search and repeat
        i -= 1;
        j += 1;
    };

    res
}
