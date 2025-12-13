use crate::app::App;
use crate::app::Screen;
use crate::app::message::Message;
use iced::Element;

pub fn view(app: &App) -> Element<'_, Message> {
    match &app.current_screen {
        Screen::MainMenu => app.view_main_menu(),
        Screen::DirectionTable {
            directions,
            name_input,
            id_input,
        } => app.view_direction_table(directions, name_input, id_input),
        Screen::StudentTable {
            students,
            fio_input,
            direction_id_input,
            id_input,
        } => app.view_student_table(students, fio_input, direction_id_input, id_input),
        Screen::CabinetTable {
            cabinets,
            number_input,
            id_input,
        } => app.view_cabinet_table(cabinets, number_input, id_input),
        Screen::LaborantTable {
            laborants,
            fio_input,
            id_input,
        } => app.view_laborant_table(laborants, fio_input, id_input),
        Screen::JournalTable {
            journals,
            fk_cabinet,
            record_no,
            date,
            lesson,
            fk_student,
            fk_lab,
            delete_fk_cabinet,
            delete_record_no,
        } => app.view_journal_table(
            journals,
            fk_cabinet,
            record_no,
            date,
            lesson,
            fk_student,
            fk_lab,
            delete_fk_cabinet,
            delete_record_no,
        ),
        Screen::SummaryTable { summaries } => app.view_summary_table(summaries),
    }
}
