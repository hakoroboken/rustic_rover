use std::sync::mpsc::Receiver;

use crate::rr_core::interface::Packet;
use crate::rr_core::thread_connection::ThreadConnector;

pub struct SerialManager
{
    pub conn:ThreadConnector<Packet>,
}

impl SerialManager {
    pub fn new()->SerialManager
    {
        SerialManager { conn: ThreadConnector::<Packet>::new()}
    }
}

pub fn task(port_name_:String, packet_subscriber:Receiver<Packet>)
{
    let mut port_ = serialport::new(port_name_.clone().as_str(), 115200)
            .data_bits(serialport::DataBits::Eight)
            .stop_bits(serialport::StopBits::One)
            .timeout(std::time::Duration::from_millis(100))
            .open().unwrap();
        loop {
            let p = packet_subscriber.recv().unwrap();
            
            let write_buf = format!("s{},{},{},{},{}e", 
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