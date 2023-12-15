use advent::read_input;

fn hash(s: &[u8]) -> u64 {
    let mut hash: u64 = 0;

    for &val in s {
        hash += val as u64;
        hash *= 17;
        hash %= 256;
    }

    hash
}

fn main() -> anyhow::Result<()> {
    let input = read_input()?;

    let mut sum = 0;
    for part in input.trim().split(',') {
        sum += hash(part.as_bytes());
    }
    println!("Silver: {}", sum);

    Ok(())
}
