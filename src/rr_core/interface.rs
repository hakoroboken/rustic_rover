

#[derive(Debug, Clone)]
pub enum RRMessage
{
    ControllerThreadMessage(DualShock4),
    ControllerType(ControllerConnectionType),
    PowerRateX(u16),
    PowerRateY(u16),
    PowerRateRotation(u16),
    PowerRateM1(u16),
    PowerRateM2(u16),
    PacketAssign1p(AssignController),
    PacketAssign1m(AssignController),
    PacketAssign2p(AssignController),
    PacketAssign2m(AssignController),
    PacketAssign3p(AssignController),
    PacketAssign3m(AssignController),
    PacketAssign4p(AssignController),
    PacketAssign4m(AssignController),
    PacketAssign5p(AssignController),
    PacketAssign5m(AssignController),
    ControllerStart,
    SerialPathInput(String),
    SerialStart,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Status
{
    pub controller_state:AppState,
    pub packet_state:AppState,
    pub serial_state:AppState
}
impl Status {
    pub fn new()->Status
    {
        Status { controller_state: AppState::NoReady, packet_state: AppState::NoReady, serial_state: AppState::NoReady }
    }
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