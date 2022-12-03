use anyhow::anyhow;
use itertools::Itertools;
use std::{collections::HashSet, fs::File, io::BufRead, io::BufReader};

fn priority(c: char) -> u32 {
    if c.is_ascii_uppercase() {
        c as u32 - 'A' as u32 + 27
    } else {
        c as u32 - 'a' as u32 + 1
    }
}

fn main() -> anyhow::Result<()> {
    let mut total: u32 = 0;

    let file = File::open("input/day03.txt")?;

    for chunk in BufReader::new(file).lines().flatten().chunks(3).into_iter() {
        let intersection = chunk
            .map(|pack| pack.chars().collect::<HashSet<char>>())
            .reduce(|int, set| set.intersection(&int).cloned().collect())
            .unwrap();

        let badge = intersection
            .iter()
            .next()
            .ok_or_else(|| anyhow!("badge not found"))?;

        total += priority(*badge);
    }

    println!("{}", total);

    Ok(())
}

#[test]
fn test_priority() {
    assert_eq!(priority('a'), 1);
    assert_eq!(priority('z'), 26);
    assert_eq!(priority('A'), 27);
    assert_eq!(priority('Z'), 52);
}
