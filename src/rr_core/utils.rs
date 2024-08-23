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
    let err_text = if app_state == AppState::NoReady
            {
                text("Let's Start Controller to press button!!").size(50)
            }
            else if app_state == AppState::ERROR
            {
                text("Controller is not connected!!").size(50)
            }
            else
            {
                text("App State Error").size(50)
            };

    err_text
}

pub fn path_to_image(path:&str, size:u16)->iced::widget::Image<iced::widget::image::Handle>
{
    iced::widget::image::Image::new(iced::widget::image::Handle::from_path(path)).width(size).height(size)
}

pub struct LogManager
{
    log_text:String,
}

impl LogManager {
    pub fn new()->LogManager
    {
        LogManager { log_text: String::new() }
    }

    pub fn add_str(&mut self ,str:String)
    {
        self.log_text = format!("{}\n{}",str, self.log_text.clone())
    }

    pub fn view<'a>(&self)->iced::widget::Text<'a, iced::Theme, iced::Renderer>
    {
        text(self.log_text.clone())
    }
}