use std::{str::FromStr, fmt::Write};

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
    damage_groups: Vec<usize>,
}

impl Puzzle {
    /// Resolves all possible combinations
    fn combinations(&self) -> Vec<Vec<Spring>> {
        let mut todo: Vec<Vec<Spring>> = vec![self.springs.clone()];
        let mut out: Vec<Vec<Spring>> = Vec::with_capacity(self.springs.len());

        while let Some(mut springs) = todo.pop() {
            // Try to find unknown spring
            if let Some(unk_pos) = springs.iter().position(|&s| s == Spring::Unknown) {
                // Unknown found, queue possible versions
                // TODO: Prune impossible branches!!
                springs[unk_pos] = Spring::Operational;
                todo.push(springs.clone());
                springs[unk_pos] = Spring::Damaged;
                todo.push(springs);
            } else {
                // No unknowns remaining, ready to be checked
                out.push(springs);
            }
        }

        out
    }
}

fn is_valid(springs: &[Spring], groups: &[usize]) -> bool {
    let mut runs: Vec<usize> = Vec::with_capacity(groups.len());

    let mut current_run: usize = 0;
    for &spring in springs.iter() {
        match spring {
            Spring::Operational => {
                // Check if previous spring was damaged
                if current_run > 0 {
                    runs.push(current_run)
                }
                current_run = 0; // Reset runs
            },
            Spring::Damaged => current_run += 1,
            Spring::Unknown => panic!("tried to check unknown string"),
        }
    }

    // Check for leftover run
    if current_run > 0 {
        runs.push(current_run);
    }
    
    return &runs == groups;
}

fn main() -> anyhow::Result<()> {
    let input = read_input()?;
    let puzzles = parse(input.lines())?;

    let mut sum = 0;
    for puzzle in puzzles {
        let combinations = puzzle.combinations();
        let valid_combinations = combinations.iter()
            .filter(|comb| is_valid(&comb, &puzzle.damage_groups))
            .count();

        sum += valid_combinations;
    }

    println!("Silver: {}", sum);

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

        let damage_groups = damages.split(',')
            .map(|num| num.parse())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { springs, damage_groups })
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
