use crate::app::App;
use crate::app::Message;
use iced::Element;
use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Length};

impl App {
    pub fn view_main_menu(&self) -> Element<'_, Message> {
        let direction_btn = button("Direction")
            .on_press(Message::DirectionButtonPressed)
            .padding(15);

        let student_btn = button("Student")
            .on_press(Message::StudentButtonPressed)
            .padding(15);

        let cabinet_btn = button("Cabinet")
            .on_press(Message::CabinetButtonPressed)
            .padding(15);

        let laborant_btn = button("Laborant")
            .on_press(Message::LaborantButtonPressed)
            .padding(15);

        let journal_btn = button("Journal")
            .on_press(Message::JournalButtonPressed)
            .padding(15);
        let summary_btn = button("Summary")
            .on_press(Message::SummaryButtonPressed)
            .padding(15);

        let content = column![
            text("University Management System").size(32),
            text("").size(20),
            text("Select table:").size(18),
            row![
                direction_btn,
                student_btn,
                cabinet_btn,
                laborant_btn,
                journal_btn,
                summary_btn
            ]
            .spacing(20)
            .align_items(Alignment::Center),
        ]
        .spacing(20)
        .padding(40)
        .align_items(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
