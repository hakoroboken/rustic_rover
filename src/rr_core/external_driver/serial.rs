use serialport;

use super::interface::Packet;

pub struct SerialDriver
{
    is_im920:bool,
    enable_smoother:bool,
    state:bool,
    path:String,
    port:Box<dyn serialport::SerialPort>,
    send_packet : Packet,
    prev_packet : Packet
}

impl SerialDriver {
    pub fn new(is_im920_:bool, enable_smother_:bool, port_name:String)->Self
    {
        let port_ = serialport::new(port_name.as_str(), 115200)
        .timeout(std::time::Duration::from_millis(100))
        .open().unwrap();

        let send_ = Packet{x:100, y:100, ro:100, m1:100, m2:100};
        let prev_ = Packet{x:100, y:100, ro:100, m1:100, m2:100};


        Self { 
            is_im920: is_im920_, 
            enable_smoother: enable_smother_, 
            path: port_name, port:port_ , 
            state: true,
            send_packet : send_,
            prev_packet: prev_
        }
    }

    pub fn task(&mut self, id:u16, target:Packet)
    {
        if self.is_im920
        {

        }
        else {
            if self.enable_smoother
            {
                
            }
        }
    }
    fn smooth(&mut self, target:Packet)
    {
        let vec = Packet{
            x: target.x - self.prev_packet.x,
            y: target.y - self.prev_packet.y,
            ro: target.ro - self.prev_packet.ro,
            m1: target.m1 - self.prev_packet.m1,
            m2: target.m2 - self.prev_packet.m2,
        };

        if vec.x > 0
        {
            self.send_packet 
        }
    }
    fn id_to_str(&self, id:u16)->String
    {
        if id < 10
        {
            format!("000{}", id)
        }
        else if id < 100
        {
            format!("00{}", id)
        }
        else if id < 1000
        {
            format!("0{}", id)
        }
        else if id < 10000
        {
            format!("{}", id)
        }
        else
        {
            String::from("ID_ERROR")
        }
    }
}