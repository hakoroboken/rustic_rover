use crate::interface::DualShock4;
use crate::dualshock::ControllerConnectionType;
use iced::window::settings::PlatformSpecific;
use iced::window::Settings as WindowSettings;
use iced::Settings;

use iced::widget::combo_box;

#[derive(Debug, PartialEq)]
pub enum AppState
{
    Settings,
    NotModeSelected,
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

pub struct ApplicationDefaultSetting
{
    pub settings:Settings<()>
}

impl ApplicationDefaultSetting {
    pub fn new()->ApplicationDefaultSetting
    {
        let window_setting = WindowSettings{
            size:iced::Size::INFINITY,
            position:iced::window::Position::Centered,
            min_size:None,
            max_size:None,
            visible:true,
            resizable:true,
            decorations:true,
            transparent:true,
            level:iced::window::Level::Normal,
            icon:None,
            platform_specific:PlatformSpecific{application_id:String::from("RusticRover")},
            exit_on_close_request:true
        };
        let mut settings_ = Settings::default();
        settings_.window = window_setting;

        ApplicationDefaultSetting { settings: settings_ }
    }
}