mod controller_driver;
mod interface;
mod thread_connection;
mod packet_manager;
mod utils;
mod external_driver;
mod save_data_manager;
mod home_manager;


use home_manager::HomeManager;
use interface::{LifeCycle, RRMessage};
use external_driver::ExternalManager; 
use external_driver::interface::Packet; 

use iced;
use iced_aw::Tabs;


pub struct RusticRover
{
    game_controller_manager:controller_driver::ControllerManager,
    packet_creator:packet_manager::PacketManager,
    life_cycle:LifeCycle,
    serial_manager:external_driver::ExternalManager,
    home_manager:home_manager::HomeManager
}

impl iced::Application for RusticRover {
    type Executor = iced::executor::Default;
    type Message = interface::RRMessage;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let app = RusticRover
        {
            game_controller_manager:controller_driver::ControllerManager::new(),
            packet_creator:packet_manager::PacketManager::new(),
            life_cycle:LifeCycle::Home,
            serial_manager:ExternalManager::new(),
            home_manager:HomeManager::new()
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
                self.home_manager.conn_viewer[0].set_controller_type(ds4.mode, self.game_controller_manager.controller_names[0], ds4.state);
                
                self.packet_creator.create_packet(ds4, 0);
                for i in 1..self.game_controller_manager.controller_num
                {
                    self.game_controller_manager.get_value[i] = match self.game_controller_manager.connectors[i].subscriber.recv()
                    {
                        Ok(get)=>{
                            get
                        }
                        Err(_e)=>{
                            controller_driver::interface::Controller::new()
                        }
                    };
                        self.home_manager.conn_viewer[i].set_controller_type(self.game_controller_manager.get_value[i].mode, self.game_controller_manager.controller_names[i], self.game_controller_manager.get_value[i].state);
                    self.packet_creator.create_packet(self.game_controller_manager.get_value[i], i);
                }

                for i in 0..self.packet_creator.packet_num
                {
                    match self.packet_creator.packet_.get(i) {
                        Some(p)=>{
                            self.home_manager.conn_viewer[i].set_packet(*p);
                        }
                        None=>{

                        }
                    }
                }
                
                for i in 0..self.serial_manager.driver_num
                {
                    if self.serial_manager.thread_reporter[i].subscriber.recv().unwrap()
                    {
                        match self.packet_creator.packet_.get(i) {
                            Some(packet)=>{
                                match packet {
                                    Some(p)=>{
                                        if self.home_manager.stop
                                        {
                                            let _ = self.serial_manager.conn[i].publisher.send(Packet{id:0,x:0.0, y:0.0, ro:0.0, m1:0.0, m2:0.0, x_smooth : false, y_smooth : false, ro_smooth : false, m1_smooth : false, m2_smooth : false});
                                        }
                                        else 
                                        {
                                            let _ = self.serial_manager.conn[i].publisher.send(*p);
                                        }
                                    }
                                    None=>{

                                    }
                                }
                            }
                            None=>{

                            }
                        }
                    }
                    else {
                        self.serial_manager.conn.remove(i);
                        self.serial_manager.driver_num -= 1;
                        self.home_manager.conn_viewer[i].set_external("ERROR".to_string());

                        break;
                    }
                }
            }
            interface::RRMessage::Controller(msg)=>{
                self.packet_creator.sdm.search_data_files();
                self.game_controller_manager.update(msg);

                for i in 0..self.game_controller_manager.controller_num
                {
                    if i != 0
                    {
                        self.packet_creator.new_set();
                    }
                    self.home_manager.add_view();
                    
                }
            }
            interface::RRMessage::Packet(msg)=>{
                self.packet_creator.update(msg)
            }
            interface::RRMessage::Serial(msg)=>{
                match msg {
                    external_driver::interface::SerialMessage::SerialStart=>
                    {
                        self.home_manager.conn_viewer[self.serial_manager.driver_num].set_external(self.serial_manager.selected.clone())
                    }
                    _=>{}
                }
                self.serial_manager.update(msg)
            }
            interface::RRMessage::Home(msg)=>{
                self.home_manager.update(msg)
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
            LifeCycle::Home, 
            self.home_manager.tab_label(), 
            self.home_manager.view()
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
        .text_size(30.0)
        .into();

        tab
    }
}