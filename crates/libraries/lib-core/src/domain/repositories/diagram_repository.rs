use crate::domain::entities::diagram::Diagram;

pub(crate) trait DiagramRepository {
    fn parse_from_content(&self, content: &str) -> Result<Diagram, DiagramRepositoryError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum DiagramRepositoryError {
    Unknown(String),
}
