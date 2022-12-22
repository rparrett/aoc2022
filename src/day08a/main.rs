use anyhow::anyhow;
use std::{fmt::Display, fs::File, io::BufRead, io::BufReader};

#[derive(Default)]
struct Grid(Vec<Vec<u32>>);
impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.rows() {
            for x in 0..self.cols() {
                write!(f, "{}", self.0[y][x])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
impl Grid {
    fn rows(&self) -> usize {
        self.0.len()
    }
    fn cols(&self) -> usize {
        self.0[0].len()
    }
    fn zeros(rows: usize, cols: usize) -> Self {
        Grid(vec![vec![0; cols]; rows])
    }
}

fn main() -> anyhow::Result<()> {
    let file = File::open("input/day08.txt")?;

    let mut grid = Grid::default();
    for line in BufReader::new(file).lines().flatten() {
        let row = line
            .chars()
            .map(|c| c.to_digit(10).ok_or_else(|| anyhow!("invalid tree height")))
            .collect::<anyhow::Result<Vec<u32>>>()?;
        grid.0.push(row);
    }

    let rows = grid.rows();
    let cols = grid.cols();

    let mut visibility_grid = Grid::zeros(rows, cols);

    for y in 0..rows {
        // west -> east
        let mut max: Option<u32> = None;
        for x in 0..cols {
            if max.filter(|v| grid.0[y][x] <= *v).is_none() {
                max = Some(grid.0[y][x]);
                visibility_grid.0[y][x] += 1;
            }
        }

        // east -> west
        let mut max: Option<u32> = None;
        for x in (0..cols).rev() {
            if max.filter(|v| grid.0[y][x] <= *v).is_none() {
                max = Some(grid.0[y][x]);
                visibility_grid.0[y][x] += 1;
            }
        }
    }

    for x in 0..cols {
        // north -> south
        let mut max: Option<u32> = None;
        for y in 0..rows {
            if max.filter(|v| grid.0[y][x] <= *v).is_none() {
                max = Some(grid.0[y][x]);
                visibility_grid.0[y][x] += 1;
            }
        }

        // south -> north
        let mut max: Option<u32> = None;
        for y in (0..rows).rev() {
            if max.filter(|v| grid.0[y][x] <= *v).is_none() {
                max = Some(grid.0[y][x]);
                visibility_grid.0[y][x] += 1;
            }
        }
    }

    println!();
    println!("{}", visibility_grid);

    let num_visible = visibility_grid
        .0
        .iter()
        .flat_map(|row| row.iter())
        .filter(|v| **v != 0)
        .count();

    println!();
    println!("num visible: {}", num_visible);

    Ok(())
}
