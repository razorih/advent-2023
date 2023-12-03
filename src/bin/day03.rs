use std::collections::{HashMap, HashSet};

use grid::Grid;

use advent::read_input;

const DIRS: [(i8, i8); 8] = [
    (-1,  0),
    ( 1,  0),
    ( 0, -1),
    ( 0,  1),
    // Diagonals
    (-1,  1),
    (-1, -1),
    ( 1,  1),
    ( 1, -1),
];

fn grid_from_string(mut s: String) -> Grid<u8> {
    // First, calculate number of columns (line length)
    let cols = s.lines().nth(0).map(|line| line.len()).unwrap();

    // Remove all newlines from the original string,
    // this ensures that we can convert the string into 1D array of bytes.
    s.retain(|c| !c.is_ascii_whitespace());

    Grid::from_vec(s.into_bytes(), cols)
}

#[derive(Debug)]
struct Number {
    /// Number's digits so far
    digits: String,
    /// Set of symbols this number is connected to.
    /// Tuple has format `(x, y, is_gear)`
    connected_symbols: HashSet<(usize, usize, bool)>
}

impl Number {
    fn new() -> Self {
        Self {
            digits: String::new(),
            connected_symbols: HashSet::new(),
        }
    }

    fn push(&mut self, digit: char) {
        self.digits.push(digit);
    }

    /// Try to build a number.
    fn take(&mut self) -> Option<usize> {
        if self.digits.is_empty() {
            return None;
        }

        if let Ok(out) = self.digits.parse::<usize>() {
            self.digits.clear();
            Some(out)
        } else {
            None
        }
    }
}

fn main() -> anyhow::Result<()> {
    let input = read_input()?;
    let grid = grid_from_string(input);
    let (rows, cols) = grid.size();

    let mut silver_sum: usize = 0;
    let mut number = Number::new();
    // Seen gear coordinates and list of numbers that are connected to them
    let mut gears: HashMap<(usize, usize), Vec<usize>> = HashMap::new();

    for x in 0..rows {
        for y in 0..cols {
            let c = grid[(x, y)];

            if !c.is_ascii_digit() {
                // Either an empty space or a symbol.

                // We may have been constructing a number,
                // but it can be discarded, because it wasn't connected to anything
                if number.connected_symbols.is_empty() {
                    number.digits.clear();
                    continue;
                }

                // Since we were connected to a symbol, there's a number ready
                let num = number.take().unwrap();
                for (gear_x, gear_y, is_gear) in number.connected_symbols.iter() {
                    if *is_gear {
                        let gear = gears.entry((*gear_x, *gear_y))
                            .or_insert_with(|| Vec::with_capacity(2));
                        gear.push(num);
                    }
                    silver_sum += num;
                }

                number.connected_symbols.clear();

                continue;
            }

            // Current character is a digit.
            // Start by pushing it to the number string.
            number.push(c as char);

            // Then, look to all directions to check if it is connected to any
            // symbols.
            for (dx, dy) in DIRS {
                // Bounds checks, usize overflows
                let Some(nx) = x.checked_add_signed(dx as isize) else {
                    continue;
                };

                let Some(ny) = y.checked_add_signed(dy as isize) else {
                    continue;
                };

                // Grid bounds
                let Some(&s) = grid.get(nx, ny) else {
                    continue;
                };

                if !s.is_ascii_digit() && s != b'.' {
                    // This digit is connected to a symbol.
                    // Set is required here as same number may be connected to the same symbol multiple times.
                    // For example, here number `123` is connected to symbol `*` 3 times;
                    // once for each digit (1, 2, and 3):
                    //   .123.
                    //   ..*..
                    
                    let is_gear = s == b'*';
                    number.connected_symbols.insert((nx, ny, is_gear));
                }
            }
        }
    }

    // Bug: Number under construction is left in the number builder
    //      if input ends with a digit.
    debug_assert!(number.digits.is_empty());

    // println!("{:?}", gears);
    let gold_sum: usize = gears.values()
        .filter_map(|numbers|
            if numbers.len() == 2 {
                Some(numbers.iter().product::<usize>()) 
            } else {
                None
            }
        ).sum();

    println!("Silver: {}", silver_sum);
    println!("  Gold: {}", gold_sum);

    Ok(())
}
