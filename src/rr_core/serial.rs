use std::sync::mpsc::Receiver;
use crate::rr_core::interface::Packet;
use crate::rr_core::thread_connection::ThreadConnector;
use crate::rr_core::utils::ComboBox;

pub struct SerialManager
{
    pub conn:ThreadConnector<Packet>,
    pub path_list:Option<ComboBox<String>>,
}

impl SerialManager {
    pub fn new()->SerialManager
    {
        SerialManager { conn: ThreadConnector::<Packet>::new(), path_list : None}
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
}

pub fn serial_task(port_name_:String, packet_subscriber:Receiver<Packet>)
{
    let mut port_ = serialport::new(port_name_.clone().as_str(), 115200)
            .data_bits(serialport::DataBits::Eight)
            .stop_bits(serialport::StopBits::One)
            .timeout(std::time::Duration::from_millis(100))
            .open().unwrap();

    let mut ab = "a";
        loop {
            let p = packet_subscriber.recv().unwrap();

            
            if ab == "a"
            {
                ab = "b";
            }
            else {
                ab = "a"
            }
            let write_buf = format!("{}{},{},{},{},{}e", ab,
                    p.x/10 as i32+10,
                    p.y/10 as i32+10,
                    p.ro/10 as i32+10,
                    p.m1/10 as i32+10,
                    p.m2/10 as i32+10);

            match port_.write(write_buf.as_bytes()) {
                Ok(_)=>{

                }
                Err(_)=>{

                }
            }
        }
}