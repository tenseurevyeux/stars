use crate::api::cabinet::fetch_cabinets;
use crate::api::direction::fetch_directions;
use crate::api::journal::fetch_journals;
use crate::api::laborant::fetch_laborants;
use crate::api::student::fetch_students;
use crate::api::summary::fetch_summary;
use crate::app::App;
use crate::app::Message;
use crate::app::Screen;
use iced::Command;

pub fn update(app: &mut App, message: Message) -> Command<Message> {
    match message {
        Message::UpdateTablePressed => {
            if app.loading {
                return Command::none();
            }

            app.loading = true;
            match &app.current_screen {
                Screen::DirectionTable { .. } => {
                    app.status = "Refreshing directions...".into();
                    Command::perform(fetch_directions(), Message::DirectionsLoaded)
                }

                Screen::StudentTable { .. } => {
                    app.status = "Refreshing students...".into();
                    Command::perform(fetch_students(), Message::StudentsLoaded)
                }

                Screen::CabinetTable { .. } => {
                    app.status = "Refreshing cabinets...".into();
                    Command::perform(fetch_cabinets(), Message::CabinetsLoaded)
                }

                Screen::LaborantTable { .. } => {
                    app.status = "Refreshing laborants...".into();
                    Command::perform(fetch_laborants(), Message::LaborantsLoaded)
                }

                Screen::JournalTable { .. } => {
                    app.status = "Refreshing journals...".into();
                    Command::perform(fetch_journals(), Message::JournalLoaded)
                }

                Screen::SummaryTable { .. } => {
                    app.status = "Refreshing summary...".into();
                    Command::perform(fetch_summary(), Message::SummaryLoaded)
                }

                _ => Command::none(),
            }
        }

        Message::OperationFinished(result) => {
            app.loading = false;
            match result {
                Ok(()) => {
                    app.status = "Operation successful!".into();
                    app.loading = true;
                    match &app.current_screen {
                        Screen::DirectionTable { .. } => {
                            Command::perform(fetch_directions(), Message::DirectionsLoaded)
                        }

                        Screen::StudentTable { .. } => {
                            Command::perform(fetch_students(), Message::StudentsLoaded)
                        }

                        Screen::CabinetTable { .. } => {
                            Command::perform(fetch_cabinets(), Message::CabinetsLoaded)
                        }

                        Screen::LaborantTable { .. } => {
                            Command::perform(fetch_laborants(), Message::LaborantsLoaded)
                        }

                        Screen::JournalTable { .. } => {
                            Command::perform(fetch_journals(), Message::JournalLoaded)
                        }

                        _ => Command::none(),
                    }
                }

                Err(e) => {
                    app.status = format!("Error: {}", e);
                    Command::none()
                }
            }
        }
        _ => Command::none(),
    }
}
