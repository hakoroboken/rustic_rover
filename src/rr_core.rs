mod dualshock;
mod interface;
mod thread_connection;
mod packet;
mod utils;

use interface::{DualShock4, ControllerConnectionType, AppState};

use iced::{self, Element};
use iced::widget::{button, text, combo_box, column, row};

pub struct RusticRover
{
    dualshock4_connector:thread_connection::ThreadConnector<DualShock4>,
    ds4_input:DualShock4,
    controller_connection_types_combo_box:utils::ComboBox<ControllerConnectionType>,
    packet_creator:packet::PacketCreator,
    app_state:AppState,
}

impl iced::Application for RusticRover {
    type Executor = iced::executor::Default;
    type Message = interface::RRMessage;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let ds4_conn = thread_connection::ThreadConnector::<DualShock4>::new();

        let app = RusticRover
        {
            dualshock4_connector: ds4_conn,
            ds4_input: DualShock4::new(),
            controller_connection_types_combo_box:utils::ComboBox::new(ControllerConnectionType::ALL.to_vec()),
            packet_creator:packet::PacketCreator::new(),
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

                (interface::RRMessage::ControllerThreadMessage(get), subscriber)
            })
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            interface::RRMessage::ControllerThreadMessage(ds4)=>{
                self.ds4_input = ds4;
                
                self.packet_creator.create_packet(ds4);
            }
            interface::RRMessage::ControllerType(type_)=>{
                self.controller_connection_types_combo_box.selected = Some(type_);
            }
            interface::RRMessage::ControllerStart=>{
                if self.controller_connection_types_combo_box.selected == None
                {
                    self.app_state = AppState::NotModeSelected;
                }
                else 
                {
                    match dualshock::DualShock4Driver::new(self.controller_connection_types_combo_box.selected.unwrap()) {
                        Some(mut dr)=>{
                            let t = self.dualshock4_connector.publisher.clone().take().unwrap();

                            std::thread::spawn(move ||{
                                loop {
                                    let get = dr.task();

                                    t.clone().send(get).unwrap();
                                }
                            });
                            self.app_state = AppState::ControllerStarted;
                        }
                        None=>{
                            self.app_state = AppState::ControllerNotFound
                        }
                    }
                }
            }
            interface::RRMessage::PowerRateX(get_rate)=>{
                self.packet_creator.x_pow_rate = get_rate
            }
            interface::RRMessage::PowerRateY(get_rate)=>{
                self.packet_creator.y_pow_rate = get_rate
            }
            interface::RRMessage::PowerRateRotation(get_rate)=>{
                self.packet_creator.ro_pow_rate = get_rate;
            }
            interface::RRMessage::PowerRateM1(get_rate)=>{
                self.packet_creator.m1_pow_rate = get_rate;
            }
            interface::RRMessage::PowerRateM2(get_rate)=>{
                self.packet_creator.m2_pow_rate = get_rate;
            }
            interface::RRMessage::PacketAssign1p(a1p)=>{
                self.packet_creator.x_cb.plus.selected = Some(a1p)
            }
            interface::RRMessage::PacketAssign1m(a1m)=>{
                self.packet_creator.x_cb.minus.selected = Some(a1m)
            }
            interface::RRMessage::PacketAssign2p(a2p)=>{
                self.packet_creator.y_cb.plus.selected = Some(a2p)
            }
            interface::RRMessage::PacketAssign2m(a2m)=>{
                self.packet_creator.y_cb.minus.selected = Some(a2m)
            }
            interface::RRMessage::PacketAssign3p(a3p)=>{
                self.packet_creator.ro_cb.plus.selected = Some(a3p)
            }
            interface::RRMessage::PacketAssign3m(a3m)=>{
                self.packet_creator.ro_cb.minus.selected = Some(a3m)
            }
            interface::RRMessage::PacketAssign4p(a4p)=>{
                self.packet_creator.m1_cb.plus.selected = Some(a4p)
            }
            interface::RRMessage::PacketAssign4m(a4m)=>{
                self.packet_creator.m1_cb.minus.selected = Some(a4m)
            }
            interface::RRMessage::PacketAssign5p(a5p)=>{
                self.packet_creator.m2_cb.plus.selected = Some(a5p)
            }
            interface::RRMessage::PacketAssign5m(a5m)=>{
                self.packet_creator.m2_cb.minus.selected = Some(a5m)
            }
        }

        iced::Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        if self.app_state == AppState::Settings || self.app_state == AppState::NotModeSelected || self.app_state == AppState::ControllerNotFound
        {
            self.title_view()
        }
        else if self.app_state == AppState::ControllerStarted
        {
            let con_state = if self.ds4_input.state
            {
                "Connected!!"
            }
            else
            {
                "Not Connected"
            };

            let lx = self.ds4_input.sticks.left_x;
            let ly = self.ds4_input.sticks.left_y;
            let rx = self.ds4_input.sticks.right_x;
            let tit = text("Controller Info").size(70);
            let tex = text(
                format!("Type:{}\nState:{}\nJoyLeftX:{}\nJoyLeftY:{}\nJoyRightX:{}",self.ds4_input.mode, con_state, lx, ly, rx)
            ).size(40);

            let controller_clm = column![tit, tex].align_items(iced::Alignment::Start);

            row![controller_clm, self.packet_creator.packet_view()].spacing(50).into()
        }
        else {
            text("App State Error").size(300).into()
        }
    }
}

impl RusticRover {
    fn title_view(&self)->Element<'_, interface::RRMessage, iced::Theme, iced::Renderer>
    {
        let title = text("RusticRover").size(200).horizontal_alignment(iced::alignment::Horizontal::Center);
        let combo_ = combo_box(
            &self.controller_connection_types_combo_box.all, 
            "Select Controller Connection Method", 
            self.controller_connection_types_combo_box.selected.as_ref(), 
        interface::RRMessage::ControllerType);

        let path = "./rustic_rover.png";

        let img = iced::widget::image::Image::new(iced::widget::image::Handle::from_path(path)).width(iced::Length::Shrink).height(iced::Length::Shrink);

        let btn = button("Start").on_press(interface::RRMessage::ControllerStart).width(iced::Length::Shrink).height(iced::Length::Shrink);

        let err_text = utils::setting_state_logger(self.app_state);

        column![title, combo_, btn, err_text,img].align_items(iced::alignment::Alignment::Center).padding(10).spacing(50).into()
    }
}