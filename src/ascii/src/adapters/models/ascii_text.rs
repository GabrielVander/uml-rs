use crate::adapters::models::{
    ascii_element::AsciiElement, ascii_grid::AsciiGrid, position::Position,
};

pub(crate) struct AsciiText {
    position: Position,
    content: String,
}

impl AsciiText {
    pub(crate) fn new(position: Position, content: String) -> Self {
        Self { position, content }
    }
}

impl AsciiElement for AsciiText {
    fn draw(&mut self, grid: &mut AsciiGrid) {
        let initial_pos: Position = self.position();

        for (y, line) in self.content.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let pos = Position::new(
                    initial_pos.x + x.try_into().unwrap_or(0),
                    initial_pos.y + y.try_into().unwrap_or(0),
                );

                grid.put_char(ch, &pos);
            }
        }
    }

    fn position(&self) -> Position {
        self.position.clone()
    }

    fn width(&self) -> u16 {
        self.content
            .lines()
            .map(|line| line.chars().count())
            .max()
            .unwrap_or(0)
            .try_into()
            .unwrap_or(0)
    }

    fn height(&self) -> u16 {
        self.content.lines().count().try_into().unwrap_or(0)
    }

    fn r#move(&mut self, new_position: Position) {
        self.position = new_position;
    }
}
