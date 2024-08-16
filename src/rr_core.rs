mod controller_manager;
mod interface;
mod thread_connection;
mod packet_manager;
mod utils;
mod serial_manager;
mod save_data_manager;

use controller_manager::DualShock4DriverManager;
use interface::{AppState,RRMessage, LifeCycle};
use serial_manager::SerialManager;
use utils::path_to_image;

use iced::{self, Element};
use iced::widget::{button, column, combo_box, row, text};
use iced_aw::Tabs;


pub struct RusticRover
{
    game_controller_manager:controller_manager::DualShock4DriverManager,
    packet_creator:packet_manager::PacketManager,
    serial_state:AppState,
    life_cycle:LifeCycle,
    serial_manager:serial_manager::SerialManager,
    input_path:String,
}

impl iced::Application for RusticRover {
    type Executor = iced::executor::Default;
    type Message = interface::RRMessage;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let app = RusticRover
        {
            game_controller_manager:DualShock4DriverManager::new(),
            packet_creator:packet_manager::PacketManager::new(),
            serial_state:AppState::NoReady,
            life_cycle:LifeCycle::Setting,
            serial_manager:SerialManager::new(),
            input_path:String::new(),
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
            self.game_controller_manager.first_connector.subscriber.take(), 
            move |mut subscriber|async move{
                let get = subscriber.as_mut().unwrap().recv().await.unwrap();

                (interface::RRMessage::ControllerThreadMessage(get), subscriber)
            })
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            interface::RRMessage::ControllerThreadMessage(ds4)=>{
                self.game_controller_manager.get_value[0] = ds4;
                for i in 1..self.game_controller_manager.controller_num
                {
                    self.game_controller_manager.get_value[i] = self.game_controller_manager.connectors[i].subscriber.recv().unwrap();
                }
                
                self.packet_creator.create_packet(ds4);

                match self.packet_creator.packet_ {
                    Some(p)=>{
                        self.packet_creator.state = AppState::OK;
                        
                        if self.serial_state == AppState::OK
                        {
                            self.serial_manager.conn.publisher.send(p).unwrap();
                        }
                    }
                    None=>{
                        self.packet_creator.state = AppState::NoReady;
                    }
                }
            }
            interface::RRMessage::Controller(msg)=>{
                self.game_controller_manager.update(msg)
            }
            interface::RRMessage::Packet(msg)=>{
                self.packet_creator.update(msg)
            }
            interface::RRMessage::Serial(msg)=>{
                self.serial_manager.update(msg)
            }
            interface::RRMessage::Cycle(cycle)=>{
                self.life_cycle = cycle
            }
            interface::RRMessage::TabClosed=>{
                println!("close tab");
            }
        }

        iced::Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        let tab = Tabs::new(RRMessage::Cycle)
            .tab_icon_position(iced_aw::tabs::Position::Bottom)
            .push(
                LifeCycle::ControllerInfo, 
                self.game_controller_manager.tab_label(), 
                self.game_controller_manager.view()
            )
            .push(
                LifeCycle::PacketInfo, 
                self.packet_creator.tab_label(), 
                self.packet_creator.view()
            )
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

        let img = utils::path_to_image(path, 1000);

        let btn = button("Start").on_press(interface::RRMessage::ControllerStart).width(iced::Length::Shrink).height(iced::Length::Shrink);

        let err_text = utils::setting_state_logger(self.controller_state);

        column![title, combo_, btn, err_text,img].align_items(iced::alignment::Alignment::Center).padding(10).spacing(50).into()

    }
    fn home_view(&self)->Element<'_, interface::RRMessage, iced::Theme, iced::Renderer>
    {
        let home_btn = utils::normal_size_button("Home", RRMessage::CycleHome).width(100);
        
        let con_btn = utils::normal_size_button("ControllerInfo", RRMessage::CycleController);
        let con_state = utils::state_to_image(self.controller_state);
        let con_clm = column![con_btn, con_state].align_items(iced::Alignment::Center);

        let serial_btn = utils::normal_size_button("SerialInfo", RRMessage::CycleSerial);
        let serial_state = utils::state_to_image(self.serial_state);
        let serial_clm = column![serial_btn, serial_state].align_items(iced::Alignment::Center);

        let packet_btn = utils::normal_size_button("PacketInfo", RRMessage::CyclePacket);
        let packet_state = utils::state_to_image(self.packet_state);
        let packet_clm = column![packet_btn, packet_state].align_items(iced::Alignment::Center);

        row![home_btn, con_clm, packet_clm, serial_clm].spacing(50).padding(10).align_items(iced::Alignment::End).into()
    }
    fn serial_view(&self)->Element<'_, interface::RRMessage, iced::Theme, iced::Renderer>
    {
        match &self.serial_manager.path_list {
            Some(get_list)=>{
                let combo_yp = combo_box(
                    &get_list.all, 
                    "Select Serial Port", 
                    get_list.selected.as_ref(), 
                    RRMessage::PortList);
                let start_b = utils::normal_size_button("Start Serial", RRMessage::SerialStart);
                let b = utils::normal_size_button("Rescan Serial", RRMessage::SerialSearch);

                column![b, combo_yp, start_b].align_items(iced::alignment::Alignment::Center).padding(10).spacing(50).into()
            }
            None=>{
                let serial_text = text("Press Button and search serialport").size(30);
                let b = utils::normal_size_button("Scan Serial", RRMessage::SerialSearch);
                column![serial_text, b].align_items(iced::alignment::Alignment::Center).padding(10).spacing(50).into()
            }
        }
    }
}