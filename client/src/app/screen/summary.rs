use crate::app::App;
use crate::app::Message;
use iced::Element;
use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Alignment, Length};
use shared::Summary;

impl App {
    pub fn view_summary_table(&self, summaries: &[Summary]) -> Element<'_, Message> {
        let back_btn = button("Back").on_press(Message::BackPressed).padding(10);

        let update_btn = button(if self.loading { "..." } else { "Update" })
            .on_press(Message::UpdateTablePressed)
            .padding(10);

        let header_row = row![back_btn, text("Summary Table").size(24), update_btn]
            .spacing(20)
            .align_items(Alignment::Center);

        // Table display
        let mut table_items = column![];

        if summaries.is_empty() {
            table_items = table_items.push(text("(Table is empty)").size(14));
        } else {
            table_items = table_items.push(
                row![
                    text("Num cab").width(Length::Fixed(70.0)),
                    text("Rec#").width(Length::Fixed(50.0)),
                    text("Date").width(Length::Fixed(100.0)),
                    text("Lesson").width(Length::Fixed(80.0)),
                    text("Stud").width(Length::Fixed(300.0)),
                    text("Direction").width(Length::Fixed(200.0)),
                    text("Lab").width(Length::Fixed(300.0)),
                ]
                .padding(10)
                .spacing(5),
            );

            for s in summaries {
                table_items = table_items.push(
                    row![
                        text(format!("{}", s.num_cabinet)).width(Length::Fixed(70.0)),
                        text(format!("{}", s.record_no)).width(Length::Fixed(50.0)),
                        text(&s.date).width(Length::Fixed(100.0)),
                        text(&s.lesson).width(Length::Fixed(80.0)),
                        text(format!("{}", s.fio_student)).width(Length::Fixed(300.0)),
                        text(format!("{}", s.name_direction)).width(Length::Fixed(200.0)),
                        text(format!("{}", s.fio_lab)).width(Length::Fixed(300.0)),
                    ]
                    .padding(8)
                    .spacing(5),
                );
            }
        }

        let table_section = column![
            text("All Summary Entries:").size(18),
            scrollable(table_items).height(Length::Fixed(300.0)),
        ]
        .spacing(10);

        let status_text = if self.loading {
            "Loading...".to_string()
        } else {
            self.status.clone()
        };

        let content = column![header_row, table_section, text(status_text).size(14),]
            .spacing(20)
            .padding(20);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
