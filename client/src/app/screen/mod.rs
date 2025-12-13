use shared::{Cabinet, Direction, Journal, Laborant, Student, Summary};

pub mod cabinet;
pub mod direction;
pub mod journal;
pub mod laborant;
pub mod main;
pub mod student;
pub mod summary;

#[derive(Debug)]
pub enum Screen {
    MainMenu,
    DirectionTable {
        directions: Vec<Direction>,
        name_input: String,
        id_input: String,
    },
    StudentTable {
        students: Vec<Student>,
        fio_input: String,
        direction_id_input: String,
        id_input: String,
    },
    CabinetTable {
        cabinets: Vec<Cabinet>,
        number_input: String,
        id_input: String,
    },
    LaborantTable {
        laborants: Vec<Laborant>,
        fio_input: String,
        id_input: String,
    },
    JournalTable {
        journals: Vec<Journal>,
        // Create fields
        fk_cabinet: String,
        record_no: String,
        date: String,
        lesson: String,
        fk_student: String,
        fk_lab: String,
        // Delete fields
        delete_fk_cabinet: String,
        delete_record_no: String,
    },
    SummaryTable {
        summaries: Vec<Summary>,
    },
}
