use anyhow::anyhow;
use itertools::Itertools;
use std::{collections::VecDeque, fs::File, io::BufRead, io::BufReader};

fn main() -> anyhow::Result<()> {
    let mut stacks: Vec<VecDeque<char>> = vec![];

    let file = File::open("input/day05.txt")?;

    let mut lines = BufReader::new(file).lines().flatten();

    for line in lines.by_ref().take_while(|l| !l.is_empty()) {
        let chars = line.chars().skip(1).step_by(4).enumerate();
        for (i, c) in chars.filter(|(_, c)| c.is_ascii_alphabetic()) {
            while stacks.len() <= i {
                stacks.push(VecDeque::new());
            }
            stacks[i].push_back(c);
        }
    }

    for line in lines {
        let Some((qty, from, to)) = line
            .split(' ')
            .filter_map(|chunk| chunk.parse::<usize>().ok())
            .collect_tuple() else {
                return Err(anyhow!("invalid instruction format"));
            };

        let from = from - 1;
        let to = to - 1;

        let moving: Vec<_> = stacks[from].drain(0..qty).collect();
        for m in moving.iter().rev() {
            stacks[to].push_front(*m);
        }
    }

    let top: String = stacks.iter().map(|s| s[0]).collect();

    println!("{}", top);

    Ok(())
}
