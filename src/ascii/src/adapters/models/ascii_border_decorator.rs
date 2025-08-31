use crate::adapters::models::{
    ascii_element::AsciiElement, ascii_grid::AsciiGrid, position::Position,
};

struct AsciiBorderDecorator {
    wrapped: Box<dyn AsciiElement>,
    fill_char: char,
    horizontal_padding: u16,
    vertical_padding: u16,
    position: Position,
}

impl AsciiBorderDecorator {
    pub(crate) fn new(
        wrapped: Box<dyn AsciiElement>,
        position: Position,
        horizontal_padding: u16,
        vertical_padding: u16,
        fill_char: char,
    ) -> Self {
        Self {
            wrapped,
            fill_char,
            horizontal_padding,
            vertical_padding,
            position,
        }
    }
}

impl AsciiElement for AsciiBorderDecorator {
    fn draw(&mut self, grid: &mut super::ascii_grid::AsciiGrid) {
        let initial_position: Position = self.position();

        let width: u16 = self.width();
        let height: u16 = self.height();

        for y in initial_position.y..(initial_position.y + height as i32) {
            for x in initial_position.x..(initial_position.x + width as i32) {
                let pos = Position::new(x, y);
                let mut ch: char = self.fill_char;

                if pos == initial_position {
                    ch = '╭';
                } else if pos
                    == Position::new(initial_position.x, initial_position.y + height as i32 - 1)
                {
                    ch = '╰'
                } else if pos
                    == Position::new(initial_position.x + width as i32 - 1, initial_position.y)
                {
                    ch = '╮';
                } else if pos
                    == Position::new(
                        initial_position.x + width as i32 - 1,
                        initial_position.y + height as i32 - 1,
                    )
                {
                    ch = '╯';
                } else if (y == initial_position.y) || (y == initial_position.y + height as i32 - 1)
                {
                    ch = '─';
                } else if (x == initial_position.x) || (x == initial_position.x + width as i32 - 1)
                {
                    ch = '│';
                }

                grid.put_char(ch, &pos);
            }
        }

        let content_start: Position = Position::new(
            initial_position.x + self.horizontal_padding as i32 + 1,
            initial_position.y + self.vertical_padding as i32 + 1,
        );

        self.wrapped.r#move(content_start);
        self.wrapped.draw(grid);
    }

    fn position(&self) -> Position {
        self.position.clone()
    }

    fn width(&self) -> u16 {
        self.wrapped.width() + (self.horizontal_padding * 2) + 2
    }

    fn height(&self) -> u16 {
        self.wrapped.height() + (self.vertical_padding * 2) + 2
    }

    fn r#move(&mut self, new_position: Position) {
        self.position = new_position;
    }
}

#[cfg(test)]
mod test {
    use crate::adapters::models::{ascii_grid::AsciiGrid, ascii_text::AsciiText};
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn empty_text_no_padding() {
        let mut grid: AsciiGrid = AsciiGrid::empty(' ');
        let text: Box<dyn AsciiElement> =
            Box::new(AsciiText::new(Position::default(), String::default()));
        let mut decorator: AsciiBorderDecorator =
            AsciiBorderDecorator::new(text, Position::default(), 0, 0, ' ');

        decorator.draw(&mut grid);

        assert_eq!(
            grid.to_string(),
            AsciiGrid::new(' ', vec![vec!['╭', '╮'], vec!['╰', '╯'],]).to_string()
        )
    }

    #[test]
    fn empty_text_with_padding() {
        let mut grid: AsciiGrid = AsciiGrid::empty(' ');
        let text: Box<dyn AsciiElement> =
            Box::new(AsciiText::new(Position::default(), String::default()));
        let mut decorator: AsciiBorderDecorator =
            AsciiBorderDecorator::new(text, Position::default(), 3, 3, ' ');

        decorator.draw(&mut grid);

        assert_eq!(
            grid.to_string(),
            AsciiGrid::new(
                ' ',
                vec![
                    vec!['╭', '─', '─', '─', '─', '─', '─', '╮'],
                    vec!['│', ' ', ' ', ' ', ' ', ' ', ' ', '│'],
                    vec!['│', ' ', ' ', ' ', ' ', ' ', ' ', '│'],
                    vec!['│', ' ', ' ', ' ', ' ', ' ', ' ', '│'],
                    vec!['│', ' ', ' ', ' ', ' ', ' ', ' ', '│'],
                    vec!['│', ' ', ' ', ' ', ' ', ' ', ' ', '│'],
                    vec!['│', ' ', ' ', ' ', ' ', ' ', ' ', '│'],
                    vec!['╰', '─', '─', '─', '─', '─', '─', '╯'],
                ]
            )
            .to_string()
        )
    }

