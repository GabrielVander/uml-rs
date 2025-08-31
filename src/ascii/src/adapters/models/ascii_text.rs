use crate::adapters::models::{
    ascii_element::AsciiElement, ascii_grid::AsciiGrid, position::Position,
};

pub struct AsciiText {
    content: String,
}

impl AsciiText {
    pub fn new(content: String) -> Self {
        Self { content }
    }
}

impl AsciiElement for AsciiText {
    fn draw(&self, grid: &mut AsciiGrid) {
        let pos = self.position(grid);

        for (x, ch) in self.content.chars().enumerate() {
            let char_position = Position::new(pos.x + x as i32, pos.y);

            grid.put_char(ch, &char_position);
        }
    }

    fn position(&self, _grid: &AsciiGrid) -> Position {
        Position::default()
    }

    fn width(&self) -> i32 {
        self.content.len() as i32
    }

    fn height(&self) -> i32 {
        1
    }
}
