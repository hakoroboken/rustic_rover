#[derive(Debug, Clone, Copy)]
pub struct Packet
{
    pub x:i32,
    pub y:i32,
    pub ro:i32,
    pub m1:i32,
    pub m2:i32,
}

impl Packet {
    pub fn get_string(&self)->String
    {
        format!("[x:{:3},y:{:3},ro:{:3},m1:{:3},m2:{:3}]", self.x, self.y, self.ro, self.m1, self.m2)
    }
}