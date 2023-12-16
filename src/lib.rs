use std::path::{Path, PathBuf};
use std::io::{self, Read, Write};
use std::fmt::{Display, Debug};

use grid::Grid;

/// Helper utility for reading advent of code input files.
pub fn read_input() -> Result<String, io::Error> {
    let filename = get_filename_from_args()?;

    if filename == "-" {
        read_from_stdin()
    } else {
        let resolved = resolve_path(&filename)?;
        std::fs::read_to_string(resolved)
    }
}

/// Similar to [`read_input`], but opens the file via memory mapping.
#[cfg(feature = "mmap")]
pub fn map_input() -> Result<memmap2::Mmap, io::Error> {
    use std::fs::File;

    let filename = get_filename_from_args()?;
    let resolved = resolve_path(&filename)?;
    let file = File::open(resolved)?;

    let map = unsafe { memmap2::Mmap::map(&file) }?;
    Ok(map)
}

/// Read **unbounded** [`String`] from standard input.
fn read_from_stdin() -> Result<String, io::Error> {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn get_filename_from_args() -> Result<String, io::Error> {
    std::env::args().nth(1)
        .ok_or_else(|| io::Error::new(
            io::ErrorKind::Other,
            r#"expected input file path or "-" as first argument"#
        ))
}

fn resolve_path(path: impl AsRef<Path>) -> Result<PathBuf, io::Error> {
    let path = path.as_ref();

    if path.is_absolute() {
        path.to_path_buf()
    } else {
        let mut base = std::env::current_dir()?;
        base.push("inputs/");
        base.push(path);
        base
    }.canonicalize()
}

pub fn print_grid<T: Display>(grid: &Grid<T>) {
    let mut lock = std::io::stdout().lock();

    for row in grid.iter_rows() {
        for tile in row {
            write!(lock, "{tile}").unwrap();
        }
        writeln!(lock).unwrap();
    }
    writeln!(lock).unwrap();
}

pub fn print_grid_debug<T: Debug>(grid: &Grid<T>) {
    let mut lock = std::io::stdout().lock();

    for row in grid.iter_rows() {
        for tile in row {
            write!(lock, "{tile:?}").unwrap();
        }
        writeln!(lock).unwrap();
    }
    writeln!(lock).unwrap();
}
