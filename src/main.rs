use rustic_rover::rr_core::RusticRover;
use rustic_rover::settings::ApplicationDefaultSetting;
use iced::Application;

fn main()->iced::Result{
    let defalt_set = ApplicationDefaultSetting::new();
    RusticRover::run(defalt_set.settings)
}