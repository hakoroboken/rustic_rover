mod controller_manager;
mod interface;
mod thread_connection;
mod packet_manager;
mod utils;
mod serial_manager;
mod save_data_manager;
mod udp_manager;

use controller_manager::DualShock4DriverManager;
use interface::{RRMessage, LifeCycle};
use serial_manager::SerialManager;

use iced;
use iced::widget::{column, text};
use iced_aw::Tabs;


pub struct RusticRover
{
    game_controller_manager:controller_manager::DualShock4DriverManager,
    packet_creator:packet_manager::PacketManager,
    life_cycle:LifeCycle,
    serial_manager:serial_manager::SerialManager,
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
            life_cycle:LifeCycle::Home,
            serial_manager:SerialManager::new(),
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
                if self.game_controller_manager.controller_num - self.packet_creator.packet_id.len() > 0
                {
                    for _i in 0..self.game_controller_manager.controller_num - self.packet_creator.packet_id.len()
                    {
                        self.packet_creator.new_set();
                    }
                }
                self.packet_creator.sdm.search_data_files();
                self.game_controller_manager.get_value[0] = ds4;
                for i in 1..self.game_controller_manager.controller_num
                {
                    self.game_controller_manager.get_value[i] = self.game_controller_manager.connectors[i].subscriber.recv().unwrap();
                }

                for i in 0..self.game_controller_manager.controller_num
                {
                    self.packet_creator.create_packet(self.game_controller_manager.get_value[i], i);
                }
                
                for i in 0..self.serial_manager.driver_num
                {
                    match self.packet_creator.packet_.get(i) {
                        Some(packet)=>{
                            match packet {
                                Some(p)=>{
                                    let _ = self.serial_manager.conn[i].publisher.send(*p);
                                }
                                None=>{

                                }
                            }
                        }
                        None=>{

                        }
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
        let con_text = text(format!("{} Controller is connected!!", self.game_controller_manager.controller_num)).size(30);
        let mut p_str = String::new();
        for i in 0..self.packet_creator.packet_id.len()
        {
            match self.packet_creator.packet_.get(i) {
                Some(packet)=>{
                    match packet {
                        Some(p)=>{
                            let str = format!("packet{} : [x:{:3},y:{:3},ro:{:3},m1:{:3},m2:{:3}]\n", i, p.x, p.y, p.ro, p.m1, p.m2);
                            p_str += &str;
                        }
                        None=>{

                        }
                    }
                }
                None=>{
                    
                }
            }
            
        }

        let p_text = text(p_str).size(30);
        let home:iced::Element<'_, RRMessage> = column![utils::path_to_image("./rustic_rover.png", 500), con_text, p_text].align_items(iced::Alignment::Center).into();
        let tab = Tabs::new(RRMessage::Cycle)
        .tab_icon_position(iced_aw::tabs::Position::Bottom)
        .push(
            LifeCycle::Home, 
            iced_aw::TabLabel::Text("Home".to_string()), 
            home
        )
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
        .push(
            LifeCycle::SerialInfo, 
            self.serial_manager.tab_label(), 
        self.serial_manager.view()
        )
        .set_active_tab(&self.life_cycle)
        .tab_bar_style(iced_aw::style::tab_bar::TabBarStyles::Dark)
        .tab_bar_position(iced_aw::TabBarPosition::Top)
        .into();

        tab
    }
}