use crate::app::App;
use crate::app::Message;
use iced::Element;
use iced::widget::{button, column, container, row, scrollable, text, text_input};
use iced::{Alignment, Length};
use shared::Journal;

impl App {
    pub fn view_journal_table(
        &self,
        journals: &[Journal],
        fk_cabinet: &str,
        record_no: &str,
        date: &str,
        lesson: &str,
        fk_student: &str,
        fk_lab: &str,
        delete_fk_cabinet: &str,
        delete_record_no: &str,
    ) -> Element<'_, Message> {
        let back_btn = button("Back").on_press(Message::BackPressed).padding(10);

        let update_btn = button(if self.loading { "..." } else { "Update" })
            .on_press(Message::UpdateTablePressed)
            .padding(10);

        let header_row = row![back_btn, text("Journal Table").size(24), update_btn]
            .spacing(20)
            .align_items(Alignment::Center);

        // Create section with all input fields
        let create_fk_cabinet_input = text_input("Cabinet ID", fk_cabinet)
            .on_input(Message::JournalFkCabinetInputChanged)
            .width(Length::Fixed(80.0))
            .padding(8);

        let create_record_no_input = text_input("Record #", record_no)
            .on_input(Message::JournalRecordNoInputChanged)
            .width(Length::Fixed(80.0))
            .padding(8);

        let create_date_input = text_input("Date (YYYY-MM-DD)", date)
            .on_input(Message::JournalDateInputChanged)
            .width(Length::Fixed(120.0))
            .padding(8);

        let create_lesson_input = text_input("Lesson", lesson)
            .on_input(Message::JournalLessonInputChanged)
            .width(Length::Fixed(120.0))
            .padding(8);

        let create_fk_student_input = text_input("Student ID", fk_student)
            .on_input(Message::JournalFkStudentInputChanged)
            .width(Length::Fixed(80.0))
            .padding(8);

        let create_fk_lab_input = text_input("Laborant ID", fk_lab)
            .on_input(Message::JournalFkLabInputChanged)
            .width(Length::Fixed(80.0))
            .padding(8);

        let create_btn = button("Create")
            .on_press(Message::CreatePressed)
            .padding(10);

        let create_section = column![
            text("Create new journal entry").size(16),
            row![
                create_fk_cabinet_input,
                create_record_no_input,
                create_date_input,
            ]
            .spacing(8),
            row![
                create_lesson_input,
                create_fk_student_input,
                create_fk_lab_input,
            ]
            .spacing(8),
            create_btn,
        ]
        .spacing(8);

        // Delete section - теперь использует ОТДЕЛЬНЫЕ поля delete_fk_cabinet и delete_record_no
        let delete_cabinet_input = text_input("Cabinet ID", delete_fk_cabinet)
            .on_input(Message::JournalDeleteFkCabinetInputChanged)
            .width(Length::Fixed(100.0))
            .padding(10);

        let delete_record_input = text_input("Record #", delete_record_no)
            .on_input(Message::JournalDeleteRecordNoInputChanged)
            .width(Length::Fixed(100.0))
            .padding(10);

        let delete_btn = button("Delete")
            .on_press(Message::DeletePressed)
            .padding(10);

        let delete_section = column![
            text("Delete by Cabinet ID and Record #:").size(16),
            row![delete_cabinet_input, delete_record_input, delete_btn].spacing(10),
        ]
        .spacing(10);

        // Table display
        let mut table_items = column![];

        if journals.is_empty() {
            table_items = table_items.push(text("(Table is empty)").size(14));
        } else {
            table_items = table_items.push(
                row![
                    text("Cabinet").width(Length::Fixed(70.0)),
                    text("Rec#").width(Length::Fixed(50.0)),
                    text("Date").width(Length::Fixed(100.0)),
                    text("Lesson").width(Length::Fixed(80.0)),
                    text("Stud").width(Length::Fixed(50.0)),
                    text("Lab").width(Length::Fixed(50.0)),
                ]
                .padding(10)
                .spacing(5),
            );

            for j in journals {
                table_items = table_items.push(
                    row![
                        text(format!("{}", j.fk_cabinet)).width(Length::Fixed(70.0)),
                        text(format!("{}", j.record_no)).width(Length::Fixed(50.0)),
                        text(&j.date).width(Length::Fixed(100.0)),
                        text(&j.lesson).width(Length::Fixed(80.0)),
                        text(format!("{}", j.fk_student)).width(Length::Fixed(50.0)),
                        text(format!("{}", j.fk_lab)).width(Length::Fixed(50.0)),
                    ]
                    .padding(8)
                    .spacing(5),
                );
            }
        }

        let table_section = column![
            text("All Journal Entries:").size(18),
            scrollable(table_items).height(Length::Fixed(300.0)),
        ]
        .spacing(10);

        let status_text = if self.loading {
            "Loading...".to_string()
        } else {
            self.status.clone()
        };

        let content = column![
            header_row,
            create_section,
            delete_section,
            table_section,
            text(status_text).size(14),
        ]
        .spacing(20)
        .padding(20);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
