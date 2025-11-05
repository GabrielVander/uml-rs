use crate::infrastructure::models::plant_uml_diagram::PlantUmlDiagram;

pub trait PlantUmlParser {
    fn parse(&self, input: &str) -> Result<PlantUmlDiagram, PlantUmlParserError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlantUmlParserError {
    UnknownError,
}
