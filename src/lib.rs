pub mod dualshock;
use dualshock::ControllerConnectionType;
pub mod interface;
use interface::DualShock4;
pub mod thread_connection;
pub mod iced_app_utils;



use iced;
use iced_app_utils::{AppState, Message};
use iced::widget::{button, text, combo_box, column, slider};

pub struct RusticRover
{
    dualshock4_connector:thread_connection::ThreadConnector<DualShock4>,
    ds4_input:DualShock4,
    power_rate:u16,
    controller_connection_types_combo_box:iced_app_utils::ComboBox<ControllerConnectionType>,
    app_state:AppState
}

impl iced::Application for RusticRover {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let ds4_conn = thread_connection::ThreadConnector::<DualShock4>::init();

        let app = RusticRover
        {
            dualshock4_connector: ds4_conn,
            ds4_input: DualShock4::new(),
            power_rate:100,
            controller_connection_types_combo_box:iced_app_utils::ComboBox::new(ControllerConnectionType::ALL.to_vec()),
            app_state:AppState::Settings
        };

        (app, iced::Command::none())
    }

    fn title(&self) -> String {
        String::from("RusticRover")
    }

    fn theme(&self) -> Self::Theme {
        iced::Theme::Light
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::subscription::unfold(
            "subscribe_controller_input", 
            self.dualshock4_connector.subscriber.take(), 
            move |mut subscriber|async move{
                let get = subscriber.as_mut().unwrap().recv().await.unwrap();

                (Message::ControllerThreadMessage(get), subscriber)
            })
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::ControllerThreadMessage(ds4)=>{
                self.ds4_input = ds4;
            }
            Message::ControllerType(type_)=>{
                self.controller_connection_types_combo_box.selected = Some(type_);
            }
            Message::ControllerStart=>{
                let mut dr = dualshock::DualShock4Driver::new(self.controller_connection_types_combo_box.selected.unwrap()).unwrap();

                let t = self.dualshock4_connector.publisher.clone().take().unwrap();

                std::thread::spawn(move ||{
                    loop {
                        let get = dr.task();

                        t.clone().send(get).unwrap();
                    }
                });
                self.app_state = AppState::ControllerStarted;
            }
            Message::PowerRate(get_rate)=>{
                self.power_rate = get_rate;
            }
        }

        iced::Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        if self.app_state == AppState::Settings
        {
            let title = text("RusticRover").size(200).horizontal_alignment(iced::alignment::Horizontal::Center);
            let combo_ = combo_box(
                &self.controller_connection_types_combo_box.all, 
                "Select Controller Connection Method", 
                self.controller_connection_types_combo_box.selected.as_ref(), 
            Message::ControllerType);

            let path = "./rustic_rover.png";

            let img = iced::widget::image::Image::new(iced::widget::image::Handle::from_path(path)).width(iced::Length::Shrink).height(iced::Length::Shrink);

            let btn = button("Start").on_press(Message::ControllerStart).width(iced::Length::Shrink).height(iced::Length::Shrink);

            column![title, combo_, btn, img].align_items(iced::alignment::Alignment::Center).spacing(50).into()
        }
        else if self.app_state == AppState::ControllerStarted
        {
            let sc = slider(
                0..=100, 
                self.power_rate, 
            Message::PowerRate).width(500);

            let con_state = if self.ds4_input.state
            {
                "Connected!!"
            }
            else
            {
                "Not Connected"
            };

            let r = self.power_rate as f32 / 100.0;

            let lx = self.ds4_input.sticks.left_x* r;
            let ly = self.ds4_input.sticks.left_y* r;
            let rx = self.ds4_input.sticks.right_x* r;
            let tit = text("Controller Info").size(70);
            let tex = text(
                format!("Type:{}\nState:{}\nx:{}\ny:{}\nrotation:{}\nOutputRate:{}%",self.ds4_input.mode, con_state, lx, ly, rx, self.power_rate)
            ).size(40);

            column![tit, tex, sc].align_items(iced::Alignment::Start).spacing(20).into()
        }
        else {
            text("Destruction APP!! ERROR").size(300).into()
        }
    }
}