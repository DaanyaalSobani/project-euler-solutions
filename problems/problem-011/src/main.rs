use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
// use std::error::Error;
use anyhow::{Context, Ok, Result, bail};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Right,
    Down,
    DownRight,
    DownLeft,
}

#[derive(Debug, PartialEq)]
struct BestRun {
    product: u64,
    start: (usize, usize),
    direction: Direction,
}
#[derive(Debug, Clone)]
struct Grid {
    cells: Vec<u64>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn new(cells: Vec<u64>, rows: usize, cols: usize) -> Result<Self> {
        if cells.len() != rows * cols {
            let len = cells.len();
            let expected = rows * cols;
            bail!("cells.len() ({len}) != rows * cols ({rows} * {cols} = {expected})");
        }
        Ok(Grid { cells, rows, cols })
    }

    fn from_string(str: &str) -> Result<Self> {
        let mut cells: Vec<u64> = Vec::new();
        let mut rows: usize = 0;
        let mut cols: usize = 0;
        for line in str.lines() {
            let row: Vec<u64> = line
                .trim()
                .split_whitespace()
                .map(|c| c.parse::<u64>())
                .collect::<Result<_, _>>()
                .with_context(|| format!("on line {}", rows + 1))?;
            if rows == 0 {
                cols = row.len();
            } else if row.len() != cols {
                bail!("inconsistent columns in row {}", rows + 1);
            }
            cells.extend(row);
            rows += 1;
        }
        Self::new(cells, rows, cols)
    }
    fn from_file(path: &Path) -> Result<Self> {
        let mut file = File::open(path)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        Self::from_string(&buf)
    }

    fn iter_indexed(&self) -> impl Iterator<Item = (usize, usize, u64)> + '_ {
        let cols = self.cols;
        self.cells
            .iter()
            .enumerate()
            .map(move |(idx, &v)| (idx / cols, idx % cols, v))
    }
}

fn main() -> Result<()> {
    let path_str = concat!(env!("CARGO_MANIFEST_DIR"), "/input_grid.txt");
    let input_grid_txt = Path::new(path_str);
    let grid = Grid::from_file(input_grid_txt)?;

    println!("{:?}", grid);

    grid.iter_indexed()
        .for_each(|(x, y, v)| println!("{x},{y}:{v}"));

    println!("done!");
    Ok(())
}
