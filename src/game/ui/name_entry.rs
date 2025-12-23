use iced::widget::{column, text, text_input, button, container};
use iced::{Color, Element, Length, Theme, Alignment};
use super::game_ui::{Message, GameUI};

pub fn name_entry_view(ui: &GameUI) -> Element<'_, Message, Theme, iced::Renderer> {
    let input = text_input("Enter your name...", &ui.name_input)
        .on_input(Message::UpdateNameInput)
        .on_submit(Message::SubmitName)
        .padding(15)
        .size(30)
        .width(Length::Fixed(400.0));

    let submit_button = button(text("Start Game").size(24))
        .padding(10)
        .on_press(Message::SubmitName);

    container(
        column![
            text("Welcome to Planck Time Trials")
                .size(50)
                .color(Color::WHITE),
            text("Please enter your name to continue")
                .size(24)
                .color(Color::from_rgb(0.7, 0.7, 0.7)),
            column![
                input,
                submit_button,
            ]
            .spacing(20)
            .align_x(Alignment::Center),
        ]
        .spacing(40)
        .align_x(Alignment::Center)
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .style(|_theme: &Theme| {
        container::Style {
            background: Some(iced::Background::Color(Color::from_rgb(0.05, 0.05, 0.1))),
            ..Default::default()
        }
    })
    .into()
}
