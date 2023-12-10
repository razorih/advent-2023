use std::fmt::{Display, Debug, Write};

use advent::read_input;
use grid::Grid;

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Vertical,   // |
    Horizontal, // -
    NorthEast,  // L
    NorthWest,  // J
    SouthWest,  // 7
    SouthEast,  // F
    Ground,     // .
    Start,      // S
}

fn parse(maze: &str) -> (Grid<Tile>, (usize, usize)) {
    let cols = maze.lines().nth(0).expect("tried to parse empty maze").len();
    let mut everything: Vec<Tile> = Vec::new();
    let mut start = (0, 0);

    for (i, ch) in maze.chars().filter(|ch| !ch.is_ascii_whitespace()).enumerate() {
        let tile = Tile::try_from(ch).unwrap();
        if tile == Tile::Start {
            start = (i / cols, i % cols);
        }
        everything.push(tile);
    }

    let mut grid = Grid::from_vec(everything, cols);

    // Resolve starting tile
    grid[start] = resolve_unknown_tile(&grid, start);

    (grid, start)
}

/// Observes neighbouring tiles to determine which tile given position should be.
fn resolve_unknown_tile(maze: &Grid<Tile>, pos: (usize, usize)) -> Tile {
    debug_assert_eq!(maze[pos], Tile::Start);

    let north_open = match maze.get(pos.0.wrapping_sub(1), pos.1) {
        Some(Tile::Vertical | Tile::NorthEast | Tile::NorthWest) => true,
        _ => false,
    };

    let south_open = match maze.get(pos.0 + 1, pos.1) {
        Some(Tile::Vertical | Tile::SouthEast | Tile::SouthWest) => true,
        _ => false,
    };

    let west_open = match maze.get(pos.0, pos.1.wrapping_sub(1)) {
        Some(Tile::Horizontal | Tile::NorthEast | Tile::SouthEast) => true,
        _ => false,
    };

    let east_open = match maze.get(pos.0, pos.1 + 1) {
        Some(Tile::Horizontal | Tile::NorthWest | Tile::SouthWest) => true,
        _ => false,
    };

    match (north_open, south_open, west_open, east_open) {
        (true, true,    _,    _) => Tile::Vertical,
        (   _,    _, true, true) => Tile::Horizontal,
        (true,    _, true,    _) => Tile::NorthWest,
        (true,    _,    _, true) => Tile::NorthEast,
        (   _, true, true,    _) => Tile::SouthWest,
        (   _, true,    _, true) => Tile::SouthEast,

        _ => unreachable!("pipe {} has more than 2 openings", maze[pos])
    }
}

/// Finds two possible coordinates one can move to from this point
/// Orientation:
///    N
///    |
/// W - - E
///    |
///    S
fn get_possible_coords(maze: &Grid<Tile>, pos: (usize, usize)) -> [(usize, usize); 2] {
    match maze[pos] {
        Tile::Vertical => [
            (pos.0 + 1, pos.1),
            (pos.0 - 1, pos.1),
        ],
        Tile::Horizontal => [
            (pos.0, pos.1 + 1),
            (pos.0, pos.1 - 1),
        ],
        Tile::NorthEast => [
            (pos.0 - 1, pos.1),
            (pos.0, pos.1 + 1),
        ],
        Tile::NorthWest => [
            (pos.0 - 1, pos.1),
            (pos.0, pos.1 - 1),
        ],
        Tile::SouthWest => [
            (pos.0 + 1, pos.1),
            (pos.0, pos.1 - 1),
        ],
        Tile::SouthEast => [
            (pos.0 + 1, pos.1),
            (pos.0, pos.1 + 1),
        ],
        Tile::Start => unreachable!("starting tile should have been resolved during grid creation"),
        Tile::Ground => unreachable!("can't never be on ground"),
    }
}

fn solve(maze: &Grid<Tile>, start: (usize, usize)) -> usize {
    // Two cursors, going different directions
    // When two meet, we've reached point furthest away from the start
    let first_next = get_possible_coords(maze, start);

    let mut forwards = first_next[0];
    let mut backwards = first_next[1];
    // Coordinates we came from so we don't backtrack
    let mut last_forward = start;
    let mut last_backward = start;

    // First step has already been taken
    let mut steps: usize = 1;

    loop {
        let forwards_next = get_possible_coords(maze, forwards);
        let backwards_next = get_possible_coords(maze, backwards);

        // Make sure we don't backtrack
        if forwards_next[0] == last_forward {
            last_forward = forwards;
            forwards = forwards_next[1];
        } else {
            last_forward = forwards;
            forwards = forwards_next[0];
        }

        if backwards_next[0] == last_backward {
            last_backward = backwards;
            backwards = backwards_next[1];
        } else {
            last_backward = backwards;
            backwards = backwards_next[0];
        }

        steps += 1;

        // Advance until we meet
        if forwards == backwards {
            break;
        }
    }

    steps
}

fn main() -> anyhow::Result<()> {
    let input = read_input()?;
    let (maze, start) = parse(&input);
    print(&maze);
    println!("start: {:?}", start);

    let silver_dist = solve(&maze, start);
    println!("Silver: {}", silver_dist);

    Ok(())
}

fn print(maze: &Grid<Tile>) {
    use std::io::Write;

    // We'll be printing a single char at a time,
    // so take stdout for the whole duration.
    let mut lock = std::io::stdout().lock();

    for row in maze.iter_rows() {
        for c in row {
            let _ = write!(lock, "{}", c);
        }
        let _ = writeln!(lock);
    }
}

impl Tile {
    fn as_char(&self) -> char {
        match self {
            Tile::Horizontal => '─' /* '-' */,
            Tile::Vertical   => '│' /* '|' */,
            Tile::NorthEast  => '└' /* 'L' */,
            Tile::NorthWest  => '┘' /* 'J' */,
            Tile::SouthWest  => '┐' /* '7' */,
            Tile::SouthEast  => '┌' /* 'F' */,
            Tile::Ground     => ' ' /* '.' */,
            Tile::Start      => 'S' /* 'S' */,
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '-' => Some(Tile::Horizontal),
            '|' => Some(Tile::Vertical),
            'L' => Some(Tile::NorthEast),
            'J' => Some(Tile::NorthWest),
            '7' => Some(Tile::SouthWest),
            'F' => Some(Tile::SouthEast),
            '.' => Some(Tile::Ground),
            'S' => Some(Tile::Start),
            _   => None,
        }.ok_or_else(|| anyhow::anyhow!("invalid tile: {}", value))
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.as_char())
    }
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.as_char())
    }
}
