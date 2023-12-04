use std::collections::HashSet;

use advent::read_input;

#[derive(Debug)]
struct MatchCountIter<T> {
    linesource: T,
    winning: HashSet<u16>,
}

impl<T> MatchCountIter<T> {
    fn from_linesource(linesource: T) -> Self {
        Self {
            linesource,
            winning: HashSet::new(),
        }
    }
}

impl<'a, T> Iterator for MatchCountIter<T>
where
    T: Iterator<Item = &'a str>    
{
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        self.winning.clear();

        if let Some(line) = self.linesource.next() {
            let s = line.strip_prefix("Card ")?;
            let (_, rest) = s.split_once(':')?;
            let (winning, have) = rest.split_once('|')?;

            // Only extend winning set, as we can check for membership separately.
            // Note: Set is empty, but the same allocation is reused in order
            //       to avoid reallocating on each time next() is called.
            self.winning.extend(
                winning.split_ascii_whitespace()
                    .map(|num| num.parse::<u16>().unwrap())
            );

            // Check for membership in a loop instead of building a separate set.
            let mut match_count: usize = 0;
            for num in have
                .split_ascii_whitespace()
                .map(|num| num.parse::<u16>().unwrap())
            {
                if self.winning.contains(&num) {
                    match_count += 1;
                }
            }

            Some(match_count)
        } else {
            None
        }
    }
}

fn score(match_count: usize) -> usize {
    if match_count > 0 {
        2_usize.pow(match_count as u32 - 1)
    } else {
        0
    }
}

fn silver(input: &str) -> usize {
    MatchCountIter::from_linesource(input.trim().lines())
        .map(|match_count| score(match_count))
        .sum()
}

fn gold(input: &str) -> usize {
    let matching = MatchCountIter::from_linesource(input.trim().lines())
        .collect::<Vec<_>>();
    
    let mut card_counts = vec![1_usize; matching.len()];
    for i in 0..card_counts.len() {
        let count = card_counts[i];

        // Accumulate extra cards if card has matching numbers
        for extra_i in 0..matching[i] {
            card_counts[i+extra_i+1] += count;
        }
    }

    card_counts.iter().sum()
}

fn main() -> anyhow::Result<()> {
    let input = read_input()?;

    println!("Silver: {}", silver(&input));
    println!("  Gold: {}", gold(&input));

    Ok(())
}
