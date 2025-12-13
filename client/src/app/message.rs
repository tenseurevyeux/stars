use shared::{Cabinet, Direction, Journal, Laborant, Student, Summary};

#[derive(Debug, Clone)]
pub enum Message {
    // Main
    DirectionButtonPressed,
    StudentButtonPressed,
    CabinetButtonPressed,
    LaborantButtonPressed,
    JournalButtonPressed,
    SummaryButtonPressed,

    // Direction
    DirectionsLoaded(Result<Vec<Direction>, String>),

    // Student
    StudentsLoaded(Result<Vec<Student>, String>),

    // Cabinet
    CabinetsLoaded(Result<Vec<Cabinet>, String>),

    // Laborant
    LaborantsLoaded(Result<Vec<Laborant>, String>),

    // Journal
    JournalLoaded(Result<Vec<Journal>, String>),

    // Summary
    SummaryLoaded(Result<Vec<Summary>, String>),

    // Management
    DirectionNameInputChanged(String),
    IdInputChanged(String),
    FioInputChanged(String),

    DirectionIdInputChanged(String),

    CabinetNumberInputChanged(String),

    JournalFkCabinetInputChanged(String),
    JournalRecordNoInputChanged(String),
    JournalDateInputChanged(String),
    JournalLessonInputChanged(String),
    JournalFkStudentInputChanged(String),
    JournalFkLabInputChanged(String),
    JournalDeleteFkCabinetInputChanged(String),
    JournalDeleteRecordNoInputChanged(String),

    // Actions
    CreatePressed,
    DeletePressed,
    UpdateTablePressed,
    BackPressed,
    OperationFinished(Result<(), String>),
}
