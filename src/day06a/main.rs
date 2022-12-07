use itertools::Itertools;
use std::fs::read_to_string;

const WINDOW_SIZE: usize = 4;

fn main() -> anyhow::Result<()> {
    let message = read_to_string("input/day06.txt")?;

    for start in 0..(message.len() - WINDOW_SIZE) {
        let end = start + WINDOW_SIZE;

        let window = &message[start..end];
        if !window.chars().combinations(2).any(|c| c[0] == c[1]) {
            println!("{}", end);
            break;
        }
    }

    Ok(())
}
