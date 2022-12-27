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
    fn enumerate_rows(
        &self,
    ) -> impl Iterator<Item = impl Iterator<Item = (usize, usize, &u32)> + Clone + '_> {
        (0..self.rows())
            .map(move |row| (0..self.cols()).map(move |col| (row, col, &self.0[row][col])))
    }
    fn enumerate_cols(
        &self,
    ) -> impl Iterator<Item = impl Iterator<Item = (usize, usize, &u32)> + '_> {
        (0..self.cols())
            .map(move |col| (0..self.rows()).map(move |row| (row, col, &self.0[row][col])))
    }
    fn iter(&self) -> impl Iterator<Item = &u32> + '_ {
        self.0.iter().flat_map(|row| row.iter())
    }
}

fn trees_visible<'a>(val: u32, line_of_sight: impl Iterator<Item = &'a u32>) -> u32 {
    let mut dist = 0;
    for val2 in line_of_sight {
        if val > *val2 {
            dist += 1;
        } else {
            dist += 1;
            break;
        }
    }
    dist
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

    grid.enumerate_rows().for_each(|line| {
        line.skip(1).for_each(|(row, col, val)| {
            let dist_left = trees_visible(*val, (0..col).rev().map(|col2| &grid.0[row][col2]));
            visibility_grid.0[row][col] = dist_left;

            let dist_right = trees_visible(*val, ((col + 1)..cols).map(|col2| &grid.0[row][col2]));
            visibility_grid.0[row][col] *= dist_right;
        });
    });

    grid.enumerate_cols().for_each(|line| {
        line.skip(1).for_each(|(row, col, val)| {
            let dist_up = trees_visible(*val, (0..row).rev().map(|row2| &grid.0[row2][col]));
            visibility_grid.0[row][col] *= dist_up;

            let dist_down = trees_visible(*val, ((row + 1)..rows).map(|row2| &grid.0[row2][col]));
            visibility_grid.0[row][col] *= dist_down;
        })
    });

    let most_visibility = visibility_grid.iter().max();

    println!("most visibility: {:?}", most_visibility);

    Ok(())
}
