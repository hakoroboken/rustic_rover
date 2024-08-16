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

use iced::{self, Element};
use iced::widget::{button, column, combo_box, text};
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
            life_cycle:LifeCycle::Setting,
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
        if self.game_controller_manager.state == AppState::NoReady
        {
            self.title_view()
        }
        else {
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
            .push(
                LifeCycle::SerialInfo, 
                self.serial_manager.tab_label(), 
            self.serial_manager.view()
            )
            .set_active_tab(&self.life_cycle)
            .tab_bar_style(iced_aw::style::tab_bar::TabBarStyles::Blue)
            .tab_bar_position(iced_aw::TabBarPosition::Bottom)
            .into();

            tab
        }
    }
}

impl RusticRover {
    fn title_view(&self)->Element<'_, interface::RRMessage, iced::Theme, iced::Renderer>
    {
        let title = text("RusticRover").size(200).horizontal_alignment(iced::alignment::Horizontal::Center);
        let combo_ = combo_box(
            &self.game_controller_manager.controller_connection_types_combo_box.all, 
            "Select Controller Connection Method", 
            self.game_controller_manager.controller_connection_types_combo_box.selected.as_ref(), 
            interface::ControllerMessage::TypeSelect);

        let path = "./rustic_rover.png";

        let img = utils::path_to_image(path, 1000);

        let btn = button("Start").on_press(interface::ControllerMessage::ControllerStart).width(iced::Length::Shrink).height(iced::Length::Shrink);

        let err_text = utils::setting_state_logger(self.game_controller_manager.state);

        use iced::widget::container::Container;

        let container:iced::Element<'_, interface::ControllerMessage> = Container::new(
        column![title, combo_, btn, err_text,img].align_items(iced::alignment::Alignment::Center).padding(10).spacing(50)
        )
        .align_x(iced::alignment::Horizontal::Center)
        .align_y(iced::alignment::Vertical::Center).into();

        container.map(RRMessage::Controller)
    }
}