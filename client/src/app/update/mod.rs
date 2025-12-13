use crate::app::screen::Screen;
use crate::app::{App, message::Message};
use iced::Command;
pub mod cabinet;
pub mod create;
pub mod delete;
pub mod direction;
pub mod journal;
pub mod laborant;
pub mod other;
pub mod reload;
pub mod student;
pub mod summary;

pub fn update(app: &mut App, message: Message) -> Command<Message> {
    match message {
        Message::DirectionButtonPressed
        | Message::DirectionIdInputChanged(_)
        | Message::DirectionsLoaded(_)
        | Message::DirectionNameInputChanged(_) => direction::update(app, message),

        Message::StudentButtonPressed | Message::StudentsLoaded(_) => student::update(app, message),

        Message::LaborantButtonPressed | Message::LaborantsLoaded(_) => {
            laborant::update(app, message)
        }

        Message::CabinetButtonPressed
        | Message::CabinetNumberInputChanged(_)
        | Message::CabinetsLoaded(_) => cabinet::update(app, message),

        Message::JournalButtonPressed
        | Message::JournalDateInputChanged(_)
        | Message::JournalDeleteFkCabinetInputChanged(_)
        | Message::JournalDeleteRecordNoInputChanged(_)
        | Message::JournalFkCabinetInputChanged(_)
        | Message::JournalFkLabInputChanged(_)
        | Message::JournalFkStudentInputChanged(_)
        | Message::JournalLessonInputChanged(_)
        | Message::JournalRecordNoInputChanged(_)
        | Message::JournalLoaded(_) => journal::update(app, message),

        Message::SummaryButtonPressed | Message::SummaryLoaded(_) => summary::update(app, message),

        Message::CreatePressed => create::update(app, message),
        Message::IdInputChanged(_) | Message::FioInputChanged(_) => other::update(app, message),

        Message::DeletePressed => delete::update(app, message),
        Message::UpdateTablePressed | Message::OperationFinished(_) => reload::update(app, message),

        Message::BackPressed => {
            app.current_screen = Screen::MainMenu;
            app.status = String::new();
            Command::none()
        }
    }
}
