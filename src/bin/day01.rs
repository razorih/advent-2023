use advent::read_input;
use aho_corasick::AhoCorasick;

/// Get first and last element of an iterator.
/// If iterator only has one item, returns first item twice.
/// 
/// Returns [`None`] if iterator is empty.
fn iter_first_last<I: Clone>(mut iter: impl Iterator<Item=I>) -> Option<(I, I)> {
    let Some(first) = iter.next() else {
        return None
    };

    let Some(last) = iter.last() else {
        return Some((first.clone(), first));
    };

    Some((first, last))
}

fn solve_line(line: &str, ac: &AhoCorasick) -> usize {
    let res = iter_first_last(ac.find_overlapping_iter(line));
    res.map(|pair| {
        // Convert pattern ID into a numeric value
        let numeric = (
            pair.0.pattern().as_usize() % 9 + 1,
            pair.1.pattern().as_usize() % 9 + 1,
        );
        numeric.0*10 + numeric.1
    }).unwrap()
}

#[cfg(feature = "parallel")]
fn solve(input: &str, ac: &AhoCorasick) -> usize {
    use rayon::prelude::*;
    input.trim().par_lines().map(|line| solve_line(line, ac)).sum()
}

#[cfg(not(feature = "parallel"))]
fn solve(input: &str, ac: &AhoCorasick) -> usize {
    input.trim().lines().map(|line| solve_line(line, ac)).sum()
}

fn silver(input: &str) -> usize {
    const DIGITS: [&str; 9] = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9",
    ];
    let ac = AhoCorasick::new(DIGITS).unwrap();

    solve(input, &ac)
}

fn gold(input: &str) -> usize {
    const DIGITS: [&str; 18] = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", 
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let ac = AhoCorasick::new(DIGITS).unwrap();

    solve(input, &ac)
}

fn main() -> anyhow::Result<()> {
    let input = read_input()?;

    println!("Silver: {}", silver(input.as_str()));
    println!("  Gold: {}", gold(input.as_str()));

    Ok(())
}
