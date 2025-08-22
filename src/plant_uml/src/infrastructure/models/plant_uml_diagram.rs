#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlantUmlDiagram {
    pub elements: Vec<PlantUmlElement>,
}

impl PlantUmlDiagram {
    pub fn new(components: Vec<PlantUmlElement>) -> Self {
        Self {
            elements: components,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlantUmlElement {
    Component(String),
}
