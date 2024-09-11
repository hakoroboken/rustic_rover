use serialport;

use super::interface::Packet;

pub struct SerialDriver
{
    is_im920:bool,
    enable_smoother:bool,
    smooth_gain:f32,
    pub state:bool,
    pub path:String,
    port:Box<dyn serialport::SerialPort>,
    send_packet : Packet,
    prev_packet : Packet
}

impl SerialDriver {
    pub fn new(is_im920_:bool, enable_smother_:bool, port_name:String, smooth_gain:f32)->Self
    {
        let port_ = serialport::new(port_name.as_str(), 115200)
        .timeout(std::time::Duration::from_millis(100))
        .open().unwrap();

        let send_ = Packet::new(0, 100.0, 100.0, 100.0, 100.0, 100.0);
        let prev_ = Packet::new(0, 100.0, 100.0, 100.0, 100.0, 100.0);


        Self { 
            is_im920: is_im920_, 
            enable_smoother: enable_smother_, 
            smooth_gain : smooth_gain,
            path: port_name, 
            port:port_ , 
            state: true,
            send_packet : send_,
            prev_packet: prev_
        }
    }

    pub fn task(&mut self, target:Packet)
    {
        if self.is_im920
        {
            if self.enable_smoother
            {
                self.smooth(target);

                let send_str = self.im920_string();

                match self.port.write(send_str.as_bytes()) {
                    Ok(_size)=>{
                        println!("Write:{}", send_str.clone());

                        let _ = self.port.clear(serialport::ClearBuffer::Input);
                    }
                    Err(_e)=>{
                        let _ = self.port.clear(serialport::ClearBuffer::Output);
                        self.state = false;
                    }
                }

                self.prev_packet = self.send_packet;
            }
            else {
                self.send_packet = target;

                let send_str = self.im920_string();

                match self.port.write(send_str.as_bytes()) {
                    Ok(_size)=>{
                        println!("Write:{}", send_str.clone());

                        let _ = self.port.clear(serialport::ClearBuffer::Input);
                    }
                    Err(_e)=>{
                        let _ = self.port.clear(serialport::ClearBuffer::Output);
                        self.state = false;
                    }
                }

                self.prev_packet = self.send_packet;
            }
        }
        else {
            if self.enable_smoother
            {
                self.smooth(target);

                let send_str = self.normal_string();

                match self.port.write(send_str.as_bytes()) {
                    Ok(_size)=>{
                        println!("Write:{}", send_str.clone());

                        let _ = self.port.clear(serialport::ClearBuffer::Input);
                    }
                    Err(_e)=>{
                        let _ = self.port.clear(serialport::ClearBuffer::Output);
                        self.state = false;
                    }
                }

                self.prev_packet = self.send_packet;
            }
            else {
                self.send_packet = target;

                let send_str = self.normal_string();

                match self.port.write(send_str.as_bytes()) {
                    Ok(_size)=>{
                        println!("Write:{}", send_str.clone());

                        let _ = self.port.clear(serialport::ClearBuffer::Input);
                    }
                    Err(_e)=>{
                        let _ = self.port.clear(serialport::ClearBuffer::Output);
                        self.state = false;
                    }
                }

                self.prev_packet = self.send_packet;
            }
        }
    }
    fn smooth(&mut self, target:Packet)
    {
        let vec = Packet{
            id : target.id,
            x: target.x - self.prev_packet.x,
            y: target.y - self.prev_packet.y,
            ro: target.ro - self.prev_packet.ro,
            m1: target.m1 - self.prev_packet.m1,
            m2: target.m2 - self.prev_packet.m2,
            x_smooth : target.x_smooth,
            y_smooth : target.y_smooth,
            ro_smooth : target.ro_smooth,
            m1_smooth : target.m1_smooth,
            m2_smooth : target.m2_smooth
        };

        if vec.x_smooth
        {
            if vec.x > 0.0
            {
                self.send_packet.x += self.smooth_gain
            }
            else if vec.x < 0.0{
                self.send_packet.x -= self.smooth_gain
            }
        }

        if vec.y_smooth
        {
            if vec.y > 0.0
            {
                self.send_packet.y += self.smooth_gain
            }
            else if vec.y < 0.0{
                self.send_packet.y -= self.smooth_gain
            }
        }

        if vec.ro_smooth
        {
            if vec.ro > 0.0
            {
                self.send_packet.ro += self.smooth_gain
            }
            else if vec.ro < 0.0{
                self.send_packet.ro -= self.smooth_gain
            }
        }

        if vec.m1_smooth
        {
            if vec.m1 > 0.0
            {
                self.send_packet.m1 += self.smooth_gain
            }
            else if vec.m1 < 0.0{
                self.send_packet.m1 -= self.smooth_gain
            }
        }

        if vec.m2_smooth
        {
            if vec.m2 > 0.0
            {
                self.send_packet.m2 += self.smooth_gain
            }
            else if vec.m2 < 0.0{
                self.send_packet.m2 -= self.smooth_gain
            }
        }

        self.send_packet.id = target.id;
    }
    fn im920_string(&self)->String
    {
        let content = format!("{},{},{},{},{}", 
            self.send_packet.x as i32 / 10 + 10,
            self.send_packet.y as i32 / 10 + 10,
            self.send_packet.ro as i32 / 10 + 10,
            self.send_packet.m1 as i32 / 10 + 10,
            self.send_packet.m2 as i32 / 10 + 10);

        let id = self.id_to_str(self.send_packet.id);

        format!("TXDU{},{}e", id, content)
    }
    fn normal_string(&self)->String
    {
        let content = format!("{},{},{}", 
            self.send_packet.x as f32 / 100.0,
            self.send_packet.y as f32 / 100.0,
            self.send_packet.ro as f32 / 100.0);

        format!("{}e", content)
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