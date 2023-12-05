use std::{ops::Range, str::FromStr};

use advent::read_input;
use anyhow::anyhow;

// 475615938 - too high
// 115354858 - wrong

#[derive(Debug)]
struct Map(Vec<MapLine>);
impl Map {
    /// Translate given ranges.
    fn translate(&self, seed_ranges: Vec<Range<usize>>) -> Vec<Range<usize>> {
        let mut out: Vec<Range<usize>> = Vec::new();
        
        for seeds in seed_ranges {
            let mut was_translated = false;
            // ASSUMPTION: Map lines never overlap
            println!("--- SEED START ---");
            for translator in &self.0 {
                if let Some(changed) = translator.try_translate(&seeds) {
                    // changed contains newly mapped range
                    // changed can contain multiple ranges, some might be 1-to-1 mapped, others mapped to dst
                    out.extend(changed);
                    was_translated = true;
                    break;
                }
            }
            // No translator could translate this range, it is passed 1-to-1
            if !was_translated {
                out.push(seeds.clone());
            }
            println!("--- SEED END   ---\n");
        }

        out
    }

    fn from_lines<'a, T>(source: &mut T) -> Option<Self>
    where
        T: Iterator<Item = &'a str>
    {
        let mut lines: Vec<MapLine> = Vec::new();
        while let Some(line) = source.next() {
            // Encountered possible newline
            if line.is_empty() {
                return Some(Self(lines));
            }

            if line.starts_with(char::is_alphabetic) {
                continue;
            }

            lines.push(line.parse::<MapLine>().unwrap());
        }

        if lines.is_empty() {
            None
        } else {
            Some(Self(lines))
        }
    }

}

#[derive(Debug)]
struct MapLine {
    dst: Range<usize>,
    src: Range<usize>,
}

fn adjust_range(range: Range<usize>, offset: isize) -> Range<usize> {
    if offset >= 0 {
        let offset = offset as usize;
        range.start+offset..range.end+offset
    } else {
        range.start.checked_add_signed(offset).expect("seed.start overflowed")
        ..
        range.end.checked_add_signed(offset).expect("seed.end overflowed")
    }
}

impl MapLine {
    /// Try to translate given range.
    fn try_translate(&self, seeds: &Range<usize>) -> Option<Vec<Range<usize>>> {
        if seeds.end <= self.src.start || seeds.start >= self.src.end {
            println!("  1-to-1 seeds: {:?} - range: {:?}", seeds, self.src);
            return None
        }

        // Seeds overlap in some way
        // Calculate offset for adjusting
        let offset = self.dst.start as isize - self.src.start as isize;

        // If seeds are fully contained within the area, we can just map them
        // without splitting the range
        if self.src.start <= seeds.start && self.src.end >= seeds.end {
            println!("  contained overlap seeds: {:?} - range: {:?}", seeds, self.src);
            return Some(vec![adjust_range(seeds.clone(), offset)])
        }

        // Seeds are larger than the area, needs to be split 3-way
        if seeds.start < self.src.start && seeds.end > self.src.end {
            println!("  FULLY OUTSIDE seeds: {:?} - range: {:?}:", seeds, self.src);
            let left_outside = seeds.start..self.src.start;
            let inside = self.src.start..self.src.end;
            let right_outside = self.src.end..seeds.end;
            println!("    inside (map): {:?}, outside (left, pass): {:?}, outside (right, pass): {:?}", inside, left_outside, right_outside);
            // panic!("impossible!");
            return Some(vec![left_outside, adjust_range(inside, offset), right_outside]);
        }

        // Partial overlaps, 2-way splits
        // Left side out of the range
        if seeds.end > self.src.start && seeds.start < self.src.start {
            println!("  LEFT  OUTSIDE seeds: {:?} - range: {:?}:", seeds, self.src);
            let inside = self.src.start..seeds.end;
            let outside = seeds.start..self.src.start;
            println!("    inside (map): {:?}, outside (pass): {:?}", inside, outside);

            return Some(vec![adjust_range(inside, offset), outside]);
        }

        // Right side out of the range
        if seeds.start < self.src.end && seeds.end > self.src.end {
            println!("  RIGHT OUTSIDE seeds: {:?} - range: {:?}:", seeds, self.src);
            let outside = self.src.end..seeds.end;
            let inside = seeds.start..self.src.end;
            println!("    inside (map): {:?}, outside (pass): {:?}", inside, outside);

            return Some(vec![adjust_range(inside, offset), outside]);
        }

        println!("!!!! unhandled overlap seeds: {:?} - range: {:?} !!!!", seeds, self.src);
        unreachable!();
    }
}


