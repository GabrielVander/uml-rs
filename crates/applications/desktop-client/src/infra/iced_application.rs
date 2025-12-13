#[derive(Debug, Default, PartialEq)]
pub(crate) struct UmlRsIcedDesktopApplication {
    state: UmlRsIcedDesktopApplicationState,
}

impl UmlRsIcedDesktopApplication {
    pub(crate) fn title(&self) -> &str {
        "UML-RS Iced Desktop Application"
    }

    pub(crate) fn view(&self) -> iced::Element<UmlRsIcedDesktopApplicationMessage> {
        // Implementation of the view goes here
        iced::widget::container(iced::widget::text("")).into()
    }

    pub(crate) fn update(&mut self, message: UmlRsIcedDesktopApplicationMessage) {
        match message {
            UmlRsIcedDesktopApplicationMessage::LoadFile => {
                self.state.show_file_picker = true;
            }
        }
    }
}

#[derive(Debug, Default, PartialEq)]
struct UmlRsIcedDesktopApplicationState {
    show_file_picker: bool,
}

#[derive(Debug, PartialEq)]
pub(crate) enum UmlRsIcedDesktopApplicationMessage {
    LoadFile,
}

#[cfg(test)]
mod test {
    use crate::infra::iced_application::{
        UmlRsIcedDesktopApplication, UmlRsIcedDesktopApplicationMessage,
        UmlRsIcedDesktopApplicationState,
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn initial_state_should_be_default() {
        let app: UmlRsIcedDesktopApplication = UmlRsIcedDesktopApplication::default();

        assert_eq!(app.state, UmlRsIcedDesktopApplicationState::default())
    }

    #[test]
    fn on_load_file_should_update_state() {
        let mut app: UmlRsIcedDesktopApplication = UmlRsIcedDesktopApplication::default();

        app.update(UmlRsIcedDesktopApplicationMessage::LoadFile);

        assert_eq!(
            app.state,
            UmlRsIcedDesktopApplicationState {
                show_file_picker: true
            }
        )
    }
}
