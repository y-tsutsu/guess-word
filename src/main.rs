use iced::{Application, Settings};
mod gui;

fn main() -> iced::Result {
    let mut settings = Settings::default();
    settings.window.size = (400, 200);
    gui::GuessWord::run(settings)
}
