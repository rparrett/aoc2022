use std::{fs::File, io::BufRead, io::BufReader};

fn main() -> anyhow::Result<()> {
    let mut max: u32 = 0;
    let mut current_elf: u32 = 0;

    let file = File::open("input/day01.txt")?;

    for line in BufReader::new(file).lines().flatten() {
        if let Ok(num) = line.parse::<u32>() {
            current_elf += num;
        } else {
            current_elf = 0;
        }

        if current_elf > max {
            max = current_elf;
        }
    }

    println!("{}", max);

    Ok(())
}
