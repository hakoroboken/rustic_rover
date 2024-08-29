use serialport;

use super::interface::Packet;

pub struct SerialDriver
{
    is_im920:bool,
    enable_smoother:bool,
    path:String,
    port:Box<dyn serialport::SerialPort>
}

impl SerialDriver {
    pub fn new(is_im920_:bool, enable_smother_:bool, port_name:String)->Self
    {
        let port_ = serialport::new(port_name.as_str(), 115200)
        .timeout(std::time::Duration::from_millis(100))
        .open().unwrap();


        Self { is_im920: is_im920_, enable_smoother: enable_smother_, path: port_name, port:port_ }
    }

    pub fn task(&mut self, packet:Packet)
    {
        
    }
}