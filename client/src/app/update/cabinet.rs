use crate::api::cabinet::fetch_cabinets;
use crate::app::App;
use crate::app::Message;
use crate::app::Screen;
use iced::Command;

pub fn update(app: &mut App, message: Message) -> Command<Message> {
    match message {
        Message::CabinetButtonPressed => {
            if app.loading {
                return Command::none();
            }
            app.loading = true;
            app.status = "Loading cabinets...".into();
            Command::perform(fetch_cabinets(), Message::CabinetsLoaded)
        }
        Message::CabinetsLoaded(result) => {
            app.loading = false;
            match result {
                Ok(cabinets) => {
                    app.current_screen = Screen::CabinetTable {
                        cabinets,
                        number_input: String::new(),
                        id_input: String::new(),
                    };
                    app.status = "Cabinets loaded".into();
                }
                Err(e) => {
                    app.status = format!("Error: {}", e);
                }
            }
            Command::none()
        }
        Message::CabinetNumberInputChanged(s) => {
            if let Screen::CabinetTable { number_input, .. } = &mut app.current_screen {
                *number_input = s;
            }
            Command::none()
        }
        _ => Command::none(),
    }
}
