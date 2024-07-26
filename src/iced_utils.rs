use crate::interface::DualShock4;
use crate::dualshock::ControllerConnectionType;
use crate::AssignController;
use iced::window::settings::PlatformSpecific;
use iced::window::Settings as WindowSettings;
use iced::Settings;

use iced::widget::text;

use iced::widget::combo_box;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AppState
{
    Settings,
    NotModeSelected,
    ControllerNotFound,
    ControllerStarted
}

#[derive(Debug, Clone)]
pub enum Message
{
    ControllerThreadMessage(DualShock4),
    PowerRateX(u16),
    PowerRateY(u16),
    PowerRateRotation(u16),
    PowerRateM1(u16),
    PowerRateM2(u16),
    ControllerType(ControllerConnectionType),
    PacketAssign1p(AssignController),
    PacketAssign1m(AssignController),
    PacketAssign2p(AssignController),
    PacketAssign2m(AssignController),
    PacketAssign3p(AssignController),
    PacketAssign3m(AssignController),
    PacketAssign4p(AssignController),
    PacketAssign4m(AssignController),
    PacketAssign5p(AssignController),
    PacketAssign5m(AssignController),
    ControllerStart
}


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