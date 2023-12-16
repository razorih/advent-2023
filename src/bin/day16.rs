use std::collections::{HashSet, VecDeque};

use advent::{read_input, print_grid};
use grid::Grid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty, // .
    ForwardMirror, // /
    BackwardMirror, // \
    VertSplit, // |
    HorSplit,  // -
}
impl Tile {
    fn from_char(tile: char) -> Tile {
        match tile {
            '.' => Self::Empty,
            '/' => Self::ForwardMirror,
            '\\' => Self::BackwardMirror,
            '|' => Self::VertSplit,
            '-' => Self::HorSplit,
            _ => panic!("invalid tile '0x{:x}'", tile as u32),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    #[default]
    Right, // "beam starts moving right"
}

/// Represents a collision between a [`Beam`] and some grid on tiles.
enum Collision {
    /// Beam hit grid edge and dies out
    Death,
    /// Beam continues unchanged
    Continue(Beam),
    /// Beam is reflected by a mirror.
    Reflection(Beam),
    /// Beam is split into two beams.
    Split(Beam, Beam),
}

#[derive(Debug, Default, Clone, Copy)]
struct Beam {
    col: usize,
    row: usize,
    direction: Dir,
}

impl Beam {
    fn new(col: usize, row: usize, direction: Dir) -> Self {
        Self { col, row, direction }
    }

    fn position(&self) -> (usize, usize, Dir) {
        (self.col, self.row, self.direction)
    }

    /// Calculates beam's position next tick along current direction.
    /// Returns [`None`] if beam goes out of bounds (below 0).
    /// 
    /// **Note**: Doesn't check some grid's bounds.
    fn next_position(&self) -> Option<(usize, usize)> {
        Some(match self.direction {
            Dir::Up    => (self.col, self.row.checked_sub(1)?),
            Dir::Left  => (self.col.checked_sub(1)?, self.row),
            Dir::Down  => (self.col, self.row + 1),
            Dir::Right => (self.col + 1, self.row),
        })
    }

    /// Moves beam one tick forward and handles necessary collisions
    fn collide_with(self, map: &Grid<Tile>) -> Collision {
        let Some(&tile) = map.get(self.row, self.col) else {
            // Beam has gone out of upper bounds and dies
            return Collision::Death
        };

        if tile == Tile::Empty {
            // We are currently on empty tile. Simply move along the current direction
            return match self.next_position() {
                Some((next_col, next_row)) =>
                    Collision::Continue(
                        Beam::new(next_col, next_row, self.direction)
                    ),
                None => Collision::Death,
            }
        }

        // We're "inside" a mirror
        match tile {
            Tile::ForwardMirror => { // '/'
                let next_pos = match self.direction {
                    Dir::Up    => Some((self.col + 1, self.row, Dir::Right)),
                    Dir::Down  => self.col.checked_sub(1).map(|col| (col, self.row, Dir::Left)),
                    Dir::Left  => Some((self.col, self.row + 1, Dir::Down)),
                    Dir::Right => self.row.checked_sub(1).map(|row| (self.col, row, Dir::Up)),
                };

                if let Some((next_col, next_row, next_direction)) = next_pos {
                    Collision::Reflection(
                        Beam::new(next_col, next_row, next_direction)
                    )
                } else {
                    Collision::Death
                }
            },
            Tile::BackwardMirror => { // '\'
                let next_pos = match self.direction {
                    Dir::Up    => self.col.checked_sub(1).map(|col| (col, self.row, Dir::Left)),
                    Dir::Down  => Some((self.col + 1, self.row, Dir::Right)),
                    Dir::Left  => self.row.checked_sub(1).map(|row| (self.col, row, Dir::Up)),
                    Dir::Right => Some((self.col, self.row + 1, Dir::Down)),
                };

                if let Some((next_col, next_row, next_direction)) = next_pos {
                    Collision::Reflection(
                        Beam::new(next_col, next_row, next_direction)
                    )
                } else {
                    Collision::Death
                }
            },
            Tile::VertSplit => { // '|'
                match self.direction {
                    Dir::Up | Dir::Down => {
                        let Some((next_col, next_row)) = self.next_position() else {
                            return Collision::Death
                        };

                        Collision::Continue(Beam::new(next_col, next_row, self.direction))
                    },
                    Dir::Left | Dir::Right => {
                        // Splits into up and down beams
                        // order of splits doesn't matter as beam don't interact with each other
                        // If either beam goes out of lower bounds, the reflection
                        // degenerates into single mirror reflection.
                        if let Some(up_row) = self.row.checked_sub(1) {
                            Collision::Split(
                                Beam::new(self.col, up_row, Dir::Up),
                                Beam::new(self.col, self.row + 1, Dir::Down),
                            )
                        } else {
                            Collision::Reflection( 
                                Beam::new(self.col, self.row + 1, Dir::Down),
                            )
                        }
                    }
                }
            },
            Tile::HorSplit => { // '-'
                match self.direction {
                    Dir::Left | Dir::Right => {
                        let Some((next_col, next_row)) = self.next_position() else {
                            return Collision::Death
                        };

                        Collision::Continue(Beam::new(next_col, next_row, self.direction))
                    },
                    Dir::Up | Dir::Down => {
                        if let Some(left_col) = self.col.checked_sub(1) {
                            Collision::Split(
                                Beam::new(left_col, self.row, Dir::Left),
                                Beam::new(self.col + 1, self.row, Dir::Right),
                            )
                        } else {
                            Collision::Reflection( 
                                Beam::new(self.col + 1, self.row, Dir::Right),
                            )
                        }
                    }
                }
            },
            Tile::Empty => unreachable!(),
        }
    }
}

fn solve(mirrors: &Grid<Tile>, start: Beam) -> usize {
    // List of beams, a new beam gets added each time a splitter is encountered.
    // Beams may also "die out" if they hit grid edges
    let mut beams: VecDeque<Beam> = vec![start.clone()].into();
    // Set of seen (energized) tiles. As each tile can be energized only once,
    // but beams can collide with some tiles multiple times
    // we can't keep a simple running number but need to record each unique tiles.
    let mut seen: HashSet<(usize, usize, Dir)> = HashSet::new();
    seen.insert(start.position());

    while let Some(beam) = beams.pop_front() {
        match beam.collide_with(mirrors) {
            Collision::Continue(beam) => {
                // println!("continue: {:?}", &beam);
                if mirrors.get(beam.row, beam.col).is_some() {
                    seen.insert(beam.position());
                    beams.push_back(beam)
                }
            },
            Collision::Reflection(beam) => {
                // seen.insert(mirror_pos);
                // println!(" reflect: {:?}", &beam);
                if mirrors.get(beam.row, beam.col).is_some() {
                    seen.insert(beam.position());
                    beams.push_back(beam);
                }
            },
            Collision::Split(first, second) => {
                // seen.insert(mirror_pos);
                // println!("   split: {:?}", &first);
                // println!("          {:?}", &second);
                if mirrors.get(first.row, first.col).is_some() && mirrors.get(second.row, second.col).is_some() {
                    if !seen.contains(&first.position()) {
                        seen.insert(first.position());
                        beams.push_back(first);
                    }

                    if !seen.contains(&second.position()) {
                        seen.insert(second.position());
                        beams.push_back(second);
                    }
                }

            },
            Collision::Death => {
                // println!("beam died!");
            },
        }
    }

    print_energized(&seen, (10, 10)).unwrap();
    seen.into_iter().map(|(col, row, _)| (col, row)).collect::<HashSet<(usize, usize)>>().len()
}


fn main() -> anyhow::Result<()> {
    let input = read_input()?;
    let puzzle = parse(&input);

    // print_grid(&puzzle);
    let energized_tiles = solve(&puzzle, Beam::new(0, 0, Dir::Right));
    println!("Silver: {}", energized_tiles);

    

    Ok(())
}

fn parse(s: &str) -> Grid<Tile> {
    let mut tiles = Vec::new();
    let cols = s.lines().next().unwrap().len();
    
    for tile in s.trim().chars().filter(|ch| !ch.is_ascii_whitespace()) {
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

fn print_energized(map: &HashSet<(usize, usize, Dir)>, bounds: (usize, usize)) -> std::io::Result<()> {
    let unrolled: HashSet<(usize, usize)> = map.into_iter().map(|&(col, row, _)| (col, row)).collect();

    use std::io::Write;
    let mut lock = std::io::stdout().lock();
    for row in 0..bounds.0 {
        for col in 0..bounds.1 {
            if unrolled.contains(&(col, row)) {
                write!(lock, "#")?;
            } else {
                write!(lock, ".")?;
            }
        }
        write!(lock, "\n")?;
    }

    Ok(())
}
