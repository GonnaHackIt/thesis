use super::*;
use iced::widget::column;

#[derive(Default)]
pub struct PluginsSelect {
    pub all: Vec<String>,
    pub selected: String,
}

impl PluginsSelect {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        use Message::*;

        match message {
            PluginChange(selected) => {
                self.selected = selected;
                Task::none()
            }
            _ => Task::none(),
        }
    }
    pub fn view(&self) -> Element<Message> {
        column![
            text("Select plugin:"),
            pick_list(&self.all[..], Some(&self.selected), Message::PluginChange)
        ]
        .into()
    }
}
