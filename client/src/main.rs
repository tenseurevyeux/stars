mod api;
mod app;
use app::App;
use iced::{Application, Settings};

pub fn main() -> iced::Result {
    App::run(Settings::default())
}
