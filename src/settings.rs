use iced::window::settings::PlatformSpecific;
use iced::window::Settings as WindowSettings;
use iced::Settings;

pub struct ApplicationDefaultSetting
{
    pub settings:Settings<()>
}

impl ApplicationDefaultSetting {
    pub fn new()->ApplicationDefaultSetting
    {
        let icon_ = iced::window::icon::from_file("./rustic_rover.png").unwrap();
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
            icon:Some(icon_),
            platform_specific:PlatformSpecific{application_id:String::from("RusticRover")},
            exit_on_close_request:true
        };
        let mut settings_ = Settings::default();
        settings_.window = window_setting;

        ApplicationDefaultSetting { settings: settings_ }
    }
}