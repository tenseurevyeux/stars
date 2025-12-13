use crate::api::cabinet::create_cabinet;
use crate::api::direction::create_direction;
use crate::api::journal::create_journal;
use crate::api::laborant::create_laborant;
use crate::api::student::create_student;

use crate::app::App;
use crate::app::Message;
use crate::app::Screen;
use iced::Command;

pub fn update(app: &mut App, message: Message) -> Command<Message> {
    match message {
        Message::CreatePressed => {
            if app.loading {
                return Command::none();
            }

            match &app.current_screen {
                Screen::DirectionTable { name_input, .. } => {
                    if name_input.trim().is_empty() {
                        app.status = "Name cannot be empty".into();
                        return Command::none();
                    }
                    app.loading = true;
                    let name = name_input.clone();
                    return Command::perform(create_direction(name), Message::OperationFinished);
                }

                Screen::StudentTable {
                    fio_input,
                    direction_id_input,
                    ..
                } => {
                    if fio_input.trim().is_empty() || direction_id_input.trim().is_empty() {
                        app.status = "FIO and Direction ID cannot be empty".into();
                        return Command::none();
                    }

                    let direction_id: i32 = match direction_id_input.trim().parse() {
                        Ok(id) => id,
                        Err(_) => {
                            app.status = "Direction ID must be a number".into();
                            return Command::none();
                        }
                    };

                    app.loading = true;
                    let fio = fio_input.clone();
                    return Command::perform(
                        create_student(fio, direction_id),
                        Message::OperationFinished,
                    );
                }

                Screen::CabinetTable { number_input, .. } => {
                    if number_input.trim().is_empty() {
                        app.status = "Cabinet number cannot be empty".into();
                        return Command::none();
                    }

                    app.loading = true;
                    let number = number_input.clone();
                    return Command::perform(create_cabinet(number), Message::OperationFinished);
                }

                Screen::LaborantTable { fio_input, .. } => {
                    if fio_input.trim().is_empty() {
                        app.status = "FIO cannot be empty".into();
                        return Command::none();
                    }

                    app.loading = true;
                    let fio = fio_input.clone();
                    return Command::perform(create_laborant(fio), Message::OperationFinished);
                }

                Screen::JournalTable {
                    fk_cabinet,
                    record_no,
                    date,
                    lesson,
                    fk_student,
                    fk_lab,
                    ..
                } => {
                    if fk_cabinet.trim().is_empty()
                        || record_no.trim().is_empty()
                        || date.trim().is_empty()
                        || lesson.trim().is_empty()
                        || fk_student.trim().is_empty()
                        || fk_lab.trim().is_empty()
                    {
                        app.status = "All fields must be filled".into();
                        return Command::none();
                    }

                    let fk_cabinet_val: i32 = match fk_cabinet.trim().parse() {
                        Ok(id) => id,
                        Err(_) => {
                            app.status = "Cabinet ID must be a number".into();
                            return Command::none();
                        }
                    };

                    let record_no_val: i32 = match record_no.trim().parse() {
                        Ok(id) => id,
                        Err(_) => {
                            app.status = "Record number must be a number".into();
                            return Command::none();
                        }
                    };

                    let fk_student_val: i32 = match fk_student.trim().parse() {
                        Ok(id) => id,
                        Err(_) => {
                            app.status = "Student ID must be a number".into();
                            return Command::none();
                        }
                    };

                    let fk_lab_val: i32 = match fk_lab.trim().parse() {
                        Ok(id) => id,
                        Err(_) => {
                            app.status = "Laborant ID must be a number".into();
                            return Command::none();
                        }
                    };

                    app.loading = true;
                    let date_clone = date.clone();
                    let lesson_clone = lesson.clone();

                    return Command::perform(
                        create_journal(
                            fk_cabinet_val,
                            record_no_val,
                            date_clone,
                            lesson_clone,
                            fk_student_val,
                            fk_lab_val,
                        ),
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
