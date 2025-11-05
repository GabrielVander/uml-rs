use crate::adapters::models::{
    ascii_element::AsciiElement, ascii_grid::AsciiGrid, position::Position,
};

pub struct AsciiComposite {
    components: Vec<Box<dyn AsciiElement>>,
}

impl AsciiComposite {
    pub fn new(components: Vec<Box<dyn AsciiElement>>) -> Self {
        Self { components }
    }

    pub fn empty() -> Self {
        Self {
            components: Vec::new(),
        }
    }

    pub fn add(&mut self, component: Box<dyn AsciiElement>) {
        self.components.push(component);
    }
}

impl AsciiElement for AsciiComposite {
    fn draw(&mut self, grid: &mut AsciiGrid) {
        self.components.iter_mut().for_each(|c| c.draw(grid));
    }

    fn position(&self) -> Position {
        self.components
            .iter()
            .map(|c| c.position())
            .min_by_key(|p| (p.y, p.x))
            .unwrap_or_default()
    }

    fn width(&self) -> u16 {
        self.components.iter().map(|c| c.width()).max().unwrap_or(0)
    }

    fn height(&self) -> u16 {
        self.components
            .iter()
            .map(|c| c.height())
            .max()
            .unwrap_or(0)
    }

    fn r#move(&mut self, new_position: Position) {
        let current_position: Position = self.position();

        let delta_x: i32 = new_position.x - current_position.x;
        let delta_y: i32 = new_position.y - current_position.y;

        self.components.iter_mut().for_each(|c| {
            c.r#move(Position::new(
                c.position().x + delta_x,
                c.position().y + delta_y,
            ))
        });
    }
}
