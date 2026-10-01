use iced::widget::{button, checkbox, column, row, scrollable, text, Container};
use iced::{Element, Length};
use crate::app::{FileExplorer, Message};

pub fn view(state: &FileExplorer) -> Element<Message> {
    let mut file_list = column![].spacing(10);

    // Current Path Header
    file_list = file_list.push(
        text(format!("Current: {}", state.current_dir.display())).size(20)
    );

    // Nav Toolbar
    let mut toolbar = row![].spacing(10);

    // Back Button
    let mut back_btn = button("<-");
    if !state.history_back.is_empty() {
        back_btn = back_btn.on_press(Message::Backward);
    }
    toolbar = toolbar.push(back_btn);

    // Forward Button
    let mut fwd_btn = button("->");
    if !state.history_forward.is_empty() {
        fwd_btn = fwd_btn.on_press(Message::Forward);
    }
    toolbar = toolbar.push(fwd_btn);

    // Parent Dir Button
    let mut parent_btn = button("Parent");
    if let Some(parent) = state.current_dir.parent() {
        parent_btn = parent_btn.on_press(Message::Navigate(parent.to_path_buf()));
    }
    toolbar = toolbar.push(parent_btn);

    // Home Button
    toolbar = toolbar.push(button("Home").on_press(Message::Home));

    // Add Toolbar to Layout
    file_list = file_list.push(toolbar);

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
