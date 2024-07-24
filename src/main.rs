use iced::Settings;
use iced::Application;
use rustic_rover::RusticRover;

fn main()->iced::Result{
    RusticRover::run(Settings::default())
}
