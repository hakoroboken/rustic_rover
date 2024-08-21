use crate::rr_core::interface::{Packet, SerialMessage, RRMessage};
use crate::rr_core::thread_connection::{ThreadConnector, ThreadManager};
use crate::rr_core::utils::ComboBox;

use iced_aw::TabLabel;

pub struct SerialManager
{
    pub driver_num:usize,
    pub id:Vec<usize>,
    pub id_box:ComboBox<usize>,
    pub is_small_packet:bool,
    pub conn:Vec<ThreadConnector<Packet>>,
    pub thread_manager:Vec<ThreadManager>,
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

                let id_combo_box = combo_box(
                    &self.id_box.all, 
                    "Select id that you want to stop", 
                    self.id_box.selected.as_ref(), 
                    SerialMessage::ThreadID
                );

                let stop = button("Stop Button").width(iced::Length::Shrink).height(iced::Length::Shrink).on_press(SerialMessage::ThreadStop);

                let container:iced::Element<'_, SerialMessage> = Container::new(
                    column![scan_b, combo_yp, row, id_combo_box, stop].align_items(iced::Alignment::Center).padding(10).spacing(50)
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
            SerialMessage::ThreadID(id)=>{
                self.id_box.selected = Some(id)
            }
            SerialMessage::ThreadStop=>{
                match self.id_box.selected {
                    Some(id)=>{
                        self.thread_manager[id].thread_stop();
                        self.conn.remove(id);
                        self.driver_num -= 1;
                        self.id.remove(id);
                    }
                    None=>{

                    }
                }
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
        let mut manager_vec = Vec::<ThreadManager>::new();
        manager_vec.push(ThreadManager::new());
        let id_v = Vec::<usize>::new();
        SerialManager {driver_num:0, is_small_packet:false,conn: v, path_list : None, selected:String::new(), thread_manager:manager_vec, id:id_v.clone(), id_box:ComboBox::<usize>::new(id_v.clone())}
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
        let selected_port = self.selected.clone();
        let node = ThreadConnector::<Packet>::new();
        self.conn[self.driver_num].publisher = node.publisher.clone();
        let clone_ = self.thread_manager[self.driver_num].get_clone();
        self.id.push(self.driver_num);
        self.id_box = ComboBox::new(self.id.clone());

        self.driver_num += 1;
        self.thread_manager.push(ThreadManager::new());
        self.conn.push(ThreadConnector::<Packet>::new());
        let is_ = self.is_small_packet.clone();

        std::thread::spawn(move ||{
            let mut port_ = serialport::new(selected_port, 115200)
            .timeout(std::time::Duration::from_millis(1000))
            .open().unwrap();
            while !clone_.load(std::sync::atomic::Ordering::Relaxed) 
            {
                let send_packet = match node.subscriber.recv()
                {
                    Ok(ok)=>{
                        ok
                    }
                    Err(_e)=>{
                        let p = Packet{x:10, y:10, ro:10, m1:10, m2:10};

                        p
                    }
                };

                let write_buf = if is_
                {
                    format!("s{},{},{},{}e",
                            send_packet.x/10 as i32+10,
                            send_packet.y/10 as i32+10,
                            send_packet.ro/10 as i32+10,
                            send_packet.m1/10 as i32+10)
                }
                else
                {
                    format!("s{},{},{},{},{}e",
                            send_packet.x/10 as i32+10,
                            send_packet.y/10 as i32+10,
                            send_packet.ro/10 as i32+10,
                            send_packet.m1/10 as i32+10,
                            send_packet.m2/10 as i32+10)
                };

                match port_.write(write_buf.as_bytes()) {
                    Ok(_)=>{
                        println!("Write:{}", write_buf);
                        let _ = port_.clear(serialport::ClearBuffer::Input);
                    }
                    Err(e)=>{
                        println!("{:?}", e);
                        let _ = port_.clear(serialport::ClearBuffer::Output);
                    }
                }
            }
        });
    }
}