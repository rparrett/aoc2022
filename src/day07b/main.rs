use std::{fmt::Display, fs::read_to_string};

use chumsky::prelude::*;
use indextree::{Arena, NodeId};

#[derive(Clone, Debug)]
enum CommandOutput {
    File { size: u32, name: String },
    Directory { name: String },
}

#[derive(Clone, Debug)]
struct Command {
    name: String,
    args: Vec<String>,
    output: Vec<CommandOutput>,
}

#[derive(Debug)]
enum FilesystemEntry {
    File { size: u32, name: String },
    Directory { name: String },
}
impl Display for FilesystemEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::File { name, size } => write!(f, "File({}, {})", name, size)?,
            Self::Directory { name } => write!(f, "Dir({})", name)?,
        }

        Ok(())
    }
}
impl From<CommandOutput> for FilesystemEntry {
    fn from(output: CommandOutput) -> Self {
        match output {
            CommandOutput::Directory { name } => Self::Directory { name },
            CommandOutput::File { name, size } => Self::File { name, size },
        }
    }
}

const CAPACITY: u32 = 70_000_000;
const MIN_FREE: u32 = 30_000_000;

fn parser() -> impl Parser<char, Vec<Command>, Error = Simple<char>> {
    let command = just("$ ").ignore_then(
        none_of(" \n")
            .repeated()
            .collect::<String>()
            .then_ignore(just(' ').or_not())
            .then(
                none_of(" \n")
                    .repeated()
                    .at_least(1)
                    .collect::<String>()
                    .separated_by(just(' ')),
            )
            .then_ignore(just('\n')),
    );

    let output = choice((
        just("dir ")
            .ignore_then(none_of("$\n").repeated().collect::<String>())
            .map(|name| CommandOutput::Directory { name }),
        text::digits(10)
            .from_str()
            .unwrapped()
            .then_ignore(just(' '))
            .then(none_of("$\n").repeated().collect::<String>())
            .map(|(size, name)| CommandOutput::File { size, name }),
    ))
    .then_ignore(just('\n').or_not())
    .repeated();

    command
        .then(output)
        .map(|((name, args), output)| Command { name, args, output })
        .repeated()
}

fn main() -> anyhow::Result<()> {
    let message = read_to_string("input/day07.txt")?;

    let filesystem = &mut Arena::new();

    let root = filesystem.new_node(FilesystemEntry::Directory {
        name: "/".to_string(),
    });
    let mut current_directory = root;

    let commands = parser().parse(message).unwrap();

    for command in commands {
        if command.name == "cd" {
            if let Some(arg) = command.args.get(0) {
                if arg == ".." {
                    if let Some(parent) = current_directory.ancestors(filesystem).nth(1) {
                        current_directory = parent;
                    }
                } else if arg == "/" {
                    current_directory = root;
                } else if let Some(node_id) = current_directory.children(filesystem).find(|child| {
                    match filesystem.get(*child).unwrap().get() {
                        FilesystemEntry::Directory { name } => *name == *arg,
                        _ => false,
                    }
                }) {
                    current_directory = node_id;
                }
            }
        } else if command.name == "ls" {
            for out in command.output {
                let entry = out.into();

                current_directory.append(filesystem.new_node(entry), filesystem);
            }
        }
    }

    println!("{}", root.debug_pretty_print(filesystem));

    let mut sizes = vec![];

    let total = get_directory_size(&root, filesystem);
    sizes.push((total, root));

    let needed = MIN_FREE - (CAPACITY - total);

    for d in root.descendants(filesystem).skip(1) {
        let size = get_directory_size(&d, filesystem);
        sizes.push((size, d));
    }

    let delete = sizes
        .iter()
        .filter(|(size, _)| *size >= needed)
        .max_by(|a, b| b.0.cmp(&a.0));

    println!("{:?}", delete);

    Ok(())
}

fn get_directory_size(node_id: &NodeId, filesystem: &Arena<FilesystemEntry>) -> u32 {
    let mut sum: u32 = 0;

    for child in node_id.children(filesystem) {
        match filesystem.get(child).unwrap().get() {
            FilesystemEntry::File { size, .. } => {
                sum += size;
            }
            FilesystemEntry::Directory { .. } => sum += get_directory_size(&child, filesystem),
        }
    }

    sum
}
