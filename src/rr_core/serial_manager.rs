use crate::rr_core::interface::{Packet, SerialMessage, RRMessage};
use crate::rr_core::thread_connection::ThreadConnector;
use crate::rr_core::utils::ComboBox;

use iced_aw::TabLabel;

pub struct SerialManager
{
    pub driver_num:usize,
    pub is_small_packet:bool,
    pub conn:Vec<ThreadConnector<Packet>>,
    pub path_list:Option<ComboBox<String>>,
    pub selected:String,
}

impl SerialManager {
    pub fn view(&self)->iced::Element<'_, RRMessage>
    {
        use iced::widget::{button, column, text, container::Container};
        match &self.path_list {
            Some(get_list)=>{
                use iced::widget::checkbox;
                let is_sp = checkbox("Small Packet", self.is_small_packet).on_toggle(SerialMessage::SetPacketSize);

                use iced::widget::combo_box;
                let combo_yp = combo_box(
                    &get_list.all, 
                    "Select Serial Port", 
                    Some(&self.selected), 
                    SerialMessage::PortSelected);
                
                let start_b = button("Start Serial").width(iced::Length::Shrink).height(iced::Length::Shrink).on_press(SerialMessage::SerialStart);
                let scan_b = button("Scan Port").width(iced::Length::Shrink).height(iced::Length::Shrink).on_press(SerialMessage::SerialScan);

                use iced::widget::row;
                let row = row![is_sp, start_b];

                let container:iced::Element<'_, SerialMessage> = Container::new(
                    column![scan_b, combo_yp, row].align_items(iced::Alignment::Center).padding(10).spacing(50)
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
                self.selected = name
            }
            SerialMessage::SerialScan=>{
                self.search_port();
            }
            SerialMessage::SerialStart=>{
                self.spawn_serial();
            }
            SerialMessage::SetPacketSize(changed)=>{
                self.is_small_packet = changed
            }
        }
    }
    fn title(&self)->String
    {
        String::from("Serial Manager")
    }
    pub fn tab_label(&self)->TabLabel
    {
        TabLabel::Text(self.title())
    }
}

impl SerialManager {
    pub fn new()->SerialManager
    {
        let mut v = Vec::<ThreadConnector<Packet>>::new();
        v.push(ThreadConnector::<Packet>::new());
        SerialManager {driver_num:0, is_small_packet:false,conn: v, path_list : None, selected:String::new()}
    }
    pub fn search_port(&mut self)
    {
        match serialport::available_ports()
        {
            Ok(vec)=>{
                let mut path_list_ = Vec::<String>::new();

                for i in 0..vec.len()
                {
                    if !vec.get(i).unwrap().port_name.contains("/dev/ttyS")
                    {
                        path_list_.push(vec.get(i).unwrap().port_name.clone())
                    }
                }

                self.path_list = Some(ComboBox::new(path_list_));
            }
            Err(_e)=>{
                self.path_list = None
            }
        }
    }
    pub fn spawn_serial(&mut self)
    {
        let mut port_ = serialport::new(self.selected.clone(), 115200)
            .data_bits(serialport::DataBits::Eight)
            .stop_bits(serialport::StopBits::One)
            .timeout(std::time::Duration::from_millis(100))
            .open().unwrap();

        let node = ThreadConnector::<Packet>::new();
        self.conn[self.driver_num].publisher = node.publisher.clone();

        self.driver_num += 1;
        let is_ = self.is_small_packet.clone();

        let mut ab = "a";

        std::thread::spawn(move ||{
            loop {
                let send_packet = node.subscriber.recv().unwrap();

                if ab == "a"
                {
                    ab = "b";
                }
                else {
                    ab = "a"
                }

                let write_buf = if is_
                {
                    format!("{}{},{},{},{}e", ab,
                            send_packet.x/10 as i32+10,
                            send_packet.y/10 as i32+10,
                            send_packet.ro/10 as i32+10,
                            send_packet.m1/10 as i32+10)
                }
                else
                {
                    format!("{}{},{},{},{},{}e", ab,
                            send_packet.x/10 as i32+10,
                            send_packet.y/10 as i32+10,
                            send_packet.ro/10 as i32+10,
                            send_packet.m1/10 as i32+10,
                            send_packet.m2/10 as i32+10)
                };

                match port_.write(write_buf.as_bytes()) {
                    Ok(_)=>{
            
                    }
                    Err(_)=>{
            
                    }
                }
            }
        });
    }
}