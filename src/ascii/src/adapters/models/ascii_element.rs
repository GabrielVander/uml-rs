use crate::adapters::models::{ascii_grid::AsciiGrid, position::Position};

pub(crate) trait AsciiElement {
    fn draw(&mut self, grid: &mut AsciiGrid);

    fn position(&self) -> Position;

    fn width(&self) -> u16;

    fn height(&self) -> u16;

    fn r#move(&mut self, new_position: Position);
}
