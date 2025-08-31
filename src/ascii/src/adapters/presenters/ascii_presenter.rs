use uml::domain::entities::diagram::{Diagram, Node};

use crate::adapters::{
    models::{
        ascii_composite::AsciiComposite, ascii_element::AsciiElement, ascii_grid::AsciiGrid,
        ascii_text::AsciiText, position::Position,
    },
    view_models::ascii_grid_view_model::AsciiGridViewModel,
};

pub struct AsciiPresenter {
    grid: AsciiGrid,
}

impl AsciiPresenter {
    pub fn new() -> Self {
        Self {
            grid: AsciiGrid::empty(' '),
        }
    }

    pub fn process_diagram(&mut self, diagram: &Diagram) -> AsciiGridViewModel {
        Box::<dyn AsciiElement>::from(diagram).draw(&mut self.grid);

        AsciiGridViewModel::new(self.grid.get_grid())
    }
}

impl From<&Diagram> for Box<dyn AsciiElement> {
    fn from(value: &Diagram) -> Self {
        Box::new(AsciiComposite::new(
            value
                .nodes
                .iter()
                .map(Box::<dyn AsciiElement>::from)
                .collect(),
        ))
    }
}

impl From<&Node> for Box<dyn AsciiElement> {
    fn from(value: &Node) -> Self {
        Box::new(AsciiText::new(
            Position::default(),
            format!("{:?}", value.r#type),
        ))
    }
}
