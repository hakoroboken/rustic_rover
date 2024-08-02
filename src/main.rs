use iced::Application;
use rustic_rover::core::RusticRover;
use rustic_rover::core::utils::ApplicationDefaultSetting;

fn main()->iced::Result{
    let defalt_set = ApplicationDefaultSetting::new();
    RusticRover::run(defalt_set.settings)
}
