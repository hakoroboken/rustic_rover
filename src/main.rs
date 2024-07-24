use iced::Application;
use rustic_rover::RusticRover;
use rustic_rover::iced_utils::ApplicationDefaultSetting;

fn main()->iced::Result{
    let defalt_set = ApplicationDefaultSetting::new();
    RusticRover::run(defalt_set.settings)
}
