pub mod serial;
pub mod udp;
pub mod interface;

use interface::{Packet, SerialMessage};
use crate::rr_core::interface::RRMessage;
use crate::rr_core::thread_connection::ThreadConnector;
use crate::rr_core::utils::{ComboBox, LogManager};

use iced_aw::TabLabel;

use super::thread_connection;

pub struct ExternalManager
{
    pub driver_num:usize,
    pub is_im920:bool,
    pub is_smooth:bool,
    pub conn:Vec<ThreadConnector<Packet>>,
    pub path_list:Option<ComboBox<String>>,
    pub port_list:Vec<String>,
    pub selected:String,
    pub smooth_value:i32,
    pub logger:LogManager
}

impl ExternalManager {
    pub fn view(&self)->iced::Element<'_, RRMessage>
    {
        use iced::widget::{button, column, text, container::Container};
        match &self.path_list {
            Some(get_list)=>{
                let p_config_text = text("Packet Config").size(80);
                use iced::widget::checkbox;
                use iced_aw::number_input;
                
                let is_sp = checkbox("Use IM920", self.is_im920).on_toggle(SerialMessage::SetIM920);
                let is_smooth = checkbox("Use Smooth", self.is_smooth).on_toggle(SerialMessage::SetSmooth);

                let sm_gain_item = if self.is_smooth
                {
                    Some(number_input(self.smooth_value, 20, SerialMessage::SmoothValue).step(1))
                }
                else
                {
                    None
                };

                let packet_config_clm = match sm_gain_item {
                    Some(sm_gain)=>{
                        iced::widget::column![p_config_text, is_sp, is_smooth, sm_gain].spacing(30)
                    }
                    None=>{
                        iced::widget::column![p_config_text, is_sp, is_smooth].spacing(30)
                    }
                };
                


                let port_config_text = text("Port Config").size(80);
                use iced::widget::combo_box;
                let combo_yp = combo_box(
                    &get_list.all, 
                    "Select Serial Port", 
                    Some(&self.selected), 
                    SerialMessage::PortSelected);
                
                let start_b = button("Start Serial").width(iced::Length::Shrink).height(iced::Length::Shrink).on_press(SerialMessage::SerialStart);
                let scan_b = button("Scan Port").width(iced::Length::Shrink).height(iced::Length::Shrink).on_press(SerialMessage::SerialScan);

                let port_config_clm = iced::widget::column![port_config_text, scan_b, combo_yp, start_b].spacing(30);

                use iced::widget::row;
                let above_row = row![packet_config_clm, port_config_clm].spacing(400);

                let state_log = self.logger.view().size(50);
                let container:iced::Element<'_, SerialMessage> = Container::new(
                    column![above_row, state_log].align_items(iced::Alignment::Center).padding(10).spacing(50)
                )
                .align_x(iced::alignment::Horizontal::Center)
                .align_y(iced::alignment::Vertical::Center).into();

                container.map(RRMessage::Serial)
            }
            None=>{
                let serial_text = text("Press Button and search serialport").size(100);
                let b = button("Scan Port").width(iced::Length::Shrink).height(iced::Length::Shrink).on_press(SerialMessage::SerialScan);
                
                let container:iced::Element<'_, SerialMessage> = Container::new(
                    column![serial_text, b].align_items(iced::Alignment::Center).padding(10).spacing(50)
                )
                .align_x(iced::alignment::Horizontal::Center)
                .align_y(iced::alignment::Vertical::Center).into();

                container.map(RRMessage::Serial)
            }
        }
    }
    pub fn update(&mut self, message:SerialMessage)
    {
        match message {
            SerialMessage::PortSelected(name)=>{
                self.selected = name.clone();
                self.logger.add_str(format!("Port path selected: {}", name));
            }
            SerialMessage::SerialScan=>{
                self.search_port();
                self.logger.add_str(format!("Search available port."));
            }
            SerialMessage::SerialStart=>{
                    self.spawn_serial();
                    self.logger.add_str(format!("Start Serial at {}", self.selected.clone()));
            }
            SerialMessage::SetIM920(changed)=>{
                self.is_im920 = changed;

                if changed
                {
                    self.logger.add_str(format!("Set small packet is enable."));
                }
                else {
                    self.logger.add_str(format!("Set small packet is disable."));
                }
            }
            SerialMessage::SmoothValue(val)=>{
                self.smooth_value = val;

                self.logger.add_str(format!("Set smooth gain : {}", val));
            }
            SerialMessage::SetSmooth(sm)=>{
                self.is_smooth = sm;

                if sm
                {
                    self.logger.add_str(format!("Set smoother to enable"));
                }
                else {
                    self.logger.add_str(format!("Set smoother to disable"));
                }
            }
        }
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

impl ExternalManager {
    pub fn new()->ExternalManager
    {
        let v = Vec::<ThreadConnector<Packet>>::new();
        let p_list = Vec::<String>::new();
        
        ExternalManager {
            driver_num:0, 
            is_im920: false,
            conn: v, 
            path_list : None, 
            selected:String::new(), 
            smooth_value:1, 
            is_smooth:false,
            logger:LogManager::new(),
            port_list: p_list
        }
    }
    pub fn search_port(&mut self)
    {
        match serialport::available_ports()
        {
            Ok(vec)=>{
                
                self.port_list = Vec::<String>::new();

                for i in 0..vec.len()
                {
                    if !vec.get(i).unwrap().port_name.contains("/dev/ttyS")
                    {
                        self.port_list.push(vec.get(i).unwrap().port_name.clone())
                    }
                }

                self.path_list = Some(ComboBox::new(self.port_list.clone()));
            }
            Err(_e)=>{
                self.path_list = None
            }
        }
    }
    pub fn spawn_serial(&mut self)
    {
        self.conn.push(ThreadConnector::<Packet>::new());
        let mut serial_driver = serial::SerialDriver::new(self.is_im920, self.is_smooth, self.selected.clone());
        let selected_index = self.port_list.iter().position(|x| x == &self.selected.clone()).unwrap();

        self.port_list.remove(selected_index);
        self.path_list = Some(ComboBox::new(self.port_list.clone()));

        let node = thread_connection::ThreadConnector::<Packet>::new();

        self.conn[self.driver_num].publisher = node.publisher.clone();

        self.driver_num += 1;

        std::thread::spawn(move ||{
            while serial_driver.state {
                let recv_packet = node.subscriber.recv().unwrap();

                serial_driver.task(recv_packet);
            }
        });

    }
}