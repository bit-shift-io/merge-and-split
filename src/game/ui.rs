use iced::widget::{column, text};
use iced::{Color, Element, Length, Theme};
use crate::game::GameState;

#[derive(Debug, Clone)]
pub struct GameUI {
    fps: i32,
    total_time: f32,
    game_state: GameState,
    leaderboard_results: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Message {
    UpdateFps(i32),
    UpdateTime(f32),
    UpdateGameState(GameState),
    UpdateLeaderboardResults(String),
}

impl GameUI {
    pub fn new() -> Self {
        Self {
            fps: 60,
            total_time: 0.0,
            game_state: GameState::Playing,
            leaderboard_results: None,
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::UpdateFps(fps) => self.fps = fps,
            Message::UpdateTime(time) => self.total_time = time,
            Message::UpdateGameState(state) => self.game_state = state,
            Message::UpdateLeaderboardResults(results) => self.leaderboard_results = Some(results),
        }
    }

    pub fn view(&self) -> Element<'_, Message, Theme, iced::Renderer> {
        let content = if self.game_state == GameState::Finished {
            column![
                text("FINISH!")
                    .size(50)
                    .color(Color::from_rgb(1.0, 0.84, 0.0)), // Gold
                text(format!("Final Time: {:.2}s", self.total_time))
                    .size(30)
                    .color(Color::WHITE),
                text(self.leaderboard_results.as_deref().unwrap_or("Loading leaderboard..."))
                    .size(20)
                    .color(Color::WHITE),
                text("Press 'r' to retry")
                    .size(20)
                    .color(Color::from_rgb(0.5, 0.5, 1.0)),
            ]
            .spacing(20)
            //.align_items(iced::Alignment::Center)
        } else {
            column![
                text(format!("FPS: {}", self.fps))
                    .size(20)
                    .color(Color::WHITE),
                text(format!("Time: {:.2}s", self.total_time))
                    .size(20)
                    .color(Color::WHITE),
            ]
        };

        content
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
