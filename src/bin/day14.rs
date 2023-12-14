use advent::read_input;
use grid::Grid;


#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile { Empty, Round, Cube }

/// Swaps two elements of given grid.
/// 
/// Caller must guarantee that reads and writes to
/// `grid[(x.0, x.1)]` and `grid[(y.0, y.1)]` are valid.
unsafe fn swap(grid: &mut Grid<Tile>, x: (usize, usize), y: (usize, usize)) {
    unsafe {
        let ptr_a: *mut Tile = grid.get_unchecked_mut(x.0, x.1);
        let ptr_b: *mut Tile = grid.get_unchecked_mut(y.0, y.1);
        std::ptr::swap(ptr_a, ptr_b);
    }
}

fn tick(grid: Grid<Tile>) -> (Grid<Tile>, usize) {
    let mut out = grid.clone();
    let mut tiles_moved = 0;

    for ((row, col), &tile) in grid.indexed_iter() {
        if tile == Tile::Round {
            // Look north for some space
            let Some(north_row) = row.checked_sub(1) else {
                continue; // Out of bounds, continue
            };
            
            match grid.get(north_row, col) {
                Some(Tile::Empty) => {
                    // SAFETY: `out` is a clone of `grid`, and thus has the same bounds as `grid`.
                    // Nothing in this function changes those bounds.
                    // `grid` (and `out`) element access is valid for (row, col) and (row-1, col).
                    unsafe { swap(&mut out, (row, col), (row-1, col)) };
                    tiles_moved += 1;
                },
                _ => continue, // May be out-of-bounds or blockage
            }
        }
    }

    (out, tiles_moved)
}

fn main() -> anyhow::Result<()> {
    let input = read_input()?;
    let mut puzzle = parse(&input);

    print_puzzle(&puzzle)?;
    loop {
        let (_puzzle, tiles_moved) = tick(puzzle);
        puzzle = _puzzle;
        println!("moved {tiles_moved} tiles");

        if tiles_moved == 0 {
            break;
        }
    }
    print_puzzle(&puzzle)?;
    println!("Silver: {}", calculate_load(&puzzle));

    Ok(())
}

fn calculate_load(puzzle: &Grid<Tile>) -> usize {
    let mut load = 0;
    let rows = puzzle.rows();

    for (row, load_multiplier) in puzzle.iter_rows().zip((1..=rows).rev()){
        for &tile in row {
            if tile == Tile::Round {
                load += load_multiplier;
            }
        }
    }

    load
}

impl Tile {
    fn from_char(ch: char) -> Self {
        match ch {
            'O' => Self::Round,
            '#' => Self::Cube,
            '.' => Self::Empty,
            _ => panic!("invalid tile"),
        }
    }
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        match self {
            Tile::Empty => f.write_char('.'),
            Tile::Round => f.write_char('O'),
            Tile::Cube => f.write_char('#'),
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        match self {
            Tile::Empty => f.write_char('·'),
            Tile::Round => f.write_char('◯'),
            Tile::Cube => f.write_char('▆'),
        }
    }
}

fn parse(input: &str) -> Grid<Tile> {
    let cols = input.lines().next().unwrap().len();
    let buffer: Vec<Tile> = input.chars().filter_map(|ch| if !ch.is_ascii_whitespace() {
        Some(Tile::from_char(ch))
    } else {
        None
    }).collect();

    Grid::from_vec(buffer, cols)
}

fn print_puzzle(grid: &Grid<Tile>) -> std::io::Result<()> {
    use std::io::Write;

    let mut lock = std::io::stdout().lock();
    for row in grid.iter_rows() {
        for tile in row {
            write!(lock, "{tile}")?;
        }
        write!(lock, "\n")?;
    }
    write!(lock, "\n\n")?;

    Ok(())
}
