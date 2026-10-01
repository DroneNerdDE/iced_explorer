use iced::widget::{button, checkbox, column, row, scrollable, text, Container};
use iced::{Element, Length};
use crate::app::{FileExplorer, Message};

pub fn view(state: &FileExplorer) -> Element<Message> {
    let mut file_list = column![].spacing(5);

    // Display the current path as a header
    file_list = file_list.push(
        text(format!("Current: {}", state.current_dir.display())).size(20)
    );

    // Button to go to parent dir
    if let Some(parent) = state.current_dir.parent() {
        file_list = file_list.push(
            button("Up to Parent Directory")
            .on_press(Message::Navigate(parent.to_path_buf()))
        );
    }

    let header = row![
        text("Name").width(Length::FillPortion(5)),
        text("Size").width(Length::FillPortion(2)),
        text("Modified").width(Length::FillPortion(3)),
        text("Permissions").width(Length::FillPortion(2)),
    ].spacing(10).padding([0, 10]);

    file_list = file_list.push(
        checkbox(state.show_hidden)
            .label("Show Hidden Files")
            .on_toggle(Message::ToggleHidden)
    );

    file_list = file_list.push(header);
    

    for file in &state.files {
        let icon = if file.is_dir {"📁"} else {"📄"};

        let content = row![
            text(format!("{} {}", icon, file.name)).width(Length::FillPortion(5)),
            text(&file.size).width(Length::FillPortion(2)),
            text(&file.modified).width(Length::FillPortion(3)),
            text(&file.permissions).width(Length::FillPortion(2)),
        ].spacing(10);
        
        let mut btn = button(content).width(Length::Fill);
        
        // Only make dirs clickable
        if file.is_dir {
            btn = btn.on_press(Message::Navigate(file.path.clone()));
        }

        file_list = file_list.push(btn);
    }

    Container::new(scrollable(file_list))
        .padding(20)
        .width(Length::Fill)
        .height(Length::Fill)
        .into() // Convert container widget into Element
}
