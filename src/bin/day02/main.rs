use advent::read_input;

mod error;
mod game;

use game::Game;

fn silver(input: &str) -> anyhow::Result<usize> {
    let mut possible_sum = 0;
    for line in input.trim().lines() {
        let game: Game = line.parse()?;

        if game.sets.iter().any(|&set| set.0 > 12 || set.1 > 13 || set.2 > 14) {
            //println!("Game {:3} is impossible", game.id);
            continue;
        }

        possible_sum += game.id;
    }

    Ok(possible_sum)
}

fn gold(input: &str) -> anyhow::Result<usize> {
    let mut power_sum: usize = 0;
    for line in input.trim().lines() {
        let game: Game = line.parse()?;

        let mut max = (0, 0, 0);
        for set in game.sets {
            if set.0 > max.0 {
                max.0 = set.0;
            }

            if set.1 > max.1 {
                max.1 = set.1;
            }

            if set.2 > max.2 {
                max.2 = set.2;
            }
        }

        power_sum += max.0 as usize * max.1 as usize * max.2 as usize;
    }

    Ok(power_sum)
}

fn main() -> anyhow::Result<()> {
    let input = read_input()?;

    println!("Silver: {}", silver(input.as_str())?);
    println!("  Gold: {}", gold(input.as_str())?);

    Ok(())
}
