use anyhow::anyhow;
use std::{fmt::Display, fs::File, io::BufRead, io::BufReader};

#[derive(Default)]
struct Grid(Vec<Vec<u32>>);
impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.rows() {
            for x in 0..self.cols() {
                write!(f, "{:4}", self.0[y][x])?;
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
        for x in 1..cols {
            let mut dist = 0;
            for x2 in (0..x).rev() {
                if grid.0[y][x] > grid.0[y][x2] {
                    dist += 1;
                } else {
                    dist += 1;
                    break;
                }
            }
            visibility_grid.0[y][x] = dist;
        }

        // east -> west
        for x in (0..(cols - 1)).rev() {
            let mut dist = 0;
            for x2 in (x + 1)..cols {
                if grid.0[y][x] > grid.0[y][x2] {
                    dist += 1;
                } else {
                    dist += 1;
                    break;
                }
            }
            visibility_grid.0[y][x] *= dist;
        }
    }

    for x in 0..cols {
        // north -> south
        for y in 1..rows {
            let mut dist = 0;
            for y2 in (0..y).rev() {
                if grid.0[y][x] > grid.0[y2][x] {
                    dist += 1;
                } else {
                    dist += 1;
                    break;
                }
            }
            visibility_grid.0[y][x] *= dist;
        }

        // south -> north
        for y in (0..(cols - 1)).rev() {
            let mut dist = 0;
            for y2 in (y + 1)..cols {
                if grid.0[y][x] > grid.0[y2][x] {
                    dist += 1;
                } else {
                    dist += 1;
                    break;
                }
            }
            visibility_grid.0[y][x] *= dist;
        }
    }

    let most_visibility = visibility_grid.0.iter().flat_map(|row| row.iter()).max();

    println!("most visibility: {:?}", most_visibility);

    Ok(())
}
