use crate::api::journal::fetch_journals;
use crate::app::App;
use crate::app::Message;
use crate::app::Screen;
use iced::Command;

pub fn update(app: &mut App, message: Message) -> Command<Message> {
    match message {
        Message::JournalButtonPressed => {
            if app.loading {
                return Command::none();
            }
            app.loading = true;
            app.status = "Loading journal...".into();
            Command::perform(fetch_journals(), Message::JournalLoaded)
        }

        Message::JournalFkCabinetInputChanged(s) => {
            if let Screen::JournalTable { fk_cabinet, .. } = &mut app.current_screen {
                *fk_cabinet = s;
            }
            Command::none()
        }

        Message::JournalRecordNoInputChanged(s) => {
            if let Screen::JournalTable { record_no, .. } = &mut app.current_screen {
                *record_no = s;
            }
            Command::none()
        }

        Message::JournalDateInputChanged(s) => {
            if let Screen::JournalTable { date, .. } = &mut app.current_screen {
                *date = s;
            }
            Command::none()
        }

        Message::JournalLessonInputChanged(s) => {
            if let Screen::JournalTable { lesson, .. } = &mut app.current_screen {
                *lesson = s;
            }
            Command::none()
        }

        Message::JournalFkStudentInputChanged(s) => {
            if let Screen::JournalTable { fk_student, .. } = &mut app.current_screen {
                *fk_student = s;
            }
            Command::none()
        }

        Message::JournalFkLabInputChanged(s) => {
            if let Screen::JournalTable { fk_lab, .. } = &mut app.current_screen {
                *fk_lab = s;
            }
            Command::none()
        }

        Message::JournalDeleteFkCabinetInputChanged(s) => {
            if let Screen::JournalTable {
                delete_fk_cabinet, ..
            } = &mut app.current_screen
            {
                *delete_fk_cabinet = s;
            }
            Command::none()
        }

        Message::JournalDeleteRecordNoInputChanged(s) => {
            if let Screen::JournalTable {
                delete_record_no, ..
            } = &mut app.current_screen
            {
                *delete_record_no = s;
            }
            Command::none()
        }

        Message::JournalLoaded(result) => {
            app.loading = false;
            match result {
                Ok(journals) => {
                    app.current_screen = Screen::JournalTable {
                        journals,
                        fk_cabinet: String::new(),
                        record_no: String::new(),
                        date: String::new(),
                        lesson: String::new(),
                        fk_student: String::new(),
                        fk_lab: String::new(),
                        delete_fk_cabinet: String::new(),
                        delete_record_no: String::new(),
                    };
                    app.status = "Journals loaded".into();
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
