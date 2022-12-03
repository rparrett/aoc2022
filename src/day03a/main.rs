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

    for mut line in BufReader::new(file).lines().flatten() {
        let compartment_two: HashSet<char> = line.split_off(line.len() / 2).chars().collect();
        let compartment_one: HashSet<char> = line.chars().collect();

        let intersection = compartment_one.intersection(&compartment_two);

        let sum: u32 = intersection.map(|c| priority(*c) as u32).sum();
        total += sum;
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
