use std::path::Path;
use std::io::{self, Read};

/// Helper utility for reading advent of code input files.
pub fn read_input() -> Result<String, io::Error> {
    let filename = std::env::args().nth(1)
        .ok_or_else(|| io::Error::new(
            io::ErrorKind::Other,
            r#"expected input file path or "-" as first argument"#
        ))?;

    if filename == "-" {
        read_from_stdin()
    } else {
        read_file(&filename)
    }
}

/// Read **unbounded** [`String`] from standard input.
fn read_from_stdin() -> Result<String, io::Error> {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer)
}

/// Read file specified by `path`.
///
/// `path` can be either absolute path to a file
/// or a path relative to `inputs/` folder next to the current working directory.
fn read_file(path: impl AsRef<Path>) -> Result<String, io::Error> {
    let path = path.as_ref();
    let resolved = if path.is_absolute() {
        path.to_path_buf()
    } else {
        let mut base = std::env::current_dir()?;
        base.push("inputs/");
        base.push(path);
        base
    }.canonicalize()?;

    std::fs::read_to_string(resolved)
}
