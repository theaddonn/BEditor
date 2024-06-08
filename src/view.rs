use iced::Element;

use crate::messages::BEditorMessage;

pub trait BEditorView {
    fn new() -> Self;

    fn update(&mut self, message: BEditorMessage);

    fn view(&self) -> Element<BEditorMessage>;
}
