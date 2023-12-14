use std::collections::HashMap;

use advent::read_input;
use grid::Grid;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Tile { Empty, Round, Cube }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction { North, West, South, East }

impl Direction {
    /// Advance to next direction.
    fn next(&mut self) -> Self {
        match self {
            Self::North => Self::West,
            Self::West  => Self::South,
            Self::South => Self::East,
            Self::East  => Self::North,
        }
    }
}

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

/// Calculate next position based on `direction`.
fn next_position(pos: (usize, usize), direction: Direction) -> Option<(usize, usize)> {
    Some(match direction {
        Direction::North => (pos.0.checked_sub(1)?, pos.1),
        Direction::West  => (pos.0, pos.1.checked_sub(1)?),
        Direction::South => (pos.0 + 1, pos.1),
        Direction::East  => (pos.0, pos.1 + 1),
    })
}

struct Puzzle {
    puzzle_main: Grid<Tile>,
    puzzle_temp: Grid<Tile>,
    seen: HashMap<Vec<Tile>, usize>,
    cycle_length: usize,
    loads: Vec<usize>,
}

impl Puzzle {
    fn new(puzzle: Grid<Tile>) -> Self {
        Self {
            puzzle_main: puzzle.clone(),
            puzzle_temp: puzzle,
            seen: HashMap::new(),
            cycle_length: 0,
            loads: Vec::new(),
        }
    }

    fn get(&self) -> &Grid<Tile> {
        &self.puzzle_main
    }

    fn print(&self) {
        print_puzzle(&self.puzzle_main).unwrap();
    }

    /// Ticks boulders on the board to some [`Direction`].
    /// 
    /// Returns how many boulders were moved.
    fn tick(&mut self, direction: Direction) -> usize {
        let mut tiles_moved = 0;

        for ((row, col), _) in self.puzzle_main.indexed_iter().filter(|(_, tile)| **tile == Tile::Round) {
            // Look north for some space
            let Some((next_row, next_col)) = next_position((row, col), direction) else {
                continue; // Out of bounds, continue
            };
            
            match self.puzzle_main.get(next_row, next_col) {
                Some(Tile::Empty) => {
                    // SAFETY: `temp` is a clone of `main`, and thus has the same bounds as `main`.
                    // Nothing in this function changes those bounds.
                    // `main` (and `temp`) element access is valid for (row, col) and (next_row, next_col).
                    unsafe {
                        swap(
                            &mut self.puzzle_temp,
                            (row, col),
                            (next_row, next_col)
                        )
                    };
                    tiles_moved += 1;
                },
                _ => continue, // May be out-of-bounds or blockage
            }
        }

        // `temp` now has a board where all boulders have moved
        // Copy it into `main` in case there's a next iteration
        self.puzzle_temp.clone_into(&mut self.puzzle_main);

        tiles_moved
    }
}

impl Iterator for &mut Puzzle {
    type Item = (usize, usize, bool);

    /// Evaluates one "cycle"
    fn next(&mut self) -> Option<Self::Item> {
        let mut direction = Direction::North;
        loop {
            let tiles_moved = self.tick(direction);

            if tiles_moved == 0 {
                direction = direction.next();

                // We've moved back to start, cycle is complete
                if direction == Direction::North {
                    let load = calculate_load(self.get());
                    // Check if we have seen this before
                    // sadly, we can't get immutable ref to the underlying vec
                    // so we have to clone the whole grid
                    let oof = self.puzzle_main.clone().into_vec();

                    self.cycle_length += 1;
                    self.loads.push(load);
                    if let Some(&cached_cycle) = self.seen.get(&oof) {
                        println!("seen this grid before! at cycle {}, loop length {}", cached_cycle, self.cycle_length - cached_cycle);
                        return Some((load, cached_cycle, true));
                    } else {
                        self.seen.insert(oof, self.cycle_length);
                    }

                    return Some((load, self.cycle_length, false));
                }
            }
        }
    }
}

fn main() -> anyhow::Result<()> {
    let input = read_input()?;
    let mut puzzle = Puzzle::new(parse(&input));

    puzzle.print();

    let mut loop_start = 0;
    let mut loop_length = 0;

    for ((_, int_cycle, cycle_found), cycle) in &mut puzzle.zip(1..) {
        // println!("cycle {cycle:<5} load: {load:?}");
        if cycle_found {
            loop_start = int_cycle;
            loop_length = cycle;
            break;
        }
    }

    let offset = (1_000_000_000 - (loop_start + 1)) % (loop_length - loop_start);
    println!("offset {offset}, answer index: {}, answer: {}", loop_start + offset, puzzle.loads[loop_start + offset]);
    println!("Gold: {}", puzzle.loads[loop_start + offset]);

    Ok(())
}

fn calculate_load(puzzle: &Grid<Tile>) -> usize {
    let mut load = 0;
    let n_rows = puzzle.rows();

    for (row, load_multiplier) in puzzle.iter_rows().zip((1..=n_rows).rev()) {
        let rocks_on_row = row.filter(|&&tile| tile == Tile::Round).count();
        load += rocks_on_row * load_multiplier;
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
