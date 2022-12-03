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

    for (one, two, three) in BufReader::new(file).lines().flatten().tuples::<(_, _, _)>() {
        let one: HashSet<char> = one.chars().collect();
        let two: HashSet<char> = two.chars().collect();
        let three: HashSet<char> = three.chars().collect();

        let intersection = one.intersection(&two).copied().collect::<HashSet<char>>();
        let mut intersection = intersection.intersection(&three);

        let badge = intersection
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