    #[test]
    fn short_text_no_padding() {
        let mut grid: AsciiGrid = AsciiGrid::empty(' ');
        let text: Box<dyn AsciiElement> =
            Box::new(AsciiText::new(Position::default(), "Short".to_string()));
        let mut decorator: AsciiBorderDecorator =
            AsciiBorderDecorator::new(text, Position::default(), 0, 0, ' ');

        decorator.draw(&mut grid);

        assert_eq!(
            grid.to_string(),
            AsciiGrid::new(
                ' ',
                vec![
                    vec!['╭', '─', '─', '─', '─', '─', '╮'],
                    vec!['│', 'S', 'h', 'o', 'r', 't', '│'],
                    vec!['╰', '─', '─', '─', '─', '─', '╯'],
                ]
            )
            .to_string()
        )
    }

    #[test]
    fn short_text_with_padding() {
        let mut grid: AsciiGrid = AsciiGrid::empty(' ');
        let text: Box<dyn AsciiElement> =
            Box::new(AsciiText::new(Position::default(), "Short".to_string()));
        let mut decorator: AsciiBorderDecorator =
            AsciiBorderDecorator::new(text, Position::default(), 3, 3, ' ');

        decorator.draw(&mut grid);

        assert_eq!(
            grid.to_string(),
            AsciiGrid::new(
                ' ',
                vec![
                    vec![
                        '╭', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '╮'
                    ],
                    vec![
                        '│', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '│'
                    ],
                    vec![
                        '│', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '│'
                    ],
                    vec![
                        '│', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '│'
                    ],
                    vec![
                        '│', ' ', ' ', ' ', 'S', 'h', 'o', 'r', 't', ' ', ' ', ' ', '│'
                    ],
                    vec![
                        '│', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '│'
                    ],
                    vec![
                        '│', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '│'
                    ],
                    vec![
                        '│', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '│'
                    ],
                    vec![
                        '╰', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '╯'
                    ],
                ]
            )
            .to_string()
        )
    }

    #[test]
    fn long_text_no_padding() {
        let mut grid: AsciiGrid = AsciiGrid::empty(' ');
        let text: Box<dyn AsciiElement> = Box::new(AsciiText::new(
            Position::default(),
            "ThisIsAnAttemptAtBeingQuiteALongText".to_string(),
        ));
        let mut decorator: AsciiBorderDecorator =
            AsciiBorderDecorator::new(text, Position::default(), 0, 0, ' ');

        decorator.draw(&mut grid);

        assert_eq!(
            grid.to_string(),
            AsciiGrid::new(
                ' ',
                vec![
                    vec![
                        '╭', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─',
                        '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─',
                        '─', '─', '─', '─', '─', '─', '─', '╮'
                    ],
                    vec![
                        '│', 'T', 'h', 'i', 's', 'I', 's', 'A', 'n', 'A', 't', 't', 'e', 'm', 'p',
                        't', 'A', 't', 'B', 'e', 'i', 'n', 'g', 'Q', 'u', 'i', 't', 'e', 'A', 'L',
                        'o', 'n', 'g', 'T', 'e', 'x', 't', '│'
                    ],
                    vec![
                        '╰', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─',
                        '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─',
                        '─', '─', '─', '─', '─', '─', '─', '╯'
                    ],
                ]
            )
            .to_string()
        )
    }

    #[test]
    fn long_text_with_padding() {
        let mut grid: AsciiGrid = AsciiGrid::empty(' ');
        let text: Box<dyn AsciiElement> = Box::new(AsciiText::new(
            Position::default(),
            "ThisIsAnAttemptAtBeingQuiteALongText".to_string(),
        ));
        let mut decorator: AsciiBorderDecorator =
            AsciiBorderDecorator::new(text, Position::default(), 3, 3, ' ');

        decorator.draw(&mut grid);

        assert_eq!(
            grid.to_string(),
            AsciiGrid::new(
                ' ',
                vec![
                    vec![
                        '╭', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─',
                        '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─',
                        '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '╮'
                    ],
                    vec![
                        '│', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '│'
                    ],
                    vec![
                        '│', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '│'
                    ],
                    vec![
                        '│', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '│'
                    ],
                    vec![
                        '│', ' ', ' ', ' ', 'T', 'h', 'i', 's', 'I', 's', 'A', 'n', 'A', 't', 't',
                        'e', 'm', 'p', 't', 'A', 't', 'B', 'e', 'i', 'n', 'g', 'Q', 'u', 'i', 't',
                        'e', 'A', 'L', 'o', 'n', 'g', 'T', 'e', 'x', 't', ' ', ' ', ' ', '│'
                    ],
                    vec![
                        '│', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '│'
                    ],
                    vec![
                        '│', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '│'
                    ],
                    vec![
                        '│', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '│'
                    ],
                    vec![
                        '╰', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─',
                        '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─',
                        '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '╯'
                    ],
                ]
            )
            .to_string()
        )
    }

