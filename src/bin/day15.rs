use advent::read_input;

#[derive(Debug)]
enum Op {
    Set {
        id: String,
        focal_length: u8,
    },
    Remove {
        id: String,
    }
}

impl Op {
    fn from_str(s: &str) -> Self {
        // Look for '=' or '-'
        match s.find(['=', '-']).map(|idx| (idx, s.chars().nth(idx).unwrap())) {
            Some((i, '=')) => {
                Self::Set { id: s[..i].to_string(), focal_length: s[i+1..].parse().unwrap() }
            },
            Some((i, '-')) => {
                Self::Remove { id: s[..i].to_string() }
            },
            _ => panic!("invalid lens instruction"),
        }
    }
}

#[derive(Debug, Clone)]
struct Lens {
    id: String,
    focal_length: u8,
}

impl Lens {
    fn new(id: &str, focal_length: u8) -> Self {
        Self { id: id.to_string(), focal_length }
    }
}

#[derive(Debug, Clone, Default)]
struct LightBox {
    lenses: Vec<Lens>, // may need a linked list if splicing gets intense
}

fn gold(input: &str) -> usize {
    let mut boxes: Vec<LightBox> = vec![LightBox::default(); 256];

    for instruction in input.trim().split(',') {
        match Op::from_str(instruction) {
            Op::Set { id, focal_length } => {
                let index = hash(id.as_bytes());

                if let Some(lens_position) = boxes[index].lenses.iter().position(|elem| elem.id == id) {
                    println!("[MODIFY] lens '{id}' at index {index} (new focal: {focal_length})   ({instruction})");
                    boxes[index].lenses[lens_position].focal_length = focal_length;
                } else {
                    println!("[INSERT] new lens with id '{id}' to index {index} (focal: {focal_length})   ({instruction})");
                    boxes[index].lenses.push(Lens::new(&id, focal_length));
                }
            },
            Op::Remove { id } => {
                let index = hash(id.as_bytes());

                if let Some(lens_position) = boxes[index].lenses.iter().position(|elem| elem.id == id) {
                    println!("[REMOVE] lens with id '{id}' from box {index}   ({instruction})");
                    boxes[index].lenses.remove(lens_position);
                } else {
                    println!("[NOOP]   did not find lens with id '{id}' in box {index}   ({instruction})");
                }
            },
        }
    }
    
    calculate_focusing_power(&boxes)
}

fn silver(input: &str) -> usize {
    let mut sum = 0;
    for part in input.trim().split(',') {
        sum += hash(part.as_bytes());
    }
    sum
}

fn main() -> anyhow::Result<()> {
    let input = read_input()?;

    println!("Silver: {}", silver(&input));
    println!("  Gold: {}", gold(&input));

    Ok(())
}

fn calculate_focusing_power(boxes: &[LightBox]) -> usize {
    let mut sum: usize = 0;

    for (lightbox, box_n) in boxes.into_iter().zip(1..) {
        for (lens, lens_n) in lightbox.lenses.iter().zip(1..) {
            sum += box_n * lens_n * lens.focal_length as usize;
        }
    }

    sum
}

fn hash(s: &[u8]) -> usize {
    let mut hash: usize = 0;

    for &val in s {
        hash += val as usize;
        hash *= 17;
        hash %= 256;
    }

    hash
}
