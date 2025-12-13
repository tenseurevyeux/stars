use crate::app::App;
use crate::app::Message;
use iced::Element;
use iced::widget::{button, column, container, row, scrollable, text, text_input};
use iced::{Alignment, Length};
use shared::Direction;

impl App {
    pub fn view_direction_table(
        &self,
        directions: &[Direction],
        name_input: &str,
        id_input: &str,
    ) -> Element<'_, Message> {
        let back_btn = button("Back").on_press(Message::BackPressed).padding(10);

        let update_btn = button(if self.loading { "..." } else { "Update" })
            .on_press(Message::UpdateTablePressed)
            .padding(10);

        let header_row = row![back_btn, text("Direction Table").size(24), update_btn]
            .spacing(20)
            .align_items(Alignment::Center);

        let create_input = text_input("Direction name", name_input)
            .on_input(Message::DirectionNameInputChanged)
            .width(Length::Fixed(200.0))
            .padding(10);

        let create_btn = button("Create")
            .on_press(Message::CreatePressed)
            .padding(10);

        let create_section = column![
            text("Create new direction").size(16),
            row![create_input, create_btn].spacing(10),
        ]
        .spacing(10);

        let delete_input = text_input("ID", id_input)
            .on_input(Message::IdInputChanged)
            .width(Length::Fixed(100.0))
            .padding(10);

        let delete_btn = button("Delete")
            .on_press(Message::DeletePressed)
            .padding(10);

        let delete_section = column![
            text("Delete direction by ID:").size(16),
            row![delete_input, delete_btn].spacing(10),
        ]
        .spacing(10);

        let mut table_items = column![];

        if directions.is_empty() {
            table_items = table_items.push(text("(Table is empty)").size(14));
        } else {
            table_items = table_items.push(
                row![
                    text("ID").width(Length::Fixed(50.0)),
                    text("Name").width(Length::Fixed(200.0)),
                ]
                .padding(10)
                .spacing(10),
            );

            for d in directions {
                table_items = table_items.push(
                    row![
                        text(format!("{}", d.id_direction)).width(Length::Fixed(50.0)),
                        text(&d.name_direction).width(Length::Fixed(200.0)),
                    ]
                    .padding(8)
                    .spacing(10),
                );
            }
        }

        let table_section = column![
            text("All Directions:").size(18),
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
