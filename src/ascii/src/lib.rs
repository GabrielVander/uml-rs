pub(crate) mod adapters;

#[cfg(test)]
mod test {

    use crate::{
        adapters::{
            presenters::ascii_presenter::AsciiPresenter,
            view_models::ascii_grid_view_model::AsciiGridViewModel,
        },
        ascii_grid_model,
    };

    use pretty_assertions::assert_eq;
    use uml::domain::entities::diagram::{Diagram, Node, NodeType};

    #[test]
    fn empty_diagram() {
        let mut presenter: AsciiPresenter = AsciiPresenter::new();
        let empty_diagram: Diagram = Diagram::new(None, vec![], vec![]);

        let result: AsciiGridViewModel = presenter.process_diagram(&empty_diagram);

        assert_eq!(result.to_string(), AsciiGridViewModel::empty().to_string())
    }

    #[test]
    fn one_component_no_name() {
        let mut presenter: AsciiPresenter = AsciiPresenter::new();
        let diagram: Diagram = Diagram::new(
            None,
            vec![Node::new(
                "my_node".to_string(),
                NodeType::Component("".to_string()),
            )],
            vec![],
        );

        let result: AsciiGridViewModel = presenter.process_diagram(&diagram);

        assert_eq!(
            result.to_string(),
            ascii_grid_model! {
                vec![
                    vec!['╭', '─', '─', '─', '─', '╮'],
                    vec!['│', ' ', ' ', ' ', ' ', '│'],
                    vec!['│', ' ', ' ', ' ', ' ', '│'],
                    vec!['│', ' ', ' ', ' ', ' ', '│'],
                    vec!['│', ' ', ' ', ' ', ' ', '│'],
                    vec!['╰', '─', '─', '─', '─', '╯'],
                ]
            }
            .to_string()
        )
    }

    #[test]
    fn one_component_with_name() {
        let mut presenter: AsciiPresenter = AsciiPresenter::new();
        let diagram: Diagram = Diagram::new(
            None,
            vec![Node::new(
                "my_node".to_string(),
                NodeType::Component("SomeComponent".to_string()),
            )],
            vec![],
        );

        let result: AsciiGridViewModel = presenter.process_diagram(&diagram);

        assert_eq!(
            result.to_string(),
            ascii_grid_model! {
                vec![
                    vec!['╭', '─', '─','─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '╮'],
                    vec!['│', ' ', ' ',' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '│'],
                    vec!['│', ' ', ' ',' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '│'],
                    vec!['│', ' ', ' ','S', 'o', 'm', 'e', 'C', 'o', 'm', 'p', 'o', 'n', 'e', 'n', 't', ' ', ' ', '│'],
                    vec!['│', ' ', ' ',' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '│'],
                    vec!['│', ' ', ' ',' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '│'],
                    vec!['╰', '─', '─','─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '╯'],
                ]
            }
            .to_string()
        )
    }
}
