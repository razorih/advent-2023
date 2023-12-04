use advent::read_input;

/// Bitset for storing integers between 0-127
#[derive(Debug)]
struct NumberBitSet(u64, u64);

impl NumberBitSet {
    fn from_iter(iter: impl Iterator<Item = u8>) -> Self {
        let mut set = Self(0, 0);
        for num in iter {
            set.insert(num);
        }
        set
    }

    fn count_matching_numbers(&self, other: &Self) -> u32 {
        (self.0 & other.0).count_ones() + (self.1 & other.1).count_ones()
    }

    fn insert(&mut self, item: u8) {
        debug_assert!(item < 100);

        // Check which integer stores this item
        if item < 64 { // 0-63
            self.0 |= 1 << item
        } else { // 64-127
            self.1 |= 1 << (item - 64)
        }
    }
}

#[derive(Debug)]
struct MatchCountIter<T>(T);
impl<T> MatchCountIter<T> {
    fn from_linesource(linesource: T) -> Self { Self(linesource) }
}

impl<'a, T> Iterator for MatchCountIter<T>
where
    T: Iterator<Item = &'a str>    
{
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(line) = self.0.next() {
            let (_, rest) = line.split_once(':')?;
            let (winning, have) = rest.split_once('|')?;

            // Only extend winning set, as we can check for membership separately.
            // Note: Set is empty, but the same allocation is reused in order
            //       to avoid reallocating on each time next() is called.
            let winning = NumberBitSet::from_iter(
                winning.split_ascii_whitespace()
                    .map(|num| num.parse::<u8>().unwrap())
            );

            let have = NumberBitSet::from_iter(
                have.split_ascii_whitespace()
                    .map(|num| num.parse::<u8>().unwrap())
            );

            Some(winning.count_matching_numbers(&have) as usize)
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
