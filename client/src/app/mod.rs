pub mod message;
pub mod screen;
pub mod update;
pub mod view;
use iced::{Application, Command, Theme, executor};
use message::Message;
use screen::Screen;
use update::update;
use view::view;

pub struct App {
    current_screen: Screen,
    status: String,
    loading: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            current_screen: Screen::MainMenu,
            status: String::new(),
            loading: false,
        }
    }
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn title(&self) -> String {
        "University Management System".into()
    }

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Self::default(), Command::none())
    }
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        update(self, message)
    }
    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        view(&self)
    }
}
