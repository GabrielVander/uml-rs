#[derive(Debug, Clone, PartialEq)]
pub struct AsciiGridViewModel {
    pub grid: Vec<Vec<char>>,
}

impl AsciiGridViewModel {
    pub fn new(grid: Vec<Vec<char>>) -> Self {
        Self { grid }
    }

    pub fn empty() -> Self {
        Self { grid: vec![] }
    }
}

impl ToString for AsciiGridViewModel {
    fn to_string(&self) -> String {
        self.grid
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

#[macro_export]
macro_rules! ascii_grid_model {
    ($cells:expr) => {{
        let received_cells: Vec<Vec<char>> = $cells;

        AsciiGridViewModel::new(received_cells)
    }};
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_ascii_grid_macro() {
        let grid: AsciiGridViewModel = ascii_grid_model!(vec![
            vec!['A'.to_owned(), 'B'.to_owned()],
            vec!['C'.to_owned(), 'D'.to_owned()],
        ]);

        assert_eq!(grid.grid.len(), 2);
        assert_eq!(grid.grid[0].len(), 2);

        assert_eq!(grid.grid[0][0], 'A');
        assert_eq!(grid.grid[0][1], 'B');
        assert_eq!(grid.grid[1][0], 'C');
        assert_eq!(grid.grid[1][1], 'D');
    }
}
