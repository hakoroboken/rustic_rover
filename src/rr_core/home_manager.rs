use iced_aw::TabLabel;
use iced::widget::text;
use iced::widget::container::Container;
use iced::widget::{column, button};

use crate::rr_core::interface::{RRMessage, Packet, HomeMessage};
use crate::rr_core::utils::path_to_image;
use super::controller_driver::interface::{ControllerConnectionType, ControllerName};

pub struct HomeManager
{
    pub conn_viewer:Vec<ConnectionViewer>,
    pub stop:bool
}

impl HomeManager {
    pub fn new()->HomeManager
    {
        let v = Vec::<ConnectionViewer>::new();
        HomeManager { conn_viewer: v , stop:false}
    }
    pub fn update(&mut self, message:HomeMessage)
    {
        match message {
            HomeMessage::EmergencyStop=>{
                self.stop= true
            }
            HomeMessage::OK=>{
                self.stop = false;
            }
        }
    }
    pub fn view(&self)->iced::Element<'_, RRMessage>
    {
        match self.conn_viewer.len() {
            0=>{
                text("").into()
            }
            1=>{
                let e_btn = button(path_to_image("./image/emergency_stop.png", 300)).on_press(HomeMessage::EmergencyStop);
                let ok_btn = button(path_to_image("./image/ok.png", 300)).on_press(HomeMessage::OK);

                let ro = iced::widget::row![e_btn, ok_btn];

                let cont:iced::Element<'_, HomeMessage> = Container::new(column![self.conn_viewer[0].create_view(0, self.stop), ro]).center_x().center_y().width(iced::Length::Fill).height(iced::Length::Fill).into();

                cont.map(RRMessage::Home)
            }
            2=>{
                let e_btn = button(path_to_image("./image/emergency_stop.png", 300)).on_press(HomeMessage::EmergencyStop);
                let ok_btn = button(path_to_image("./image/ok.png", 300)).on_press(HomeMessage::OK);


                let ro = iced::widget::row![e_btn, ok_btn];

                let cont:iced::Element<'_, HomeMessage> = Container::new(column![
                    self.conn_viewer[0].create_view(0, self.stop), 
                    self.conn_viewer[1].create_view(1, self.stop),
                    ro]).center_x().center_y().width(iced::Length::Fill).height(iced::Length::Fill).into();

                cont.map(RRMessage::Home)
            }
            3=>{
                let e_btn = button(path_to_image("./image/emergency_stop.png", 300)).on_press(HomeMessage::EmergencyStop);
                let ok_btn = button(path_to_image("./image/ok.png", 300)).on_press(HomeMessage::OK);

                let ro = iced::widget::row![e_btn, ok_btn];

                let cont:iced::Element<'_, HomeMessage> = Container::new(column![
                    self.conn_viewer[0].create_view(0, self.stop), 
                    self.conn_viewer[1].create_view(1, self.stop),
                    self.conn_viewer[2].create_view(2, self.stop),
                    ro]).center_x().center_y().width(iced::Length::Fill).height(iced::Length::Fill).into();

                cont.map(RRMessage::Home)
            }
            _=>{
                text("Errrrr").into()
            }
        }
    }
    fn title(&self)->String
    {
        String::from("ホーム")
    }
    pub fn tab_label(&self)->TabLabel
    {
        TabLabel::Text(self.title())
    }
    pub fn add_view(&mut self)
    {
        self.conn_viewer.push(ConnectionViewer::new())
    }
}

pub struct ConnectionViewer
{
    controller_name: ControllerName,
    controller_connection_type: ExternalType,
    packet: Option<Packet>,
    external_path:String,
    external_type:ExternalType
}

