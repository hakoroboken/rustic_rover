mod dualshock;
mod interface;
mod thread_connection;
mod packet;
mod utils;
mod serial;

use interface::{AppState, ControllerConnectionType, DualShock4, Packet, RRMessage, Status};

use iced::{self, Element};
use iced::widget::{button, text, combo_box, column, row, text_input};
use serial::SerialManager;

pub struct RusticRover
{
    dualshock4_connector:thread_connection::AsyncThreadConnector<DualShock4>,
    ds4_input:DualShock4,
    controller_connection_types_combo_box:utils::ComboBox<ControllerConnectionType>,
    packet_creator:packet::PacketCreator,
    status:Status,
    serial_manager:serial::SerialManager,
    input_path:String
}

impl iced::Application for RusticRover {
    type Executor = iced::executor::Default;
    type Message = interface::RRMessage;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let ds4_conn = thread_connection::AsyncThreadConnector::<DualShock4>::new();

        let app = RusticRover
        {
            dualshock4_connector: ds4_conn,
            ds4_input: DualShock4::new(),
            controller_connection_types_combo_box:utils::ComboBox::new(ControllerConnectionType::ALL.to_vec()),
            packet_creator:packet::PacketCreator::new(),
            status:Status::new(),
            serial_manager:SerialManager::new(),
            input_path:String::new()
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

                match self.packet_creator.packet_ {
                    Some(p)=>{
                        self.status.packet_state = AppState::OK;
                        
                        if self.status.serial_state == AppState::OK
                        {
                            self.serial_manager.conn.publisher.send(p).unwrap();
                        }
                    }
                    None=>{
                        self.status.packet_state = AppState::NoReady;
                    }
                }
            }
            interface::RRMessage::ControllerType(type_)=>{
                self.controller_connection_types_combo_box.selected = Some(type_);
            }
            interface::RRMessage::ControllerStart=>{
                if self.controller_connection_types_combo_box.selected == None
                {
                    self.status.controller_state = AppState::NoReady;
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
                            self.status.controller_state = AppState::OK;
                        }
                        None=>{
                            self.status.controller_state = AppState::ERROR
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
            },
            interface::RRMessage::SerialPathInput(path)=>{
                self.input_path = path
            }
            interface::RRMessage::SerialStart=>{
                let con_p = thread_connection::ThreadConnector::<Packet>::new();
                self.serial_manager.conn.publisher = con_p.publisher.clone();
                // let state_publisher = self.serial_manager.state_mailer.publisher.clone();
                let port_name_ = self.input_path.clone();

                self.status.serial_state = AppState::OK;
                
                std::thread::spawn(move ||{
                    let mut port_ = serialport::new(port_name_.clone().as_str(), 115200)
                        .data_bits(serialport::DataBits::Eight)
                        .stop_bits(serialport::StopBits::One)
                        .timeout(std::time::Duration::from_millis(100))
                        .open().unwrap();
                    loop {
                        let p = con_p.subscriber.recv().unwrap();
                        
                        let write_buf = format!("s{},{},{},{},{}e", 
                                p.x/10 as i32+10,
                                p.y/10 as i32+10,
                                p.ro/10 as i32+10,
                                p.m1/10 as i32+10,
                                p.m2/10 as i32+10);

                        match port_.write(write_buf.as_bytes()) {
                            Ok(_)=>{
                            }
                            Err(_)=>{

                            }
                        }
                    }
                });
                    // match serialport::new(port_name_.as_str(), 115200)
                    //     .data_bits(serialport::DataBits::Eight)
                    //     .stop_bits(serialport::StopBits::One)
                    //     .timeout(std::time::Duration::from_millis(100))
                    //     .open()
                    // {
                    //     Ok(p)=>{
                    //         self.serial_manager.port_ = p
                    //     }
                    //     Err(_)=>{

                    //     }
                    // }
                
            }
        }

        iced::Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        if self.status.controller_state == AppState::NoReady || self.status.controller_state == AppState::ERROR
        {
            self.title_view()
        }
        else if self.status.controller_state == AppState::OK
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
            
            if self.status.packet_state == AppState::OK
            {
                if self.status.serial_state == AppState::NoReady
                {
                    let sp_input = text_input("Input Serial Path", self.input_path.as_str())
                    .on_input(RRMessage::SerialPathInput)
                    .on_submit(RRMessage::SerialStart);

                    let controller_clm = column![tit, tex, sp_input].align_items(iced::Alignment::Start);

                    row![controller_clm, self.packet_creator.packet_view()].spacing(50).into()
                }
                else if self.status.serial_state == AppState::ERROR{
                    let serial_text = text("Serial Error!!!!");

                    let controller_clm = column![tit, tex, serial_text].align_items(iced::Alignment::Start);

                    row![controller_clm, self.packet_creator.packet_view()].spacing(50).into()
                }else if self.status.serial_state == AppState::OK{
                    let serial_text = text(format!("Serial Ok:{}", self.input_path));

                    let controller_clm = column![tit, tex, serial_text].align_items(iced::Alignment::Start);

                    row![controller_clm, self.packet_creator.packet_view()].spacing(50).into()
                }
                else
                {
                    text("App State Error").size(300).into()
                }
            }
            else {
                let controller_clm = column![tit, tex].align_items(iced::Alignment::Start);

                row![controller_clm, self.packet_creator.packet_view()].spacing(50).into()
            }
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

        let err_text = utils::setting_state_logger(self.status.controller_state);

        column![title, combo_, btn, err_text,img].align_items(iced::alignment::Alignment::Center).padding(10).spacing(50).into()
    }
}