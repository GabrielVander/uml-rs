use pest::Parser;
use pest_derive::Parser;

use crate::infrastructure::{
    models::plant_uml_diagram::{PlantUmlDiagram, PlantUmlElement},
    plant_uml_parser::{PlantUmlParser, PlantUmlParserError},
};

#[derive(Parser)]
#[grammar = "src/infrastructure/pest/plant_uml_grammar.pest"]
struct PestParser {}

pub struct PestPlantUmlParser {}

impl PestPlantUmlParser {
    pub fn new() -> Self {
        Self {}
    }

    fn parse_with_pest_parser<'a>(
        &self,
        input: &'a str,
    ) -> Result<pest::iterators::Pairs<'a, Rule>, PlantUmlParserError> {
        PestParser::parse(Rule::diagram, input)
            .inspect(|p| println!("{:?}", p))
            .inspect_err(|e| println!("{:?}", e))
            .map_err(|_| PlantUmlParserError::UnknownError)
    }

    fn create_component_from_pair(&self, pair: pest::iterators::Pair<Rule>) -> PlantUmlElement {
        let mut name: String = "".to_string();
        let mut alias: Option<String> = None;

        for component_pair in pair.into_inner() {
            match component_pair.as_rule() {
                Rule::identifier => {
                    name = component_pair.as_str().to_string();
                }
                Rule::alias => {
                    alias = Some(component_pair.into_inner().as_str().to_string());
                }
                _ => {}
            }
        }

        PlantUmlElement::Component(name, alias)
    }
}

impl PlantUmlParser for PestPlantUmlParser {
    fn parse(&self, input: &str) -> Result<PlantUmlDiagram, PlantUmlParserError> {
        let mut diagram: PlantUmlDiagram = PlantUmlDiagram::new(vec![]);

        for pair in self.parse_with_pest_parser(input)? {
            match pair.as_rule() {
                Rule::component_declaration => {
                    diagram.elements.push(self.create_component_from_pair(pair));
                }
                _ => {}
            }
        }

        Ok(diagram)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    macro_rules! parse_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (first_param, second_param) = $value;

                let input: &str = first_param;
                let expected: Result<PlantUmlDiagram, PlantUmlParserError> = second_param;

                let parser: Box<dyn PlantUmlParser> = Box::new(PestPlantUmlParser::new());
                let result: Result<PlantUmlDiagram, PlantUmlParserError> = parser.parse(input);

                assert_eq!(result, expected);
            }
        )*
        }
    }

    parse_tests! {
        empty_input: ("", Err(PlantUmlParserError::UnknownError)),
        empty_diagram: ("@startuml@enduml", Ok(PlantUmlDiagram::new(vec![]))),
        empty_diagram_with_line_breaks: ("@startuml\n\n\n\n\n@enduml", Ok(PlantUmlDiagram::new(vec![]))),
        one_component: ("@startuml\ncomponent MyComponent\n@enduml", Ok(PlantUmlDiagram::new(vec![PlantUmlElement::Component("MyComponent".to_string(), None)]))),
        multiple_components: (
            "@startuml\ncomponent MyComponent\ncomponent MyOtherComponent\n\n\n\n\ncomponent YetAnotherComponent\n@enduml",
            Ok(PlantUmlDiagram::new(vec![
                PlantUmlElement::Component("MyComponent".to_string(), None),
                PlantUmlElement::Component("MyOtherComponent".to_string(), None),
                PlantUmlElement::Component("YetAnotherComponent".to_string(), None)
            ]))
        ),
        component_with_alias: (
            "@startuml\ncomponent MyComponent as some_alias\n@enduml",
            Ok(PlantUmlDiagram::new(vec![
                PlantUmlElement::Component("MyComponent".to_string(), Some("some_alias".to_string()))
            ]))
        ),
    }
}
