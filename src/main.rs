#![windows_subsystem = "windows"]

use iced::widget::text;
use iced::{Element, Sandbox, Settings};

use crate::messages::BEditorMessage;
use crate::nbt_view::NbtView;
use crate::state::BEditorState;
use crate::view::BEditorView;

mod messages;
mod nbt_view;
pub mod state;
mod view;

pub fn main() -> iced::Result {
    App::run(Settings::default())
}

struct App {
    state: BEditorState,
}

impl Sandbox for App {
    type Message = BEditorMessage;

    fn new() -> Self {
        Self {
            state: BEditorState::NbtView(NbtView::new()),
        }
    }

    fn title(&self) -> String {
        match self.state {
            BEditorState::Idle => String::from("BEditor - Idle"),
            BEditorState::NbtView(_) => String::from("BEditor - Nbt"),
        }
    }

    fn update(&mut self, message: Self::Message) {
        match &mut self.state {
            BEditorState::Idle => {}
            BEditorState::NbtView(v) => v.update(message),
        }
    }

    fn view(&self) -> Element<Self::Message> {
        match &self.state {
            BEditorState::Idle => text("Idle").into(),
            BEditorState::NbtView(v) => v.view(),
        }
    }
}
