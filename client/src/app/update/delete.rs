use crate::api::cabinet::delete_cabinet;
use crate::api::direction::delete_direction;
use crate::api::journal::delete_journal;
use crate::api::laborant::delete_laborant;
use crate::api::student::delete_student;
use crate::app::App;
use crate::app::Message;
use crate::app::Screen;
use iced::Command;

pub fn update(app: &mut App, message: Message) -> Command<Message> {
    match message {
        Message::DeletePressed => {
            if app.loading {
                return Command::none();
            }

            match &app.current_screen {
                Screen::DirectionTable { id_input, .. } => {
                    let id: i32 = match id_input.trim().parse() {
                        Ok(id) => id,
                        Err(_) => {
                            app.status = "ID must be a number".into();
                            return Command::none();
                        }
                    };

                    app.loading = true;
                    return Command::perform(delete_direction(id), Message::OperationFinished);
                }

                Screen::StudentTable { id_input, .. } => {
                    let id: i32 = match id_input.trim().parse() {
                        Ok(id) => id,
                        Err(_) => {
                            app.status = "ID must be a number".into();
                            return Command::none();
                        }
                    };

                    app.loading = true;
                    return Command::perform(delete_student(id), Message::OperationFinished);
                }

                Screen::CabinetTable { id_input, .. } => {
                    let id: i32 = match id_input.trim().parse() {
                        Ok(id) => id,
                        Err(_) => {
                            app.status = "ID must be a number".into();
                            return Command::none();
                        }
                    };

                    app.loading = true;
                    return Command::perform(delete_cabinet(id), Message::OperationFinished);
                }

                Screen::LaborantTable { id_input, .. } => {
                    let id: i32 = match id_input.trim().parse() {
                        Ok(id) => id,
                        Err(_) => {
                            app.status = "ID must be a number".into();
                            return Command::none();
                        }
                    };

                    app.loading = true;
                    return Command::perform(delete_laborant(id), Message::OperationFinished);
                }

                Screen::JournalTable {
                    delete_fk_cabinet,
                    delete_record_no,
                    ..
                } => {
                    if delete_fk_cabinet.trim().is_empty() || delete_record_no.trim().is_empty() {
                        app.status = "Cabinet ID and Record number must be filled".into();
                        return Command::none();
                    }

                    let fk_cabinet_val: i32 = match delete_fk_cabinet.trim().parse() {
                        Ok(id) => id,
                        Err(_) => {
                            app.status = "Cabinet ID must be a number".into();
                            return Command::none();
                        }
                    };

                    let record_no_val: i32 = match delete_record_no.trim().parse() {
                        Ok(id) => id,
                        Err(_) => {
                            app.status = "Record number must be a number".into();
                            return Command::none();
                        }
                    };

                    app.loading = true;
                    return Command::perform(
                        delete_journal(fk_cabinet_val, record_no_val),
                        Message::OperationFinished,
                    );
                }

                _ => {}
            }

            Command::none()
        }
        _ => Command::none(),
    }
}
