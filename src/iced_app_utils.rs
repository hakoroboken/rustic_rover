use crate::interface::DualShock4;
use crate::dualshock::ControllerConnectionType;

use iced::widget::combo_box;

#[derive(Debug, PartialEq)]
pub enum AppState
{
    Settings,
    ControllerStarted
}

#[derive(Debug, Clone)]
pub enum Message
{
    ControllerThreadMessage(DualShock4),
    PowerRate(u16),
    ControllerType(ControllerConnectionType),
    ControllerStart
}


pub struct ComboBox<T>
{
    pub all:iced::widget::combo_box::State<T>,
    pub selected:Option<T>,
}

impl<T: std::fmt::Display + std::clone::Clone> ComboBox<T> {
    pub fn new(all_list:Vec<T>)->ComboBox<T>
    {
        ComboBox { all: combo_box::State::new(all_list), selected: None }
    }
}