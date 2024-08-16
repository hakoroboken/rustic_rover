use crate::rr_core::interface::{Packet, SerialMessage, AppState, RRMessage};
use crate::rr_core::thread_connection::ThreadConnector;
use crate::rr_core::utils::ComboBox;

use iced_aw::TabLabel;

pub struct SerialManager
{
    pub conn:ThreadConnector<Packet>,
    pub path_list:Option<ComboBox<String>>,
    pub selected:String,
    pub state:AppState
}

impl SerialManager {
    pub fn view(&self)->iced::Element<'_, RRMessage>
    {
        use iced::widget::{button, column, text, container::Container};
        match &self.path_list {
            Some(get_list)=>{
                use iced::widget::combo_box;
                let combo_yp = combo_box(
                    &get_list.all, 
                    "Select Serial Port", 
                    get_list.selected.as_ref(), 
                    SerialMessage::PortSelected);
                
                let start_b = button("Start Serial").width(iced::Length::Shrink).height(iced::Length::Shrink).on_press(SerialMessage::SerialStart);
                let scan_b = button("Scan Port").width(iced::Length::Shrink).height(iced::Length::Shrink).on_press(SerialMessage::SerialScan);

                let container:iced::Element<'_, SerialMessage> = Container::new(
                    column![scan_b, combo_yp, start_b].align_items(iced::alignment::Alignment::Center).padding(10).spacing(50)
                )
                .align_x(iced::alignment::Horizontal::Center)
                .align_y(iced::alignment::Vertical::Center).into();

                container.map(RRMessage::Serial)
            }
            None=>{
                let serial_text = text("Press Button and search serialport").size(30);
                let b = button("Scan Port").width(iced::Length::Shrink).height(iced::Length::Shrink).on_press(SerialMessage::SerialScan);
                
                let container:iced::Element<'_, SerialMessage> = Container::new(
                    column![serial_text, b].align_items(iced::alignment::Alignment::Center).padding(10).spacing(50)
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
        }
    }
    fn title(&self)->String
    {
        String::from("Serial Manager")
    }
    pub fn tab_label(&self)->TabLabel
    {
        TabLabel::IconText('C', self.title())
    }
}

impl SerialManager {
    pub fn new()->SerialManager
    {
        SerialManager { conn: ThreadConnector::<Packet>::new(), path_list : None, selected:String::new(), state:AppState::NoReady}
    }
    pub fn search_port(&mut self)
    {
        match serialport::available_ports()
        {
            Ok(vec)=>{
                let mut path_list_ = Vec::<String>::new();

                for i in 0..vec.len()
                {
                    path_list_.push(vec.get(i).unwrap().port_name.clone())
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
        self.conn.publisher = node.publisher.clone();

        let mut ab = "a";

        std::thread::spawn(move ||{
            let send_packet = node.subscriber.recv().unwrap();

            if ab == "a"
            {
                ab = "b";
            }
            else {
                ab = "a"
            }

            let write_buf = format!("{}{},{},{},{},{}e", ab,
                    send_packet.x/10 as i32+10,
                    send_packet.y/10 as i32+10,
                    send_packet.ro/10 as i32+10,
                    send_packet.m1/10 as i32+10,
                    send_packet.m2/10 as i32+10);

            match port_.write(write_buf.as_bytes()) {
                Ok(_)=>{
        
                }
                Err(_)=>{
        
                }
            }
        });
    }
}