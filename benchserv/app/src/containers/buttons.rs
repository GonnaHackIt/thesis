use super::*;
use iced::widget::column;

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
