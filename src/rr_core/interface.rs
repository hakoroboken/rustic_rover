

#[derive(Debug, Clone)]
pub enum RRMessage
{
    ControllerThreadMessage(DualShock4),
    Controller(ControllerMessage),
    Packet(PacketMessage),
    Serial(SerialMessage),
    Cycle(LifeCycle),
    Home(HomeMessage),
    TabClosed,
}

#[derive(Debug, Clone)]
pub enum HomeMessage {
    EmergencyStop,
    OK
}

// #[derive(Debug,Clone)]
// pub enum UDPMessage {
//     SpawnUDPDriver
// }

#[derive(Debug, Clone)]
pub enum SerialMessage
{
    SetPacketSize(bool),
    SetSmooth(bool),
    SerialScan,
    SerialStart,
    PortSelected(String),
    ThreadID(usize),
    ThreadStop,
    SmoothValue(i32),
}

#[derive(Debug,Clone)]
pub enum PacketMessage
{
    PacketID(usize),
    FileSelect(String),
    PowerRateX(u16),
    PowerRateY(u16),
    PowerRateRotation(u16),
    PowerRateM1(u16),
    PowerRateM2(u16),
    Assign1p(AssignController),
    Assign1m(AssignController),
    Assign2p(AssignController),
    Assign2m(AssignController),
    Assign3p(AssignController),
    Assign3m(AssignController),
    Assign4p(AssignController),
    Assign4m(AssignController),
    Assign5p(AssignController),
    Assign5m(AssignController),
}

#[derive(Debug,Clone)]
pub enum ControllerMessage
{
    ControllerStart,
}


#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum LifeCycle
{
    Setting,
    Home,
    ControllerInfo,
    PacketInfo,
    SerialInfo,
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AppState
{
    NoReady,
    OK,
    ERROR,
}

/// It is used assign packet value from Game Controller
#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum AssignController {
    JoyLeftX,
    JoyLeftY,
    JoyRightX,
    JoyRightY,
    DPadUp,
    DPadDown,
    DPadLeft,
    DPadRight,
    BtnCircle,
    BtnCross,
    BtnTriangle,
    BtnCube,
    BtnL1,
    BtnR1,
    BtnL2,
    BtnR2,
}
impl std::fmt::Display for AssignController {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AssignController::JoyLeftX=>"Left_Stick_X",
                AssignController::JoyLeftY=>"Left_Stick_Y",
                AssignController::JoyRightX=>"Right_Stick_X",
                AssignController::JoyRightY=>"Right_Stick_Y",
                AssignController::DPadUp=>"Up_Key",
                AssignController::DPadDown=>"Down_Key",
                AssignController::DPadLeft=>"Left_Key",
                AssignController::DPadRight=>"Right_Key",
                AssignController::BtnCircle=>"Circle_Button",
                AssignController::BtnCross=>"Cross_Button",
                AssignController::BtnCube=>"Cube_Button",
                AssignController::BtnTriangle=>"Triangle_Button",
                AssignController::BtnL1=>"L1_Button",
                AssignController::BtnL2=>"L2_Button",
                AssignController::BtnR1=>"R1_Button",
                AssignController::BtnR2=>"R2_Button",
            }
        )
    }
}
impl AssignController {
    pub const ALL:[AssignController;16]=[
        AssignController::JoyLeftX,
        AssignController::JoyLeftY,
        AssignController::JoyRightX,
        AssignController::JoyRightY,
        AssignController::BtnCircle,
        AssignController::BtnCross,
        AssignController::BtnCube,
        AssignController::BtnTriangle,
        AssignController::BtnL1,
        AssignController::BtnL2,
        AssignController::BtnR1,
        AssignController::BtnR2,
        AssignController::DPadUp,
        AssignController::DPadDown,
        AssignController::DPadRight,
        AssignController::DPadLeft
    ];
}

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum ControllerConnectionType
{
    BLE,
    SERIAL
}
impl ControllerConnectionType {
    pub const ALL:[ControllerConnectionType;2]= [ControllerConnectionType::BLE, ControllerConnectionType::SERIAL];
}
impl std::fmt::Display for ControllerConnectionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ControllerConnectionType::BLE=>"Bluetooth",
                ControllerConnectionType::SERIAL=>"Serial"
            }
        )
    }
}

#[derive(Debug,Clone,Copy)]
pub struct RGB
{
    pub red:u8,
    pub blue:u8,
    pub grenn:u8,
}
impl RGB {
    pub fn new()->RGB
    {
        RGB { red: 0, blue: 255, grenn: 0 }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DualShock4
{
    pub mode:ControllerConnectionType,
    pub state:bool,
    pub sticks:JoyStick,
    pub btns:Buttons,
    pub dpad:Dpad
}

impl DualShock4 {
    pub fn new()->DualShock4
    {
        DualShock4 { mode:ControllerConnectionType::BLE,state:true, sticks: JoyStick::new(), btns: Buttons::new(), dpad: Dpad::new() }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct JoyStick
{
    pub left_x:f32,
    pub left_y:f32,
    pub right_x:f32,
    pub right_y:f32,
}
impl JoyStick {
    pub fn new()->JoyStick
    {
        JoyStick { left_x: 0.0, left_y: 0.0, right_x: 0.0, right_y: 0.0 }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Dpad
{
    pub up_key:bool,
    pub down_key:bool,
    pub left_key:bool,
    pub right_key:bool,   
}
impl Dpad {
    pub fn new()->Dpad
    {
        Dpad { up_key: false, down_key: false, left_key: false, right_key: false }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Buttons
{
    pub circle:bool,
    pub cross:bool,
    pub triangle:bool,
    pub cube:bool,
    pub r1:bool,
    pub r2:bool,
    pub l1:bool,
    pub l2:bool,
    pub left_push:bool,
    pub right_push:bool
}
impl Buttons {
    pub fn new()->Buttons
    {
        Buttons { circle: false, cross: false, triangle: false, cube: false, r1: false, r2: false, l1: false, l2: false, left_push: false, right_push: false }
    }
}

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