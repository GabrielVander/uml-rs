use iced::widget::{pane_grid, text, text_editor};

pub(crate) struct IcedApp {
    state: IcedAppState,
}

impl IcedApp {
    pub(crate) fn title(&self) -> &str {
        "UML-RS Iced Desktop Application"
    }

    pub(crate) fn view(&self) -> iced::Element<IcedAppMessage> {
        // Implementation of the view goes here
        pane_grid(
            &self.state.panes,
            |_pane, pane_type, _is_maximized| match pane_type {
                UiPanes::TextEditor => self.render_text_editor(),
                UiPanes::DiagramVisualizer => text("Diagram Visualizer Pane").into(),
            },
        )
        .into()
    }

    pub(crate) fn update(&mut self, message: IcedAppMessage) {
        match message {}
    }

    pub fn new() -> Self {
        Self {
            state: IcedAppState {
                panes: pane_grid::State::new(UiPanes::default()).0,
                text_editor_content: text_editor::Content::new(),
            },
        }
    }
    fn render_text_editor(&self) -> pane_grid::Content<IcedAppMessage> {
        text_editor(&self.state.text_editor_content).into()
    }
}

#[derive(Default)]
enum UiPanes {
    #[default]
    TextEditor,
    DiagramVisualizer,
}

pub(crate) struct IcedAppState {
    panes: iced::widget::pane_grid::State<UiPanes>,
    text_editor_content: text_editor::Content,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum IcedAppMessage {}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use crate::infra::iced_app::IcedApp;

    #[test]
    fn test_iced_app_creation() {
        let app = IcedApp::new();

        assert_eq!(app.title(), "UML-RS Iced Desktop Application");
        assert_eq!(app.state.panes.len(), 1); // Default pane
        assert_eq!(app.state.text_editor_content.text(), "".to_owned());
    }
}
