use std::{ops::Range, str::FromStr};

use advent::read_input;
use anyhow::anyhow;

#[derive(Debug)]
struct Seeds(Vec<usize>);
impl FromStr for Seeds {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("seeds: ").ok_or_else(|| anyhow!("missing 'seeds' prefix"))?;
        let inner = s.split_ascii_whitespace().map(|num| num.parse::<usize>().unwrap());

        Ok(Self(inner.collect()))
    }
}

#[derive(Debug)]
struct Map(Vec<MapLine>);

impl Map {
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

            println!("parsed: {}", line);
            lines.push(line.parse::<MapLine>().unwrap());
        }

        if lines.is_empty() {
            None
        } else {
            Some(Self(lines))
        }
    }

    fn translate(&self, src: usize) -> usize {
        self.0.iter()
            // Check if any range can translate this point
            .find_map(|mapline| mapline.try_translate(src))
            // Otherwise, return the original point
            .unwrap_or(src)
    }
}

#[derive(Debug)]
struct MapLine {
    dst: Range<usize>,
    src: Range<usize>,
}

impl MapLine {
    fn try_translate(&self, src: usize) -> Option<usize> {
        if self.src.contains(&src) {
            // map using dst range
            let offset = src - self.src.start;
            Some(self.dst.start + offset)
        } else {
            // 1-to-1 mapping
            None
        }
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

fn main() -> anyhow::Result<()> {
    let input = read_input()?;

    let mut lines = input.lines();
    let mut seeds: Seeds = lines.next().map(|line| line.parse().unwrap()).unwrap();
    let _ = lines.next(); // Eat newline after seeds

    let mut chain: Vec<Map> = Vec::new();
    while let Some(map) = Map::from_lines(&mut lines) {
        chain.push(map)
    }

    println!("seeds: {:?}", seeds);
    println!("maps:  {:#?}", chain);

    for map in chain {
        for seed in &mut seeds.0 {
            *seed = map.translate(*seed);
        }
        println!("{:?}", &seeds);
    }

    println!("Silver: {}", seeds.0.iter().min().unwrap());

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
    fn map_line_translate() {
        let line: MapLine = "50 98 2".parse().unwrap();
        assert_eq!(line.try_translate(99), Some(51));
        assert_eq!(line.try_translate(0), None);
    }

    #[test]
    fn parse_seeds() {
        let seeds: Seeds = "seeds: 79 14 55 13".parse().unwrap();
        assert_eq!(seeds.0, &[79, 14, 55, 13]);
    }
}
