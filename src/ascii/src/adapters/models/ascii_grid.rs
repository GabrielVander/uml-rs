use std::fmt::Display;

use crate::adapters::models::position::Position;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct AsciiGrid {
    fill_char: char,
    grid: Vec<Vec<char>>,
}

impl AsciiGrid {
    pub(crate) fn new(fill_char: char, grid: Vec<Vec<char>>) -> Self {
        Self { fill_char, grid }
    }

    pub(crate) fn empty(fill_char: char) -> Self {
        Self {
            fill_char,
            grid: vec![],
        }
    }

    pub(crate) fn put_char(&mut self, ch: char, pos: &Position) {
        let x = pos.x as usize;
        let y = pos.y as usize;

        if y >= self.grid.len() {
            self.grid.resize_with(y + 1, Vec::new);
        }
        if x >= self.grid[y].len() {
            self.grid[y].resize(x + 1, ' ');
        }

        self.grid[y][x] = ch;
    }

    pub(crate) fn get_grid(&self) -> Vec<Vec<char>> {
        self.grid.clone()
    }
}

impl Display for AsciiGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for &ch in row {
                write!(f, "{}", ch)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
