use crate::rr_core::interface::{FloatPacket, Packet, SerialMessage, RRMessage};
use crate::rr_core::thread_connection::{ThreadConnector, ThreadManager};
use crate::rr_core::utils::ComboBox;

use iced_aw::TabLabel;

pub struct SerialManager
{
    pub driver_num:usize,
    pub id:Vec<usize>,
    pub id_box:ComboBox<usize>,
    pub is_small_packet:bool,
    pub is_smooth:bool,
    pub conn:Vec<ThreadConnector<Packet>>,
    pub thread_manager:Vec<ThreadManager>,
    pub path_list:Option<ComboBox<String>>,
    pub selected:String,
    pub smooth_value:f32,
    pub state_text:String
}

impl SerialManager {
    pub fn view(&self)->iced::Element<'_, RRMessage>
    {
        use iced::widget::{button, column, text, container::Container};
        match &self.path_list {
            Some(get_list)=>{
                use iced::widget::checkbox;
                let is_sp = checkbox("Small Packet", self.is_small_packet).on_toggle(SerialMessage::SetPacketSize);
                let is_smooth = checkbox("Use Smooth", self.is_smooth).on_toggle(SerialMessage::SetSmooth);

                use iced::widget::combo_box;
                let combo_yp = combo_box(
                    &get_list.all, 
                    "Select Serial Port", 
                    Some(&self.selected), 
                    SerialMessage::PortSelected);
                
                let start_b = button("Start Serial").width(iced::Length::Shrink).height(iced::Length::Shrink).on_press(SerialMessage::SerialStart);
                let scan_b = button("Scan Port").width(iced::Length::Shrink).height(iced::Length::Shrink).on_press(SerialMessage::SerialScan);

                use iced::widget::row;
                let row = row![is_smooth,is_sp, start_b].spacing(30);

                let id_combo_box = combo_box(
                    &self.id_box.all, 
                    "Select id that you want to stop", 
                    self.id_box.selected.as_ref(), 
                    SerialMessage::ThreadID
                );

                let stop = button("Stop Button").width(iced::Length::Shrink).height(iced::Length::Shrink).on_press(SerialMessage::ThreadStop);

                use iced_aw::number_input;
                let number_input = number_input(self.smooth_value, 2.0, SerialMessage::SmoothValue).step(0.1);

                let state_log = text(self.state_text.clone()).size(50);
                let container:iced::Element<'_, SerialMessage> = Container::new(
                    column![scan_b, combo_yp, row, number_input, id_combo_box, stop, state_log].align_items(iced::Alignment::Center).padding(10).spacing(50)
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
                self.selected = name;
                self.state_text = format!("Port path selected:{}\n{}", self.selected.clone(), self.state_text.clone())
            }
            SerialMessage::SerialScan=>{
                self.search_port();
                self.state_text = format!("Search available port.\n{}", self.state_text.clone())
            }
            SerialMessage::SerialStart=>{
                if self.is_smooth
                {
                    self.spawn_smooth_serial(self.smooth_value);
                    self.state_text = format!("Spawned Smooth Serial path:{}\n{}", self.selected.clone(), self.state_text.clone())
                }
                else {
                    self.spawn_serial();
                    self.state_text = format!("Spawned Serial path:{}\n{}", self.selected.clone(), self.state_text.clone())
                }
            }
            SerialMessage::SetPacketSize(changed)=>{
                self.is_small_packet = changed;

                if changed
                {
                    self.state_text = format!("Set packet size: Small\n{}", self.state_text.clone())
                }
                else {
                    self.state_text = format!("Set packet size: Normal\n{}", self.state_text.clone())
                }
            }
            SerialMessage::ThreadID(id)=>{
                self.id_box.selected = Some(id)
            }
            SerialMessage::ThreadStop=>{
                match self.id_box.selected {
                    Some(id_)=>{
                        self.thread_manager[id_].thread_stop();
                        self.conn.remove(id_);
                        self.driver_num -= 1;
                        self.id.remove(id_);

                        self.state_text = format!("Stop thread at ID:{}\n{}", id_, self.state_text.clone())
                    }
                    None=>{
                        self.state_text = format!("Can't stop thread because don't select thread id.\n{}", self.state_text.clone())
                    }
                }
            }
            SerialMessage::SmoothValue(val)=>{
                self.smooth_value = val;
            }
            SerialMessage::SetSmooth(sm)=>{
                self.is_smooth = sm;
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
        SerialManager {
            driver_num:0, 
            is_small_packet:false,
            conn: v, 
            path_list : None, 
            selected:String::new(), 
            thread_manager:manager_vec, 
            id:id_v.clone(), 
            id_box:ComboBox::<usize>::new(id_v.clone()), 
            smooth_value:1.0, 
            is_smooth:false,
            state_text:String::new()
        }
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
    pub fn spawn_smooth_serial(&mut self, smooth_value:f32)
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

            let mut send = FloatPacket{x:10.0, y:10.0, ro:10.0, m1:10.0, m2:10.0};
            let mut history = Packet{x:10, y:10, ro:10, m1:10, m2:10};
            while !clone_.load(std::sync::atomic::Ordering::Relaxed) 
            {
                let target = match node.subscriber.recv()
                {
                    Ok(ok)=>{
                        ok
                    }
                    Err(_e)=>{
                        let p = Packet{x:10, y:10, ro:10, m1:10, m2:10};

                        p
                    }
                };

                let vec = Packet{
                    x: target.x - history.x,
                    y: target.y - history.y,
                    ro: target.ro - history.ro,
                    m1: target.m1 - history.m1,
                    m2: target.m2 - history.m2,
                };

                if vec.x > 0
                {
                    send.x += smooth_value;
                }
                else if vec.x < 0
                {
                    send.x -= smooth_value;
                }

                if vec.y > 0
                {
                    send.y += smooth_value
                }
                else if vec.y < 0
                {
                    send.y -= smooth_value;
                }

                if vec.ro > 0
                {
                    send.ro += smooth_value;
                }
                else if vec.ro < 0
                {
                    send.ro -= smooth_value;
                }

                if vec.m1 > 0
                {
                    send.m1 += smooth_value;
                }
                else if vec.m1 < 0
                {
                    send.m1 -= smooth_value;
                }

                if vec.m2 > 0
                {
                    send.m2 += smooth_value;
                }
                else if vec.m2 < 0
                {
                    send.m2 -= smooth_value;
                }

                let write_buf = if is_
                {
                    format!("s{},{},{},{}e",
                            (send.x/10.0) as i32+10,
                            (send.y/10.0) as i32+10,
                            (send.ro/10.0) as i32+10,
                            (send.m1/10.0) as i32+10)
                }
                else
                {
                    format!("s{},{},{},{},{}e",
                            (send.x/10.0) as i32+10,
                            (send.y/10.0) as i32+10,
                            (send.ro/10.0) as i32+10,
                            (send.m1/10.0) as i32+10,
                            (send.m2/10.0) as i32+10)
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

                history.x = send.x as i32;
                history.y = send.y as i32;
                history.ro = send.ro as i32;
                history.m1 = send.m1 as i32;
                history.m2 = send.m2 as i32;
            }

            drop(port_);
        });

        println!("Stop SmoothSerial path:{}", self.selected.clone());
    }
}