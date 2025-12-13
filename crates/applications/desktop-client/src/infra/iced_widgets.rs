use iced::widget::{Column, TextInput};

use crate::Message;

struct TextEditor {}

impl TextEditor {
    pub fn new() -> Self {
        Self {}
    }

    fn view(&mut self) -> iced::Element<'_, Message> {
        let input = TextInput::new("Type here...", "").padding(10).size(20);

        Column::new().push(input).into()
    }
}
