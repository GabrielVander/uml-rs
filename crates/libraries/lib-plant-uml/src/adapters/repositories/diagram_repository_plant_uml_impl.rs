use std::sync::Arc;

use lib_core::domain::{
    entities::diagram::{Diagram, Node, NodeType},
    repositories::diagram_repository::{DiagramRepository, DiagramRepositoryError},
};

use crate::infra::models::plant_uml_diagram::{PlantUmlDiagram, PlantUmlElement};

struct DiagramRepositoryPlantUmlImpl {
    parser: Arc<dyn PlantUmlParser>,
}

impl DiagramRepositoryPlantUmlImpl {
    fn new(parser: Arc<dyn PlantUmlParser>) -> Self {
        Self { parser }
    }

    fn parse_plant_uml_script(&self, content: &str) -> Result<Diagram, DiagramRepositoryError> {
        self.parser
            .parse(content)
            .map_err(DiagramRepositoryError::from)
            .map(Diagram::from)
    }
}

pub(crate) trait PlantUmlParser {
    fn parse(&self, input: &str) -> Result<PlantUmlDiagram, PlantUmlParserError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum PlantUmlParserError {
    UnknownError(String),
}

impl DiagramRepository for DiagramRepositoryPlantUmlImpl {
    fn parse_from_content(&self, content: &str) -> Result<Diagram, DiagramRepositoryError> {
        self.parse_plant_uml_script(content)
    }
}

impl From<PlantUmlParserError> for DiagramRepositoryError {
    fn from(value: PlantUmlParserError) -> Self {
        match value {
            PlantUmlParserError::UnknownError(msg) => DiagramRepositoryError::Unknown(msg),
        }
    }
}

impl From<PlantUmlDiagram> for Diagram {
    fn from(value: PlantUmlDiagram) -> Self {
        Self {
            title: None,
            nodes: value.elements.iter().map(Node::from).collect(),
            edges: Vec::new(),
        }
    }
}

impl From<&PlantUmlElement> for Node {
    fn from(value: &PlantUmlElement) -> Self {
        match value {
            PlantUmlElement::Component(name, alias) => Node::new(
                alias.clone().unwrap_or(name.clone()),
                NodeType::Component(name.clone()),
            ),
        }
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use crate::{
        adapters::repositories::diagram_repository_plant_uml_impl::{
            DiagramRepositoryPlantUmlImpl, PlantUmlParser, PlantUmlParserError,
        },
        infra::models::plant_uml_diagram::{PlantUmlDiagram, PlantUmlElement},
    };
    use lib_core::domain::{
        entities::diagram::{Diagram, Node, NodeType},
        repositories::diagram_repository::{DiagramRepository, DiagramRepositoryError},
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn parse_from_content_should_return_expected_error_given_parser_error() {
        let test_cases: Vec<(PlantUmlParserError, DiagramRepositoryError)> = vec![(
            PlantUmlParserError::UnknownError("Parser unknown error".to_string()),
            DiagramRepositoryError::Unknown("Parser unknown error".to_string()),
        )];

        for (parser_error, expected_repository_error) in test_cases {
            let parser_result: Result<PlantUmlDiagram, PlantUmlParserError> = Err(parser_error);

            let parser: Arc<dyn PlantUmlParser> =
                Arc::new(PlantUmlParserMockImpl::new(parser_result));
            let repository: DiagramRepositoryPlantUmlImpl =
                DiagramRepositoryPlantUmlImpl::new(parser);

            let result = repository.parse_from_content("Some dummy content");

            assert_eq!(Err(expected_repository_error), result)
        }
    }

    #[test]
    fn parse_from_content_should_return_expected_diagram_given_parsed_plant_uml_diagram() {
        let test_cases: Vec<(PlantUmlDiagram, Diagram)> = vec![
            (
                PlantUmlDiagram::new(Vec::new()),
                Diagram::new(None, Vec::new(), Vec::new()),
            ),
            (
                PlantUmlDiagram::new(vec![PlantUmlElement::Component(
                    "Some Component".to_owned(),
                    None,
                )]),
                Diagram::new(
                    None,
                    vec![Node::new(
                        "Some Component".to_owned(),
                        NodeType::Component("Some Component".to_owned()),
                    )],
                    vec![],
                ),
            ),
            (
                PlantUmlDiagram::new(vec![
                    PlantUmlElement::Component("Component A".to_owned(), Some("A".to_owned())),
                    PlantUmlElement::Component("Component B".to_owned(), None),
                ]),
                Diagram::new(
                    None,
                    vec![
                        Node::new(
                            "A".to_owned(),
                            NodeType::Component("Component A".to_owned()),
                        ),
                        Node::new(
                            "Component B".to_owned(),
                            NodeType::Component("Component B".to_owned()),
                        ),
                    ],
                    vec![],
                ),
            ),
        ];

        for (plant_uml_diagram, expected_diagram) in test_cases {
            let parser_result: Result<PlantUmlDiagram, PlantUmlParserError> = Ok(plant_uml_diagram);

            let parser: Arc<dyn PlantUmlParser> =
                Arc::new(PlantUmlParserMockImpl::new(parser_result));
            let repository: DiagramRepositoryPlantUmlImpl =
                DiagramRepositoryPlantUmlImpl::new(parser);

            let result = repository.parse_from_content("Some dummy content");

            assert_eq!(Ok(expected_diagram), result)
        }
    }

    struct PlantUmlParserMockImpl {
        result: Result<PlantUmlDiagram, PlantUmlParserError>,
    }

    impl PlantUmlParserMockImpl {
        fn new(result: Result<PlantUmlDiagram, PlantUmlParserError>) -> Self {
            Self { result }
        }
    }

    impl PlantUmlParser for PlantUmlParserMockImpl {
        fn parse(&self, _input: &str) -> Result<PlantUmlDiagram, PlantUmlParserError> {
            self.result.clone()
        }
    }
}
