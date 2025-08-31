use crate::adapters::models::{ascii_grid::AsciiGrid, position::Position};

pub trait AsciiElement {
    fn draw(&self, grid: &mut AsciiGrid);

    fn position(&self, grid: &AsciiGrid) -> Position;

    fn width(&self) -> i32;

    fn height(&self) -> i32;
}
