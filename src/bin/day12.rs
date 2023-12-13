use std::{str::FromStr, fmt::Write, collections::HashMap};

use advent::read_input;
use anyhow::anyhow;

#[derive(Clone, Copy, PartialEq)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl Spring {
    fn from_char(ch: char) -> Result<Self, anyhow::Error> {
        match ch {
            '#' => Ok(Self::Damaged),
            '.' => Ok(Self::Operational),
            '?' => Ok(Self::Unknown),
            _   => Err(anyhow!("invalid char {ch}"))
        }
    }
}

#[derive(Debug)]
struct Puzzle {
    springs: Vec<Spring>,
    groups: Vec<usize>,
}


impl Puzzle {
    /// Count valid combinations
    /// 
    /// Python implementation by **rrutkows**:
    /// https://github.com/rrutkows/aoc_py/blob/1efbd74961499edcf68b0749f39ec8b13853db8a/2023/d12.py
    /// 
    /// Idea by **KayZGames**:
    /// https://www.reddit.com/r/adventofcode/comments/18hbjdi/2023_day_12_part_2_this_image_helped_a_few_people/
    fn combinations(&self) -> usize {
        // Map to keep track of all permutation counts there is.
        // Key here is (group_idx, group_amount) and value is the number of permutations.
        //
        // group_idx is an index to the array which contains sizes of each contiguous group of damage springs (part of the puzzle input).
        // e.g. here is points to the last element:
        // [1, 1, 3]
        //        ^ group_idx == 2
        // group_idx may also point out of bounds.
        // 
        // group_amount on other hand indicates how many damaged springs we have seen
        // in this group.
        let mut permutations: HashMap<(usize, usize), usize> = HashMap::new();
        permutations.insert((0, 0), 1);

        for &spring in &self.springs {
            let mut next: Vec<(usize, usize, usize)> = Vec::new();

            for (&(group_idx, group_amount), &perm_count) in permutations.iter() {
                // Unknown springs are handled as them being both damaged and operational
                // Both paths here push a new (group_idx, group_amount, permutations) triple.

                if spring != Spring::Damaged {
                    if group_amount == 0 {
                        // No damaged springs have been seen yet, nothing happens
                        next.push((group_idx, group_amount, perm_count))
                    } else if group_amount == self.groups[group_idx] {
                        // A group just ended and group is filled completely, i.e.
                        // ##.. [2,]
                        //   ^ we're here  
                        next.push((group_idx + 1, 0, perm_count))
                    }
                    // Impossible cases dropped.
                    // 1. ending a group that is not yet full
                    // 2. 
                }

                if spring != Spring::Operational {
                    if group_idx < self.groups.len() && group_amount < self.groups[group_idx] {
                        // There is still space in this group, increase amount
                        next.push((group_idx, group_amount + 1, perm_count))
                    }
                    // Impossible cases dropped:
                    // 1. group would overfill
                    // 2. trying to fill nonexisting group
                }
            }

            // Reset permutations from last iteration and sum all
            // (group, amount) permutations we currently have.
            // 
            // For example, `next` array: [(0, 1, 3), (0, 1, 4), (1, 0, 1)]
            // gets turned into map {(0, 1): 7, (1, 0): 1}
            permutations.clear();
            for (group_id, group_amount, perm_count) in next {
                permutations.entry((group_id, group_amount))
                    .and_modify(|amount| *amount += perm_count)
                    .or_insert(perm_count);
            }
        }


        // Function to check if given permutation pair is actually valid
        // Two possibilities:
        // 1. group index has gone out-of-bounds,
        //    meaning all groups have been filled.
        // 2. Index points to last group and amount is exactly as required.
        let is_valid = |idx: usize, amount: usize| {
            (idx == self.groups.len() && amount == 0)
            || (idx == self.groups.len() - 1 && amount == self.groups[idx])
        };

        // Finally, return sum of all valid permutations
        permutations.iter().filter_map(|(&(id, amount), &v)| {
            if is_valid(id, amount) {
                Some(v)
            } else {
                None
            }
        }).sum()
    }
}

fn main() -> anyhow::Result<()> {
    let input = read_input()?;
    let puzzles = parse(input.lines())?;

    let mut sum = 0;
    for puzzle in puzzles {
        println!("{}", puzzle);

        sum += dbg!(puzzle.combinations());
    }

    println!("Gold: {sum}");

    Ok(())
}

fn parse<'a>(lines: impl Iterator<Item = &'a str>) -> Result<Vec<Puzzle>, anyhow::Error> {
    lines.map(|line| line.parse()).collect()
}

impl FromStr for Puzzle {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (springs, damages) = s.split_once(' ').ok_or_else(|| anyhow!("invalid puzzle line"))?;
        let springs = springs.chars()
            .map(|ch| Spring::from_char(ch))
            .collect::<Result<Vec<_>, _>>()?;

        // oof
        let springs = [
            &springs[..], &[Spring::Unknown],
            &springs[..], &[Spring::Unknown],
            &springs[..], &[Spring::Unknown],
            &springs[..], &[Spring::Unknown],
            &springs[..],
        ].concat();

        let damage_groups = damages.split(',')
            .map(|num| num.parse())
            .collect::<Result<Vec<_>, _>>()?;
        let damage_groups = [&damage_groups[..]; 5].concat();

        Ok(Self { springs, groups: damage_groups })
    }
}

impl std::fmt::Debug for Spring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Spring::Operational => f.write_char('.'),
            Spring::Damaged => f.write_char('#'),
            Spring::Unknown => f.write_char('?'),
        }
    }
}

impl std::fmt::Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?}", self.springs, self.groups)
    }
}
