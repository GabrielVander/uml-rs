use crate::domain::entities::diagram::Diagram;

pub trait DiagramRepository {
    fn parse_from_content(&self, content: &str) -> Result<Diagram, DiagramRepositoryError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiagramRepositoryError {
    Unknown(String),
}
