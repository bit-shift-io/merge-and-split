pub mod level;
pub mod entity;
pub mod event;
pub mod introduction;
pub mod game;
pub mod irc;
pub mod leaderboard;
pub mod ui;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Playing,
    Finished,
}