use anyhow::anyhow;
use std::{fs::File, io::BufRead, io::BufReader};

#[derive(Debug)]
struct ElfRange {
    start: u32,
    end: u32,
}

impl ElfRange {
    fn intersection(&self, other: &ElfRange) -> ElfRange {
        ElfRange {
            start: self.start.max(other.start),
            end: self.end.min(other.end),
        }
    }

    fn len(&self) -> u32 {
        (self.end + 1).saturating_sub(self.start)
    }

    fn overlaps(&self, other: &ElfRange) -> bool {
        self.intersection(other).len() > 0
    }
}

impl TryFrom<&str> for ElfRange {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (start, end) = value
            .split_once('-')
            .ok_or_else(|| anyhow!("invalid range format"))?;

        let start: u32 = start.parse()?;
        let end: u32 = end.parse()?;

        Ok(ElfRange { start, end })
    }
}

fn main() -> anyhow::Result<()> {
    let mut total: u32 = 0;

    let file = File::open("input/day04.txt")?;

    for line in BufReader::new(file).lines().flatten() {
        let pairs: anyhow::Result<Vec<ElfRange>> =
            line.split(',').map(ElfRange::try_from).collect();
        let pairs = pairs?;

        let (Some(one), Some(two)) = (pairs.get(0), pairs.get(1)) else {
            return Err(anyhow!("invalid elf pair format"));
        };

        if one.overlaps(two) {
            total += 1;
        }
    }

    println!("{}", total);

    Ok(())
}
