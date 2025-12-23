use iced::widget::{column, text, row, container};
use iced::{Color, Element, Length, Theme, Alignment};
use crate::game::GameState;
use crate::game::leaderboard::LeaderboardEntry;

#[derive(Debug, Clone)]
pub struct GameUI {
    fps: i32,
    total_time: f32,
    game_state: GameState,
    leaderboard_results: Vec<LeaderboardEntry>,
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
        let content = if self.game_state == GameState::Finished {
            let header_text_col = Color::from_rgb(0.6, 0.6, 1.0);
            let mut leaderboard_col = column![
                row![
                    text("Pos").width(Length::Fixed(50.0)).color(header_text_col),
                    text("Player").width(Length::Fill).color(header_text_col),
                    text("Time").width(Length::Fixed(100.0)).color(header_text_col),
                ]
                .spacing(10)
                .padding(5)
            ]
            .spacing(5);

            if self.leaderboard_results.is_empty() {
                leaderboard_col = leaderboard_col.push(text("Loading leaderboard...").color(Color::from_rgb(0.7, 0.7, 0.7)));
            } else {
                for entry in &self.leaderboard_results {
                    let color = if entry.is_current_run {
                        Color::from_rgb(0.0, 1.0, 0.0) // Green for current run
                    } else {
                        Color::WHITE
                    };

                    leaderboard_col = leaderboard_col.push(
                        row![
                            text(format!("{}.", entry.rank)).width(Length::Fixed(50.0)).color(color),
                            text(&entry.name).width(Length::Fill).color(color),
                            text(format!("{:.3}s", entry.time)).width(Length::Fixed(100.0)).color(color),
                        ]
                        .spacing(10)
                        .padding(2)
                    );
                }
            }

            container(
                column![
                    text(format!("Final Time: {:.2}s", self.total_time))
                        .size(40)
                        .color(Color::WHITE),
                    container(leaderboard_col)
                        .width(Length::Fixed(400.0))
                        .padding(20)
                        .style(|_theme: &Theme| {
                            container::Style {
                                background: Some(iced::Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.5))),
                                border: iced::Border {
                                    radius: 10.0.into(),
                                    width: 1.0,
                                    color: Color::from_rgb(0.4, 0.4, 0.4),
                                },
                                ..Default::default()
                            }
                        }),
                    text("Press 'r' to retry")
                        .size(22)
                        .color(Color::from_rgb(0.6, 0.6, 1.0)),
                ]
                .spacing(30)
                .align_x(Alignment::Center)
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(|_theme: &Theme| {
                container::Style {
                    background: Some(iced::Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.8))),
                    ..Default::default()
                }
            })
        } else {
            container(
                column![
                    text(format!("FPS: {}", self.fps))
                        .size(20)
                        .color(Color::WHITE),
                    text(format!("Time: {:.2}s", self.total_time))
                        .size(20)
                        .color(Color::WHITE),
                ]
                .padding(10)
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Start)
            .align_y(Alignment::Start)
        };

        content.into()
    }
}
