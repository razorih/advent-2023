use std::collections::{HashSet, VecDeque};

use advent::read_input;
use grid::Grid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    ForwardMirror,
    BackwardMirror,
    VertSplit,
    HorSplit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir { Up, Down, Left, Right, }

/// Represents a collision between a [`Beam`] and grid tile
enum Collision<'a> {
    /// Beam hit grid edge and dies out
    Death,
    /// Beam continues unchanged
    Continue(Beam<'a>),
    /// Beam is reflected by a mirror.
    Reflection(Beam<'a>),
    /// Beam is split into two beams.
    Split(Beam<'a>, Beam<'a>),
}

#[derive(Debug, Clone, Copy)]
struct Beam<'a> {
    col: usize,
    row: usize,
    direction: Dir,
    grid: &'a Grid<Tile>
}

impl<'a> Beam<'a> {
    /// Creates a new beam at some position inside a grid.
    /// 
    /// Panic if location is out of grid bounds.
    fn new_in_grid(
        col: usize,
        row: usize,
        direction: Dir,
        grid: &'a Grid<Tile>
    ) -> Self {
        if col >= grid.cols() || row >= grid.rows() {
            panic!("initial beam position is outside of grid boundaries");
        }

        Self { col, row, direction, grid }
    }

    /// Get the tile beam is currently on
    fn tile(&self) -> Tile {
        self.grid[(self.row, self.col)]
    }

    /// Consumes the beam and tries to move it to given direction.
    ///
    /// Returns [`None`] if beam goes out of grid bounds.
    fn moved_to_direction(self, direction: Dir) -> Option<Self> {
        let (col, row) = match direction {
            Dir::Up    => (self.col, self.row.checked_sub(1)?),
            Dir::Down  => (self.col, self.row + 1),
            Dir::Left  => (self.col.checked_sub(1)?, self.row),
            Dir::Right => (self.col + 1, self.row),
        };

        if col >= self.grid.cols() || row >= self.grid.rows() {
            return None
        }

        Some(Self { col, row, direction, grid: self.grid })
    }

    /// Copies current beam and moves it to some direction.
    fn copied_to_direction(&self, direction: Dir) -> Option<Self> {
        self.clone().moved_to_direction(direction)
    }

    /// Get beam's current position and direction
    fn position(&self) -> (usize, usize, Dir) {
        (self.col, self.row, self.direction)
    }

    /// Collides beam with a tile in its current position.
    /// 
    /// Returned [`Collision`] object contains beam's next state, if any.
    fn collide(self) -> Collision<'a> {
        // We're "inside" a mirror
        match self.tile() {
            Tile::ForwardMirror => { // '/'
                let next_direction = match self.direction {
                    Dir::Up    => Dir::Right,
                    Dir::Down  => Dir::Left,
                    Dir::Left  => Dir::Down,
                    Dir::Right => Dir::Up,
                };

                if let Some(reflected_beam) = self.moved_to_direction(next_direction) {
                    Collision::Reflection(reflected_beam)
                } else {
                    Collision::Death
                }
            },
            Tile::BackwardMirror => { // '\'
                let next_direction = match self.direction {
                    Dir::Up    => Dir::Left,
                    Dir::Down  => Dir::Right,
                    Dir::Left  => Dir::Up,
                    Dir::Right => Dir::Down,
                };

                if let Some(reflected_beam) = self.moved_to_direction(next_direction) {
                    Collision::Reflection(reflected_beam)
                } else {
                    Collision::Death
                }
            },
            Tile::VertSplit => { // '|'
                match self.direction {
                    Dir::Up | Dir::Down => {
                        if let Some(beam) = self.moved_to_direction(self.direction) {
                            Collision::Continue(beam)
                        } else {
                            Collision::Death
                        }
                    },
                    Dir::Left | Dir::Right => {
                        let up = self.copied_to_direction(Dir::Up);
                        let down = self.moved_to_direction(Dir::Down);

                        match (up, down) {
                            (None, Some(beam)) => Collision::Reflection(beam),
                            (Some(beam), None) => Collision::Reflection(beam),
                            (Some(up), Some(down)) => Collision::Split(up, down),
                            (None, None) => Collision::Death,
                        }
                    }
                }
            },
            Tile::HorSplit => { // '-'
                match self.direction {
                    Dir::Left | Dir::Right => {
                        if let Some(beam) = self.moved_to_direction(self.direction) {
                            Collision::Continue(beam)
                        } else {
                            Collision::Death
                        }
                    },
                    Dir::Up | Dir::Down => {
                        let left = self.copied_to_direction(Dir::Left);
                        let right = self.moved_to_direction(Dir::Right);

                        match (left, right) {
                            (None, Some(beam)) => Collision::Reflection(beam),
                            (Some(beam), None) => Collision::Reflection(beam),
                            (Some(left), Some(right)) => Collision::Split(left, right),
                            (None, None) => Collision::Death,
                        }
                    }
                }
            },
            Tile::Empty => {
                if let Some(beam) = self.moved_to_direction(self.direction) {
                    Collision::Continue(beam)
                } else {
                    Collision::Death
                }
            },
        }
    }
}

