use std::{fs::File, io::BufRead, io::BufReader};

fn main() -> anyhow::Result<()> {
    let mut elves = vec![];
    let mut current_elf: u32 = 0;

    let file = File::open("input/day01.txt")?;

    for line in BufReader::new(file).lines().flatten() {
        if let Ok(num) = line.parse::<u32>() {
            current_elf += num;
        } else {
            elves.push(current_elf);
            current_elf = 0;
        }
    }

    elves.sort_by(|a, b| b.cmp(a));

    let top_three: u32 = elves.iter().take(3).sum();

    println!("{}", top_three);

    Ok(())
}
