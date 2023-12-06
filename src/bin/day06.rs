use advent::read_input;

#[derive(Debug)]
struct Race {
    time: usize,
    record: usize,
}

impl Race {
    /// Calculate how many possible ways there are to win this race
    fn number_of_wins(&self) -> usize {
        let mut result = 0;

        for holding in 0..self.time {
            let speed = holding;
            let remaining = self.time - holding;

            let distance_covered = speed * remaining;

            if distance_covered > self.record {
                result += 1;
            }
        }

        result
    }
}

enum ParseMode {
    Multiple, // Silver part
    Single,   // Gold part
}

fn parse(input: &str, mode: ParseMode) -> Vec<Race> {
    let mut lines = input.lines();
    let times = lines.next().expect("missing 'times' line");
    let records = lines.next().expect("missing 'record distance' line");

    let (_, times) = times.split_once(':').unwrap();
    let (_, records) = records.split_once(':').unwrap();


    let (times, records) = match mode {
        ParseMode::Multiple => (times.to_string(), records.to_string()),
        ParseMode::Single => (
            times.chars().filter(|c| c.is_ascii_digit()).collect(),
            records.chars().filter(|c| c.is_ascii_digit()).collect()
        ),
    };

    let mut out: Vec<Race> = Vec::new();
    for (time, record) in times.split_ascii_whitespace().zip(records.split_ascii_whitespace()) {
        out.push(Race {
            time: time.parse().unwrap(),
            record: record.parse().unwrap(),
        });
    }

    out
}

fn main() -> anyhow::Result<()> {
    let input = read_input()?;
    let silver_races = parse(&input, ParseMode::Multiple);
    let gold_race = parse(&input, ParseMode::Single);

    let silver_sum: usize = silver_races.iter().map(|race| race.number_of_wins()).product();

    println!("Silver: {}", silver_sum);
    println!("  Gold: {}", gold_race.first().unwrap().number_of_wins());

    Ok(())
}