fn solve(start: Beam) -> usize {
    // List of beams, a new beam gets added each time a splitter is encountered.
    // Beams may also "die out" if they hit grid edges
    let mut beams: VecDeque<Beam> = vec![start.clone()].into();
    // Set of seen (energized) tiles. As each tile can be energized only once,
    // but beams can collide with some tiles multiple times
    // we can't keep a simple running number but need to record each unique tiles.
    let mut seen: HashSet<(usize, usize, Dir)> = HashSet::new();
    seen.insert(start.position());

    while let Some(beam) = beams.pop_front() {
        match beam.collide() {
            Collision::Continue(beam) => {
                seen.insert(beam.position());
                beams.push_back(beam);
            },
            Collision::Reflection(beam) => {
                seen.insert(beam.position());
                beams.push_back(beam);
            },
            Collision::Split(first, second) => {
                // Check if we are going to loop
                // i.e. we are revisiting a point from same angle
                if !seen.contains(&first.position()) {
                    seen.insert(first.position());
                    beams.push_back(first);
                }

                if !seen.contains(&second.position()) {
                    seen.insert(second.position());
                    beams.push_back(second);
                }
            },
            Collision::Death => {
                // println!("beam died!");
            },
        }
    }

    // Hacky way to count visited points without directions
    seen.into_iter()
        .map(|(col, row, _)| (col, row))
        .collect::<HashSet<(usize, usize)>>()
        .len()
}

fn gold(puzzle: &Grid<Tile>) -> usize {
    let (rows, cols) = puzzle.size();
    let mut max: usize = 0;

    for col in 0..cols {
        let downwards_beam = Beam::new_in_grid(col, 0, Dir::Down, &puzzle);
        let upwards_beam = Beam::new_in_grid(col, rows-1, Dir::Up, &puzzle);

        let tiles = solve(downwards_beam);
        if tiles > max {
            max = tiles;
        }

        let tiles = solve(upwards_beam);
        if tiles > max {
            max = tiles;
        }
    }

    for row in 0..rows {
        let rightward_beam = Beam::new_in_grid(0, row, Dir::Right, &puzzle);
        let leftward_beam = Beam::new_in_grid(cols-1, row, Dir::Left, &puzzle);

        let tiles = solve(rightward_beam);
        if tiles > max {
            max = tiles;
        }
        let tiles = solve(leftward_beam);
        if tiles > max {
            max = tiles;
        }
    }

    max
}

fn main() -> anyhow::Result<()> {
    let input = read_input()?;
    let puzzle = parse(&input);

    let energized_tiles = solve(Beam::new_in_grid(0, 0, Dir::Right, &puzzle));
    println!("Silver: {}", energized_tiles);

    let maxed = gold(&puzzle);
    println!("  Gold: {}", maxed);

    Ok(())
}

fn parse(s: &str) -> Grid<Tile> {
    let mut tiles = Vec::new();
    let cols = s.lines().next().expect("got empty input").len();
    
    for tile in s.chars().filter(|ch| !ch.is_ascii_whitespace()) {
        tiles.push(Tile::from_char(tile));
    }

    Grid::from_vec(tiles, cols)
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        match self {
            Tile::Empty => f.write_char('.'),
            Tile::ForwardMirror => f.write_char('/'),
            Tile::BackwardMirror => f.write_char('\\'), 
            Tile::VertSplit => f.write_char('|'), 
            Tile::HorSplit => f.write_char('-'),
        }
    }
}

impl Tile {
    fn from_char(tile: char) -> Tile {
        match tile {
            '.' => Self::Empty,
            '/' => Self::ForwardMirror,
            '\\' => Self::BackwardMirror,
            '|' => Self::VertSplit,
            '-' => Self::HorSplit,
            _ => panic!("invalid tile '{tile}'"),
        }
    }
}
