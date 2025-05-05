use super::*;
use iced::widget::column;

#[derive(Default)]
pub struct Connections {
    pub text_state: TextInputState,
    pub value: f64,
}

impl Deref for Connections {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DerefMut for Connections {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl Connections {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        use Message::*;
        self.text_state.incorrect = false;

        match message {
            ConnectionsChangedSlider(value) => {
                self.value = value;
                self.text_state.content = value.to_string();

                Task::none()
            }
            ConnectionsChangedInput(content) => {
                self.text_state.content = content.clone();

                if let Ok(value) = content.parse::<u64>() {
                    self.value = value as f64;
                } else {
                    self.text_state.incorrect = true;
                    self.value = 0.0;
                }

                Task::none()
            }
            _ => Task::none(),
        }
    }
    pub fn view(&self, constant: bool) -> Element<Message> {
        let text = if constant {
            "Connections Number"
        } else {
            "Max connections"
        };

        let err = if self.text_state.incorrect {
            "Bad connections number"
        } else {
            ""
        };

        column![
            row![
                text!("{text}: "),
                text_input("0", &self.text_state.content)
                    .on_input(Message::ConnectionsChangedInput)
                    .width(90),
            ],
            slider(
                0.0..=10_000.0,
                self.value,
                Message::ConnectionsChangedSlider
            )
            .step(1.0),
            text!("{err}").color(Color::from_rgb(255.0, 0.0, 0.0))
        ]
        .into()
    }
}
