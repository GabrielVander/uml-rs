use crate::adapters::models::position::Position;

#[derive(Debug, Clone, PartialEq)]
pub struct AsciiGrid {
    fill_char: char,
    grid: Vec<Vec<char>>,
}

impl AsciiGrid {
    pub fn new(fill_char: char, grid: Vec<Vec<char>>) -> Self {
        Self { fill_char, grid }
    }

    pub fn empty(fill_char: char) -> Self {
        Self {
            fill_char,
            grid: vec![],
        }
    }

    pub fn put_char(&mut self, ch: char, pos: &Position) {
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

    pub fn get_grid(&self) -> Vec<Vec<char>> {
        self.grid.clone()
    }
}
