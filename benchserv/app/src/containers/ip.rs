use super::*;
use iced::widget::column;
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