impl ConnectionViewer {
    pub fn new()->ConnectionViewer
    {
        ConnectionViewer {controller_name:ControllerName::DualShock4, controller_connection_type: ExternalType::None, packet: None, external_path: String::new() , external_type:ExternalType::None}
    }
    pub fn create_view(&self, controller_number:usize, stop:bool)->iced::Element<'_, HomeMessage>
    {
        let set_rgb = match controller_number {
            0=>{
                iced::Color::from_rgb8(0, 0, 255)
            },
            1=>{
                iced::Color::from_rgb8(0, 255, 0)
            }
            2=>{
                iced::Color::from_rgb8(255, 0, 0)
            }
            _=>{
                iced::Color::from_rgb8(255, 0, 0)
            }
        };

        let controller_connection: iced::Element<'_, HomeMessage> = if self.controller_connection_type == ExternalType::BLE
        {
            let te = text(self.controller_name).style(set_rgb).size(40);
            let controller = path_to_image("./image/controller.png", 200);
            let line = path_to_image("./image/wireless.png", 100);

            iced::widget::row![te,controller, line].align_items(iced::Alignment::Center).into()
        }
        else if self.controller_connection_type == ExternalType::Serial
        {
            let te = text(self.controller_name).style(set_rgb).size(40);
            let controller = path_to_image("./image/controller.png", 200);
            let line = path_to_image("./image/wired.png", 100);

            iced::widget::row![te,controller, line].align_items(iced::Alignment::Center).into()
        }
        else
        {
            let controller = path_to_image("./image/None.png", 100);

            iced::widget::row![controller].align_items(iced::Alignment::Center).into()
        };

        let packet_text = match self.packet {
            Some(p)=>{
                text(p.get_string()).size(40)
            }
            None=>{
                text("").size(40)
            }
        };

        let micro_connection:iced::Element<'_, HomeMessage> = if self.external_type == ExternalType::UDP
        {
            let micon = path_to_image("./image/micon.png", 100);
            let te = text(self.external_path.clone()).size(40);
            let line = is_stop_wireless(stop);

            iced::widget::row![line, micon, te].align_items(iced::Alignment::Center).into()
        }
        else if self.external_type == ExternalType::Serial
        {
            let micon = path_to_image("./image/micon.png", 100);
            let te = text(self.external_path.clone()).size(40);
            let line = is_stop_wired(stop);

            iced::widget::row![line, micon, te].align_items(iced::Alignment::Center).into()
        }
        else
        {
            path_to_image("./image/None.png", 100).into()
        };

        let content = iced::widget::row![controller_connection, packet_text, micro_connection].align_items(iced::Alignment::Center);

        content.into()
    }
    pub fn set_controller_type(&mut self, input:ControllerConnectionType)
    {
        if input == ControllerConnectionType::BLE{
            self.controller_connection_type = ExternalType::BLE
        }
        else if input == ControllerConnectionType::SERIAL
        {
            self.controller_connection_type = ExternalType::Serial
        }
    }
    pub fn set_packet(&mut self, input:Option<Packet>)
    {
        self.packet = input
    }
    pub fn set_external(&mut self, input : String)
    {
        self.external_path = input.clone();

        let check = input.clone().contains("/dev");

        if check
        {
            self.external_type = ExternalType::Serial
        }
        else if input.clone().contains(":"){
            self.external_type = ExternalType::UDP
        }
        else {
            self.external_type = ExternalType::None
        }
    }
    pub fn set_controller_name(&mut self, name:ControllerName)
    {
        self.controller_name = name
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum ExternalType {
    Serial,
    UDP,
    None,
    BLE,
}

fn is_stop_wireless(stop:bool)->iced::widget::Image<iced::widget::image::Handle>
{
    if stop
    {
        path_to_image("./image/lock_wireless.png", 100)
    }
    else {
        path_to_image("./image/wireless.png", 100)
    }
}

fn is_stop_wired(stop:bool)->iced::widget::Image<iced::widget::image::Handle>
{
    if stop
    {
        path_to_image("./image/lock_wired.png", 100)
    }
    else {
        path_to_image("./image/wired.png", 100)
    }
}