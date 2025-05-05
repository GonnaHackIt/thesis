use crate::Message;
use iced::{
    Alignment, Color, Element, Length, Padding, Task,
    widget::{
        button,
        button::{danger, primary, secondary},
        checkbox, container, pick_list, row, slider, text, text_input,
    },
};
use std::{
    default::Default,
    ops::{Deref, DerefMut},
};

pub mod buttons;
pub mod chart;
pub mod connections;
pub mod ip;
pub mod mode;
pub mod plugins;

pub use buttons::*;
pub use chart::*;
pub use connections::*;
pub use ip::*;
pub use mode::*;
pub use plugins::*;

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
