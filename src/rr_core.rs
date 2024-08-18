mod controller_manager;
mod interface;
mod thread_connection;
mod packet_manager;
mod utils;
mod serial_manager;
mod save_data_manager;
mod udp_manager;

use controller_manager::DualShock4DriverManager;
use interface::{AppState,RRMessage, LifeCycle};
use serial_manager::SerialManager;

use iced;
use iced::widget::column;
use iced_aw::Tabs;


pub struct RusticRover
{
    game_controller_manager:controller_manager::DualShock4DriverManager,
    packet_creator:packet_manager::PacketManager,
    serial_state:AppState,
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
            serial_state:AppState::NoReady,
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
                self.packet_creator.sdm.search_data_files();
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
        let home:iced::Element<'_, RRMessage> = column![utils::path_to_image("./rustic_rover.png", 500)].align_items(iced::Alignment::Center).into();
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