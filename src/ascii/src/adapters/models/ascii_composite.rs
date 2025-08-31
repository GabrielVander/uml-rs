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
    fn draw(&self, grid: &mut AsciiGrid) {
        self.components.iter().for_each(|c| c.draw(grid));
    }

    fn position(&self, grid: &AsciiGrid) -> Position {
        self.components
            .iter()
            .map(|c| c.position(grid))
            .min_by_key(|p| (p.y, p.x))
            .unwrap_or_default()
    }

    fn width(&self) -> i32 {
        self.components.iter().map(|c| c.width()).max().unwrap_or(0)
    }

    fn height(&self) -> i32 {
        self.components
            .iter()
            .map(|c| c.height())
            .max()
            .unwrap_or(0)
    }
}
