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
}

impl PlantUmlParser for PestPlantUmlParser {
    fn parse(&self, input: &str) -> Result<PlantUmlDiagram, PlantUmlParserError> {
        let mut elements: Vec<PlantUmlElement> = vec![];

        for pair in PestParser::parse(Rule::diagram, input)
            .map_err(|_| PlantUmlParserError::UnknownError)?
        {
            match pair.as_rule() {
                Rule::component_declaration => {
                    for component_pair in pair.into_inner() {
                        match component_pair.as_rule() {
                            Rule::identifier => {
                                elements.push(PlantUmlElement::Component(
                                    component_pair.as_str().to_string(),
                                ));
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(PlantUmlDiagram::new(elements))
    }
}

#[cfg(test)]
mod tests {
    use crate::infrastructure::plant_uml_parser::PlantUmlParser;

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
        one_component: ("@startuml\ncomponent MyComponent\n@enduml", Ok(PlantUmlDiagram::new(vec![PlantUmlElement::Component("MyComponent".to_string())]))),
        multiple_components: ("@startuml\ncomponent MyComponent\ncomponent MyOtherComponent\n\n\n\n\ncomponent YetAnotherComponent\n@enduml", Ok(PlantUmlDiagram::new(vec![PlantUmlElement::Component("MyComponent".to_string()), PlantUmlElement::Component("MyOtherComponent".to_string()), PlantUmlElement::Component("YetAnotherComponent".to_string())]))),
    }
}
