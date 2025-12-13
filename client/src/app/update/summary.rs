use crate::api::summary::fetch_summary;
use crate::app::App;
use crate::app::Message;
use crate::app::Screen;
use iced::Command;

pub fn update(app: &mut App, message: Message) -> Command<Message> {
    match message {
        Message::SummaryButtonPressed => {
            if app.loading {
                return Command::none();
            }
            app.loading = true;
            app.status = "Loading summary...".into();
            Command::perform(fetch_summary(), Message::SummaryLoaded)
        }
        Message::SummaryLoaded(result) => {
            app.loading = false;
            match result {
                Ok(summaries) => {
                    app.current_screen = Screen::SummaryTable { summaries };
                    app.status = "Summary loaded".into();
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
