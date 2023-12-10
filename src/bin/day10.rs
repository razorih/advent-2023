use std::{fmt::{Display, Debug, Write}, collections::HashSet};

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

fn solve(maze: &Grid<Tile>, start: (usize, usize)) -> Vec<(usize, usize)> {
    let first_next = get_possible_coords(maze, start);
    let mut cursor = first_next[0];

    // Coordinate we came from so we don't backtrack
    let mut last_cursor = start;

    // Keep a list of coordinates we stepped on,
    // this will form a list of all points along the shape's edge
    let mut steps = vec![start];

    loop {
        let cursor_candidates = get_possible_coords(maze, cursor);

        // Decide next position, taking care we don't backtrack
        let next_cursor = if cursor_candidates[0] == last_cursor {
            cursor_candidates[1]
        } else {
            cursor_candidates[0]
        };

        last_cursor = cursor;
        steps.push(cursor);
        cursor = next_cursor;

        // Advance until we loop back to start
        if cursor == start {
            break;
        }
    }

    steps
}

/// Calculate signed area of a polygon given its vertices.
fn shoelace(vertices: &[(usize, usize)]) -> isize {
    /// Calculates determinant of 2x2 matrix formed from two points
    /// | x1  x2 |
    /// | y1  y2 |
    fn det((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> isize {
        x1 as isize * y2 as isize - x2 as isize * y1 as isize
    }

    let mut incomplete_sum: isize = 0;
    for pair in vertices.windows(2) {
        incomplete_sum += det(pair[0], pair[1]);
    }

    let complete_sum = incomplete_sum + det(vertices[vertices.len() - 1], vertices[0]);
    complete_sum / 2
}

/// Calculate number of interior points using Pick's theorem.
/// Area MUST have been derived from a polygon with discrete vertex coordinates.
fn n_interior_points(area: isize, n_boundary_points: isize) -> isize {
    area.abs() - (n_boundary_points / 2) + 1
}

fn main() -> anyhow::Result<()> {
    let input = read_input()?;
    let (maze, start) = parse(&input);

    let path = solve(&maze, start);
    let area = shoelace(&path);

    print(&maze, &HashSet::from_iter(path.iter().cloned()));

    // Distance to furthest point along the edge is edge length / 2
    println!("Silver: {}", path.len() / 2);
    println!("Gold:   {}", n_interior_points(area, path.len() as isize));

    Ok(())
}

fn print(maze: &Grid<Tile>, path: &HashSet<(usize, usize)>) {
    const RED: &str = "\x1B[31m";
    const GREEN: &str = "\x1B[32m";
    const RESET: &str = "\x1B[0m";

    use std::io::Write;

    // We'll be printing a single char at a time,
    // so take stdout for the whole duration.
    let mut lock = std::io::stdout().lock();

    for (i, row) in maze.iter_rows().enumerate() {
        for (j, c) in row.enumerate() {
            if path.contains(&(i, j)) {
                let _ = write!(lock, "{GREEN}{}", c);
            } else {
                let _ = write!(lock, "{RED}{}", c);
            }
        }
        let _ = writeln!(lock, "{RESET}");
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
