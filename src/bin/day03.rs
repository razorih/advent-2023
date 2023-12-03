use grid::Grid;

use advent::read_input;

#[derive(Debug)]
struct Schematic {
    grid: Grid<u8>,
}

impl Schematic {
    fn from_string(mut s: String) -> Self {
        // First, calculate number of columns (line length)
        let cols = s.lines().nth(0).map(|line| line.len()).unwrap();

        // Remove all newlines from the original string,
        // this ensures that we can convert the string into 1D array of bytes.
        s.retain(|c| !c.is_ascii_whitespace());

        Self {
            grid: Grid::from_vec(s.into_bytes(), cols),
        }
    }
}

#[derive(Debug)]
struct Number {
    digits: String,
}

impl Number {
    fn new() -> Self {
        Self { digits: String::with_capacity(16) }
    }

    fn push(&mut self, digit: char) {
        self.digits.push(digit);
    }

    fn clear(&mut self) {
        self.digits.clear()
    }

    /// Try to build a number.
    fn take(&mut self) -> Option<usize> {
        if self.digits.is_empty() {
            return None;
        }

        if let Ok(out) = self.digits.parse::<usize>() {
            self.clear();
            Some(out)
        } else {
            None
        }
    }
}

fn main() -> anyhow::Result<()> {
    let input = read_input()?;
    let schematic = Schematic::from_string(input);
    let grid = schematic.grid;

    let (rows, cols) = grid.size();

    const DIRS: [(isize, isize); 8] = [
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


    let mut sum: usize = 0;
    let mut number = Number::new();
    let mut was_connected = false;

    for x in 0..cols {
        for y in 0..rows {
            // print!("{}", grid[(x, y)] as char);
            let c = grid[(x, y)];
            if !c.is_ascii_digit() {
                // Check if connected number ended here
                if was_connected {
                    let num = number.take().unwrap();
                    println!("constructed: {}", num);
                    sum += num;
                    was_connected = false;
                } else {
                    number.clear()
                }

                continue;
            }

            // ASCII digit
            number.push(c as char);

            // Look to all directions, trying to see a symbol
            for (dx, dy) in DIRS {
                // Bounds checking, underflows
                let Some(nx) = x.checked_add_signed(dx) else {
                    continue;
                };

                let Some(ny) = y.checked_add_signed(dy) else {
                    continue;
                };

                // Overflows
                let Some(&s) = grid.get(nx, ny) else {
                    continue;
                };

                if !s.is_ascii_digit() && s as char != '.' {
                    println!("connected symbol: {} -> {}", c as char, s as char);
                    was_connected = true;
                }
            }
        }
        print!("\n")
    }

    println!("Silver: {}", sum);

    Ok(())
}
