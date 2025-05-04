use crate::Message;
use iced::{
    Alignment, Color, Element, Length, Task,
    widget::{
        button,
        button::{danger, primary, secondary},
        checkbox, column, container, pick_list, row, slider, text, text_input,
    },
};
use std::{
    default::Default,
    ops::{Deref, DerefMut},
};

#[derive(Default)]
pub struct TextInputState {
    pub content: String,
    pub incorrect: bool,
}

impl TextInputState {
    fn new(content: String, incorrect: bool) -> Self {
        TextInputState { content, incorrect }
    }
}

pub struct Ip(pub TextInputState);

impl Default for Ip {
    fn default() -> Self {
        Ip(TextInputState::new("127.0.0.1".to_owned(), false))
    }
}

impl Deref for Ip {
    type Target = TextInputState;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Ip {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Ip {
    fn check(&self, ip: &str) -> bool {
        if ip == "localhost" {
            return true;
        }

        let split = ip.split(".").collect::<Vec<_>>();

        if split.len() < 4 {
            return false;
        }

        if split.iter().any(|octet| octet.parse::<u8>().is_err()) {
            return false;
        }

        return true;
    }
    pub fn update(&mut self, message: Message) -> Task<Message> {
        use Message::*;

        match message {
            IpChanged(new_content) => {
                self.incorrect = false;

                if new_content.len() > 0 && !self.check(&new_content) {
                    self.incorrect = true;
                }

                self.content = new_content;

                Task::none()
            }
            _ => Task::none(),
        }
    }
    pub fn view(&self) -> Element<Message> {
        let ip_err = if self.incorrect { "Bad ip address" } else { "" };

        column![
            text!("Server IP:"),
            text_input("Server's IP", &self.content)
                .on_input(Message::IpChanged)
                .width(170),
            text(ip_err).color(Color::from_rgb(255.0, 0.0, 0.0))
        ]
        .into()
    }
}

pub struct Port(pub TextInputState);

impl Default for Port {
    fn default() -> Self {
        Port(TextInputState::new("80".to_owned(), false))
    }
}

impl Deref for Port {
    type Target = TextInputState;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Port {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Port {
    fn check(&self, port: &str) -> bool {
        if port.parse::<u16>().is_err() {
            return false;
        }

        true
    }
    pub fn update(&mut self, message: Message) -> Task<Message> {
        use Message::*;

        match message {
            PortChanged(new_content) => {
                self.incorrect = false;

                if new_content.len() > 0 && !self.check(&new_content) {
                    self.incorrect = true;
                }

                self.content = new_content;

                Task::none()
            }
            _ => Task::none(),
        }
    }
    pub fn view(&self) -> Element<Message> {
        let port_err = if self.incorrect {
            "Bad server port"
        } else {
            ""
        };

        column![
            text!("Server Port:"),
            text_input("Port", &self.content)
                .on_input(Message::PortChanged)
                .width(90),
            text(port_err).color(Color::from_rgb(255.0, 0.0, 0.0))
        ]
        .into()
    }
}

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

#[derive(Default)]
pub struct Buttons {}

impl Buttons {
    pub fn view(&self, cant_run: bool, test_running: bool, paused: bool) -> Element<Message> {
        macro_rules! new_button {
            ($content:expr, $msg:expr) => {
                button($content).on_press($msg)
            };
        }
        let run_error = if cant_run {
            "Fill all inputs before running!"
        } else {
            ""
        };

        // buttons
        let buttons = row![];

        let run_button = (!test_running).then_some(
            new_button!("Run", Message::RunTest).style(|theme, status| primary(theme, status)),
        );
        let stop_button = test_running.then_some(
            new_button!("Stop", Message::Stop).style(|theme, status| danger(theme, status)),
        );

        let resume_pause_button = if test_running && !paused {
            Some(
                new_button!("Pause", Message::Pause)
                    .style(|theme, status| secondary(theme, status)),
            )
        } else if test_running && paused {
            Some(
                new_button!("Resume", Message::Resume)
                    .style(|theme, status| primary(theme, status)),
            )
        } else {
            None
        };

        let buttons = buttons
            .push_maybe(run_button)
            .push_maybe(resume_pause_button)
            .push_maybe(stop_button)
            .spacing(10);

        column![
            container(buttons)
                .width(Length::Fill)
                .align_x(Alignment::End),
            text(run_error)
                .color(Color::from_rgb(255.0, 0.0, 0.0))
                .width(Length::Fill)
                .align_x(Alignment::End)
        ]
        .into()
    }
}

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
