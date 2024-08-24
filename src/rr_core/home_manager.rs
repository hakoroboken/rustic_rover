use iced_aw::TabLabel;
use iced::widget::{text};

use crate::rr_core::interface::{RRMessage, ControllerConnectionType, Packet};
use crate::rr_core::utils::path_to_image;

pub struct HomeManager
{

}

impl HomeManager {
    pub fn view(&self)->iced::Element<'_, RRMessage>
    {
        
    }
    fn title(&self)->String
    {
        String::from("シリアル設定")
    }
    pub fn tab_label(&self)->TabLabel
    {
        TabLabel::Text(self.title())
    }
}

pub struct ConnectionViewer
{
    controller_connection_type: ControllerConnectionType,
    packet: Packet,
    external_path:String,
    external_type:ExternalType
}

impl ConnectionViewer {
    pub fn new()->ConnectionViewer
    {
        ConnectionViewer { controller_connection_type: ControllerConnectionType::BLE, packet: Packet::new(), external_path: String::new() , external_type:ExternalType::Serial}
    }
    pub fn create_view()
    {
        let controller_image = path_to_image("./image/controller.png", 100);
    }
    pub fn set_controller_type(&mut self, input:ControllerConnectionType)
    {
        self.controller_connection_type = input
    }
    pub fn set_packet(&mut self, input:Packet)
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
        else {
            self.external_type = ExternalType::UDP
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum ExternalType {
    Serial,
    UDP
}