    #[test]
    fn multiline_text_no_padding() {
        let mut grid: AsciiGrid = AsciiGrid::empty(' ');
        let text: Box<dyn AsciiElement> = Box::new(AsciiText::new(
            Position::default(),
            "Some\nmultiline\ntext".to_string(),
        ));
        let mut decorator: AsciiBorderDecorator =
            AsciiBorderDecorator::new(text, Position::default(), 0, 0, ' ');

        decorator.draw(&mut grid);

        assert_eq!(
            grid.to_string(),
            AsciiGrid::new(
                ' ',
                vec![
                    vec!['╭', '─', '─', '─', '─', '─', '─', '─', '─', '─', '╮'],
                    vec!['│', 'S', 'o', 'm', 'e', ' ', ' ', ' ', ' ', ' ', '│'],
                    vec!['│', 'm', 'u', 'l', 't', 'i', 'l', 'i', 'n', 'e', '│'],
                    vec!['│', 't', 'e', 'x', 't', ' ', ' ', ' ', ' ', ' ', '│'],
                    vec!['╰', '─', '─', '─', '─', '─', '─', '─', '─', '─', '╯'],
                ]
            )
            .to_string()
        )
    }

    #[test]
    fn multiline_text_with_padding() {
        let mut grid: AsciiGrid = AsciiGrid::empty(' ');
        let text: Box<dyn AsciiElement> = Box::new(AsciiText::new(
            Position::default(),
            "Some\nmultiline\ntext".to_string(),
        ));
        let mut decorator: AsciiBorderDecorator =
            AsciiBorderDecorator::new(text, Position::default(), 3, 3, ' ');

        decorator.draw(&mut grid);

        assert_eq!(
            grid.to_string(),
            AsciiGrid::new(
                ' ',
                vec![
                    vec![
                        '╭', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─',
                        '─', '╮'
                    ],
                    vec![
                        '│', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                        ' ', '│'
                    ],
                    vec![
                        '│', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                        ' ', '│'
                    ],
                    vec![
                        '│', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                        ' ', '│'
                    ],
                    vec![
                        '│', ' ', ' ', ' ', 'S', 'o', 'm', 'e', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                        ' ', '│'
                    ],
                    vec![
                        '│', ' ', ' ', ' ', 'm', 'u', 'l', 't', 'i', 'l', 'i', 'n', 'e', ' ', ' ',
                        ' ', '│'
                    ],
                    vec![
                        '│', ' ', ' ', ' ', 't', 'e', 'x', 't', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                        ' ', '│'
                    ],
                    vec![
                        '│', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                        ' ', '│'
                    ],
                    vec![
                        '│', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                        ' ', '│'
                    ],
                    vec![
                        '│', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                        ' ', '│'
                    ],
                    vec![
                        '╰', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─',
                        '─', '╯'
                    ],
                ]
            )
            .to_string()
        )
    }

    #[test]
    fn short_text_with_padding_different_fill_char() {
        let mut grid: AsciiGrid = AsciiGrid::empty(' ');
        let text: Box<dyn AsciiElement> =
            Box::new(AsciiText::new(Position::default(), "Short".to_string()));
        let mut decorator: AsciiBorderDecorator =
            AsciiBorderDecorator::new(text, Position::default(), 3, 3, '#');

        decorator.draw(&mut grid);

        assert_eq!(
            grid.to_string(),
            AsciiGrid::new(
                ' ',
                vec![
                    vec![
                        '╭', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '╮'
                    ],
                    vec![
                        '│', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '│'
                    ],
                    vec![
                        '│', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '│'
                    ],
                    vec![
                        '│', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '│'
                    ],
                    vec![
                        '│', '#', '#', '#', 'S', 'h', 'o', 'r', 't', '#', '#', '#', '│'
                    ],
                    vec![
                        '│', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '│'
                    ],
                    vec![
                        '│', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '│'
                    ],
                    vec![
                        '│', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '│'
                    ],
                    vec![
                        '╰', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '╯'
                    ],
                ]
            )
            .to_string()
        )
    }
}
