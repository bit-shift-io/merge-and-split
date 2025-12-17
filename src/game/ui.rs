use iced::widget::{column, text};
use iced::{Color, Element, Length, Theme};

#[derive(Debug, Clone)]
pub struct GameUI {
    fps: i32,
}

#[derive(Debug, Clone)]
pub enum Message {
    UpdateFps(i32),
}

impl GameUI {
    pub fn new() -> Self {
        Self { fps: 60 }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::UpdateFps(fps) => self.fps = fps,
        }
    }

    pub fn view(&self) -> Element<Message, Theme, iced::Renderer> {
        column![
            text(format!("FPS: {}", self.fps))
                .size(20)
                .color(Color::WHITE),
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
