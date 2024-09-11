#[derive(Debug, Clone)]
pub enum SerialMessage
{
    SetIM920(bool),
    SetSmooth(bool),
    SerialScan,
    SerialStart,
    PortSelected(String),
    SmoothValue(f32),
}

#[derive(Debug, Clone, Copy)]
pub struct Packet
{
    pub id:u16,
    pub x:f32,
    pub y:f32,
    pub ro:f32,
    pub m1:f32,
    pub m2:f32,
    pub x_smooth : bool,
    pub y_smooth : bool,
    pub ro_smooth : bool,
    pub m1_smooth : bool,
    pub m2_smooth : bool
}

impl Packet {
    pub fn new(id_:u16, x_:f32, y_:f32, ro_:f32, m1_:f32, m2_:f32)->Packet
    {
        Packet { id: id_, x: x_, y: y_, ro: ro_, m1: m1_, m2: m2_ , x_smooth : false, y_smooth : false, ro_smooth : false, m1_smooth : false, m2_smooth : false}
    }
    pub fn get_string(&self)->String
    {
        format!("ID:{}   [x:{:3.0},y:{:3.0},ro:{:3.0},m1:{:3.0},m2:{:3.0}]", self.id,self.x, self.y, self.ro, self.m1, self.m2)
    }
}