use crate::app::App;
use crate::app::Message;
use crate::app::Screen;
use iced::Command;

pub fn update(app: &mut App, message: Message) -> Command<Message> {
    match message {
        Message::IdInputChanged(s) => {
            match &mut app.current_screen {
                Screen::DirectionTable { id_input, .. } => *id_input = s,
                Screen::StudentTable { id_input, .. } => *id_input = s,
                Screen::CabinetTable { id_input, .. } => *id_input = s,
                Screen::LaborantTable { id_input, .. } => *id_input = s,
                _ => {}
            }
            Command::none()
        }

        Message::FioInputChanged(s) => {
            match &mut app.current_screen {
                Screen::StudentTable { fio_input, .. } => *fio_input = s,
                Screen::LaborantTable { fio_input, .. } => *fio_input = s,
                _ => {}
            }
            Command::none()
        }
        _ => Command::none(),
    }
}