fn solve(seeds: Seeds, maps: &[Map]) -> usize {
    println!("starting seeds: {seeds:?}");
    let pre_total = seeds.0.iter().fold(0, |acc, range| acc + range.len()) as isize;
    // println!("pre total seeds:    {}", pre_total);

    // let mut seeds = HashSet::from_iter(seeds.0);
    let mut seeds = seeds.0;
    for map in maps {
        seeds = map.translate(seeds);
        let count = seeds.iter().fold(0, |acc, range| acc + range.len()) as isize;
        println!("seeds after map: {:?} (count: {count}\n\n\n", &seeds);
    }

    let post_total = seeds.iter().fold(0, |acc, range| acc + range.len()) as isize;

    if pre_total != post_total {
        panic!("unexpected amount of seeds!, expected {pre_total} got {post_total} (diff: {})", post_total - pre_total);
    }
    seeds.iter().min_by_key(|&range| range.start.min(range.end)).unwrap().start
}

fn main() -> anyhow::Result<()> {
    let input = read_input()?;

    let mut lines = input.lines();
    let seedline = lines.next().unwrap();
    let _ = lines.next(); // Eat newline after seeds

    let silver_seeds = Seeds::from_singles_str(seedline);
    let gold_seeds = Seeds::from_ranges_str(seedline);

    let mut maps: Vec<Map> = Vec::new();
    while let Some(map) = Map::from_lines(&mut lines) {
        maps.push(map)
    }

    // println!("Silver: {}", solve(silver_seeds, &maps));
    println!("  Gold: {}", solve(gold_seeds, &maps));

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ranges() {
        let line: MapLine = "50 98 2".parse().unwrap();
        assert_eq!(line.dst, 50..52);
        assert_eq!(line.src, 98..100);
    }

    #[test]
    fn parse_single_seeds() {
        let seeds: Seeds = Seeds::from_singles_str("seeds: 79 14 55 13");
        assert_eq!(seeds.0, &[79..80, 14..15, 55..56, 13..14]);
    }

    #[test]
    fn parse_seed_range() {
        let seeds: Seeds = Seeds::from_ranges_str("seeds: 79 14 55 13");
        assert_eq!(seeds.0, &[79..79+14, 55..55+13]);
    }
}



impl FromStr for MapLine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_ascii_whitespace();
        let dst_start: usize = split.next()
            .ok_or_else(|| anyhow!("missing dst start"))?
            .parse()?;
        let src_start: usize = split.next()
            .ok_or_else(|| anyhow!("missing src start"))?
            .parse()?;
        let range_len: usize = split.next()
            .ok_or_else(|| anyhow!("missing range length"))?
            .parse()?;

        Ok(Self {
            dst: dst_start..(dst_start + range_len),
            src: src_start..(src_start + range_len),
        })
    }
}


#[derive(Debug)]
struct Seeds(Vec<Range<usize>>);
impl Seeds {
    fn from_ranges_str(s: &str) -> Self {
        let s = s.strip_prefix("seeds: ").unwrap();
        let mut parts = s.split_ascii_whitespace();

        let mut out = Vec::new();
        while let (Some(Ok(start)), Some(Ok(length))) = (
            parts.next().map(|p| p.parse::<usize>()),
            parts.next().map(|p| p.parse::<usize>()),
        )
        {
            out.push(start..start+length)
        }

        Self(out)
    }

    fn from_singles_str(s: &str) -> Self {
        let s = s.strip_prefix("seeds: ").unwrap();
        let inner = s.split_ascii_whitespace().map(|num| {
            let start = num.parse::<usize>().unwrap();
            start..start+1
        });

        Self(inner.collect())
    }
}
