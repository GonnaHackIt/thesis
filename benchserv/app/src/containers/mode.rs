use super::*;
use iced::widget::column;

#[derive(Default)]
pub struct Mode {
    pub constant: bool,
    pub increase: bool,
}

impl Mode {
    pub fn chosen(&self) -> bool {
        self.constant | self.increase
    }
    pub fn view(&self) -> Element<Message> {
        column![
            text!("Test mode: "),
            checkbox("Constant connections", self.constant).on_toggle(Message::ConstantModeChanged),
            checkbox("Increase connections", self.increase).on_toggle(Message::IncreaseModeChanged)
        ]
        .into()
    }
    pub fn update(&mut self, message: Message) -> Task<Message> {
        use Message::*;

        self.constant = false;
        self.increase = false;

        match message {
            ConstantModeChanged(val) => self.constant = val,
            IncreaseModeChanged(val) => self.increase = val,
            _ => unreachable!(),
        }

        Task::none()
    }
}
