use iced::{Element, Theme};
use crate::game::GameState;
use crate::game::leaderboard::LeaderboardEntry;

mod hud;
mod leaderboard;

use hud::hud_view;
use leaderboard::leaderboard_view;

#[derive(Debug, Clone)]
pub struct GameUI {
    pub(crate) fps: i32,
    pub(crate) total_time: f32,
    pub(crate) game_state: GameState,
    pub(crate) leaderboard_results: Vec<LeaderboardEntry>,
}

#[derive(Debug, Clone)]
pub enum Message {
    UpdateFps(i32),
    UpdateTime(f32),
    UpdateGameState(GameState),
    UpdateLeaderboardResults(Vec<LeaderboardEntry>),
}

impl GameUI {
    pub fn new() -> Self {
        Self {
            fps: 60,
            total_time: 0.0,
            game_state: GameState::Playing,
            leaderboard_results: Vec::new(),
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::UpdateFps(fps) => self.fps = fps,
            Message::UpdateTime(time) => self.total_time = time,
            Message::UpdateGameState(state) => self.game_state = state,
            Message::UpdateLeaderboardResults(results) => self.leaderboard_results = results,
        }
    }

    pub fn view(&self) -> Element<'_, Message, Theme, iced::Renderer> {
        if self.game_state == GameState::Finished {
            leaderboard_view(self)
        } else {
            hud_view(self)
        }
    }
}
