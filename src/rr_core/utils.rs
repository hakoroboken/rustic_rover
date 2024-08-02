use crate::rr_core::interface::AppState;
use iced::widget::text;
use iced::widget::combo_box;

#[derive(Clone)]
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

pub fn setting_state_logger<'a>(app_state:AppState)->iced::widget::text::Text<'a,iced::Theme, iced::Renderer>
{
    let err_text = if app_state == AppState::Settings
            {
                text("").size(50)
            }
            else if app_state == AppState::NotModeSelected
            {
                text("Not Mode Selected!!").size(50)
            }
            else if app_state == AppState::ControllerNotFound
            {
                text("Controller is not connected!!").size(50)
            }
            else
            {
                text("App State Error").size(50)
            };

    err_text
}