use crate::api::direction::fetch_directions;
use crate::app::App;
use crate::app::Message;
use crate::app::Screen;
use iced::Command;

pub fn update(app: &mut App, message: Message) -> Command<Message> {
    match message {
        Message::DirectionButtonPressed => {
            if app.loading {
                return Command::none();
            }
            app.loading = true;
            app.status = "Loading direction".into();
            Command::perform(fetch_directions(), Message::DirectionsLoaded)
        }
        Message::DirectionNameInputChanged(s) => {
            if let Screen::DirectionTable { name_input, .. } = &mut app.current_screen {
                *name_input = s;
            }
            Command::none()
        }

        Message::DirectionsLoaded(result) => {
            app.loading = false;
            match result {
                Ok(directions) => {
                    app.current_screen = Screen::DirectionTable {
                        directions,
                        name_input: String::new(),
                        id_input: String::new(),
                    };
                    app.status = "Direction loaded".into();
                }
                Err(e) => {
                    app.status = format!("Error: {}", e);
                }
            }
            Command::none()
        }

        Message::DirectionIdInputChanged(s) => {
            if let Screen::StudentTable {
                direction_id_input, ..
            } = &mut app.current_screen
            {
                *direction_id_input = s;
            }
            Command::none()
        }

        _ => Command::none(),
    }
}
