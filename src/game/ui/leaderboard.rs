use iced::widget::{column, text, row, container};
use iced::{Color, Element, Length, Theme, Alignment};
use super::game_ui::{Message, GameUI};

pub fn leaderboard_view(ui: &GameUI) -> Element<'_, Message, Theme, iced::Renderer> {
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

    if ui.leaderboard_results.is_empty() {
        leaderboard_col = leaderboard_col.push(text("Loading leaderboard...").color(Color::from_rgb(0.7, 0.7, 0.7)));
    } else {
        for entry in &ui.leaderboard_results {
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
            text(format!("Final Time: {:.2}s", ui.total_time))
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
    .into()
}
