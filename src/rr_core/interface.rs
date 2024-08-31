use crate::rr_core::controller_driver::interface::Controller;

#[derive(Debug, Clone)]
pub enum RRMessage
{
    ControllerThreadMessage(Controller),
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
    ViewPacketID,
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
