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
    fn enumerate_rows(
        &self,
    ) -> impl Iterator<Item = impl Iterator<Item = (usize, usize, &u32)> + '_> {
        (0..self.rows())
            .map(move |row| (0..self.cols()).map(move |col| (row, col, &self.0[row][col])))
    }
    fn enumerate_rev_rows(
        &self,
    ) -> impl Iterator<Item = impl Iterator<Item = (usize, usize, &u32)> + '_> {
        (0..self.rows()).map(move |row| {
            (0..self.cols())
                .rev()
                .map(move |col| (row, col, &self.0[row][col]))
        })
    }
    fn enumerate_cols(
        &self,
    ) -> impl Iterator<Item = impl Iterator<Item = (usize, usize, &u32)> + '_> {
        (0..self.cols())
            .map(move |col| (0..self.rows()).map(move |row| (row, col, &self.0[row][col])))
    }
    fn enumerate_rev_cols(
        &self,
    ) -> impl Iterator<Item = impl Iterator<Item = (usize, usize, &u32)> + '_> {
        (0..self.cols()).map(move |col| {
            (0..self.rows())
                .rev()
                .map(move |row| (row, col, &self.0[row][col]))
        })
    }
    fn iter(&self) -> impl Iterator<Item = &u32> + '_ {
        self.0.iter().flat_map(|row| row.iter())
    }
}

fn add_line_visibility<'a>(
    line: impl Iterator<Item = (usize, usize, &'a u32)>,
    visibility: &mut Grid,
) {
    let mut max: Option<u32> = None;
    for (row, col, val) in line {
        if max.filter(|max_v| *val <= *max_v).is_none() {
            max = Some(*val);
            visibility.0[row][col] += 1;
        }
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

    let mut visibility = Grid::zeros(rows, cols);

    grid.enumerate_rows()
        .for_each(|line| add_line_visibility(line, &mut visibility));
    grid.enumerate_rev_rows()
        .for_each(|line| add_line_visibility(line, &mut visibility));
    grid.enumerate_cols()
        .for_each(|line| add_line_visibility(line, &mut visibility));
    grid.enumerate_rev_cols()
        .for_each(|line| add_line_visibility(line, &mut visibility));

    println!();
    println!("{}", visibility);

    let num_visible = visibility.iter().filter(|v| **v != 0).count();

    println!();
    println!("num visible: {}", num_visible);

    Ok(())
}
