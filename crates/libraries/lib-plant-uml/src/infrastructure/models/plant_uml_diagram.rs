#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlantUmlDiagram {
    pub elements: Vec<PlantUmlElement>,
}

impl PlantUmlDiagram {
    pub fn new(elements: Vec<PlantUmlElement>) -> Self {
        Self { elements }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlantUmlElement {
    // Name, Alias
    Component(String, Option<String>),
}
