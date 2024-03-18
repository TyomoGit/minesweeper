use std::fmt::Display;

use crate::{cell::Cell, error::{self, Error}};

pub struct Game {
    opened: Vec<Vec<bool>>,
    cells: Vec<Vec<Cell>>,
    unopened_count: usize,
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "   | ")?;
        for col in 0..self.cells[0].len() {
            let apha = (b'a' + col as u8) as char;
            write!(f, "{} ", apha)?;
        }
        writeln!(f)?;

        write!(f, "---+-")?;
        for _ in 0..self.cells[0].len() {
            write!(f, "--")?;
        }
        writeln!(f)?;

        for row in 0..self.cells.len() {
            write!(f, "{:2} | ", row)?;
            for col in 0..self.cells[0].len() {
                if self.opened[row][col] {
                    write!(f, "{} ", self.cells[row][col])?;
                } else {
                    write!(f, "◼️ ")?;
                }
            }


            if row != self.cells.len() - 1 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

impl Game {
    pub fn new(n_rows: usize, n_cols: usize, n_bombs: usize, init_row: usize, init_col: usize) -> Self {
        let mut cells: Vec<Vec<Cell>> = vec![vec![Cell::Safe(0); n_cols]; n_rows];

        make_bomb(&mut cells, n_bombs, init_row, init_col);

        init_cells(&mut cells);

        Self { opened: vec![vec![false; n_cols]; n_rows], cells, unopened_count: n_rows * n_cols - n_bombs }
    }

    pub fn open(&mut self, row: usize, col: usize) -> Result<(), Error> {
        let result = self.opened.get(row).and_then(|row| row.get(col));
        if let Some(true) = result {
            return Err(Error::AlreadyOpened);
        } else if result.is_none() {
            dbg!(row, col);
            return Err(Error::OutOfBounds);
        }

        self.opened[row][col] = true;
        self.unopened_count -= 1;

        if matches!(self.cells[row][col], Cell::Bomb) {
            return Err(Error::Lose);
        }

        if self.unopened_count == 0 {
            return Err(Error::Win);
        }

        if !matches!(self.cells[row][col], Cell::Safe(0)) {
            return Ok(());
        }

        for dy in -1..=1 {
            for dx in -1..=1 {
                if dy == 0 && dx == 0 {
                    continue;
                }

                let Ok(row): Result<usize, _> = (row as i32 + dy).try_into() else {
                    continue;
                };
                let Ok(col): Result<usize, _> = (col as i32 + dx).try_into() else {
                    continue;
                };

                if self.opened.get(row).and_then(|row| row.get(col)).is_none() {
                    continue;
                }

                let result = self.open(row, col);
                if !matches!(result, Ok(()) | Err(error::Error::AlreadyOpened)) {
                    return result;
                }
            }
        }
        
        Ok(())
    }
}

fn make_bomb(cells: &mut Vec<Vec<Cell>>, mut n_bombs: usize, init_row: usize, init_col: usize) {
    let (n_rows, n_cols) = (cells.len(), cells[0].len());

    while n_bombs > 0 {
        let row = rand::random::<usize>() % n_rows;
        let col = rand::random::<usize>() % n_cols;

        if cells[row][col] == Cell::Bomb {
            continue;
        }

        if row == init_row && col == init_col {
            continue;
        }

        cells[row][col] = Cell::Bomb;

        n_bombs -= 1;
    }
}

fn init_cells(cells: &mut Vec<Vec<Cell>>) {
    for row in 0..cells.len() {
        for col in 0..cells[0].len() {
            let cell = cells[row][col];

            if cell == Cell::Bomb {
                continue;
            }

            cells[row][col] = Cell::Safe(count_bombs(cells, row, col));
        }
    }
}

fn count_bombs(cells: &[Vec<Cell>], row: usize, col: usize) -> u8 {
    let (row, col) = (row as i32, col as i32);
    let mut result = 0;

    for dy in -1..=1 {
        for dx in -1..=1 {
            if dy == 0 && dx == 0 {
                continue;
            }

            let Ok(row): Result<usize, _> = (row + dy).try_into() else {
                continue;
            };
            let Ok(col): Result<usize, _> = (col + dx).try_into() else {
                continue;
            };

            if cells.get(row).and_then(|row| row.get(col)) == Some(&Cell::Bomb) {
                result += 1;
            }
        }
    }

    result
}
