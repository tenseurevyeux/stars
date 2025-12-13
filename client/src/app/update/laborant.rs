use crate::api::laborant::fetch_laborants;
use crate::app::App;
use crate::app::Message;
use crate::app::Screen;
use iced::Command;

pub fn update(app: &mut App, message: Message) -> Command<Message> {
    match message {
        Message::LaborantButtonPressed => {
            if app.loading {
                return Command::none();
            }
            app.loading = true;
            app.status = "Loading laborants...".into();
            Command::perform(fetch_laborants(), Message::LaborantsLoaded)
        }
        Message::LaborantsLoaded(result) => {
            app.loading = false;
            match result {
                Ok(laborants) => {
                    app.current_screen = Screen::LaborantTable {
                        laborants,
                        fio_input: String::new(),
                        id_input: String::new(),
                    };
                    app.status = "Laborants loaded".into();
                }
                Err(e) => {
                    app.status = format!("Error: {}", e);
                }
            }
            Command::none()
        }
        _ => Command::none(),
    }
}
