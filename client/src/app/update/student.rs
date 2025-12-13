use crate::api::student::fetch_students;
use crate::app::App;
use crate::app::Message;
use crate::app::Screen;
use iced::Command;

pub fn update(app: &mut App, message: Message) -> Command<Message> {
    match message {
        Message::StudentButtonPressed => {
            if app.loading {
                return Command::none();
            }
            app.loading = true;
            app.status = "Loading students...".into();
            Command::perform(fetch_students(), Message::StudentsLoaded)
        }

        Message::StudentsLoaded(result) => {
            app.loading = false;
            match result {
                Ok(students) => {
                    app.current_screen = Screen::StudentTable {
                        students,
                        fio_input: String::new(),
                        direction_id_input: String::new(),
                        id_input: String::new(),
                    };
                    app.status = "Students loaded".into();
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
