use crate::core::interface::{Packet, DualShock4};
use crate::core::iced_utils::ComboBox;

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

#[derive(Clone)]
pub struct PlusMinus
{
    pub plus:ComboBox<AssignController>,
    pub minus:ComboBox<AssignController>,
}
impl PlusMinus {
    pub fn new()->PlusMinus
    {
        PlusMinus { plus: ComboBox::new(AssignController::ALL.to_vec()), minus: ComboBox::new(AssignController::ALL.to_vec()) }
    }
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

pub struct PacketCreator
{
    pub x_cb:PlusMinus,
    pub y_cb:PlusMinus,
    pub ro_cb:PlusMinus,
    pub m1_cb:PlusMinus,
    pub m2_cb:PlusMinus,
    pub packet_:Option<Packet>,
    pub x_pow_rate:u16,
    pub y_pow_rate:u16,
    pub ro_pow_rate:u16,
    pub m1_pow_rate:u16,
    pub m2_pow_rate:u16
}

impl PacketCreator {
    pub fn new()->PacketCreator
    {
        let x_cb_ = PlusMinus::new();
        let y_cb_ = PlusMinus::new();
        let ro_cb_ = PlusMinus::new();
        let m1_cb_ = PlusMinus::new();
        let m2_cb_ = PlusMinus::new();

        PacketCreator { x_cb: x_cb_, y_cb: y_cb_, ro_cb: ro_cb_, m1_cb: m1_cb_, m2_cb: m2_cb_ , packet_:None, x_pow_rate:100, y_pow_rate:100, ro_pow_rate:100, m1_pow_rate:100, m2_pow_rate:100}
    }

    pub fn create_packet(&mut self, controller_input:DualShock4)
    {
        match assign_to_controller(self.x_cb.clone(), controller_input)
        {
            Some(x_)=>{
                match assign_to_controller(self.y_cb.clone(), controller_input) {
                    Some(y_)=>{
                        match assign_to_controller(self.ro_cb.clone(), controller_input) {
                            Some(ro_)=>{
                                match assign_to_controller(self.m1_cb.clone(), controller_input) {
                                    Some(m1_)=>{
                                        match assign_to_controller(self.m2_cb.clone(), controller_input) {
                                            Some(m2_)=>{
                                                self.packet_ = Some(Packet {
                                                     x: (x_  *self.x_pow_rate as f32) as i32, 
                                                     y: (y_  *self.y_pow_rate as f32) as i32, 
                                                     ro: (ro_  *self.ro_pow_rate as f32) as i32, 
                                                     m1: (m1_  *self.m1_pow_rate as f32) as i32, 
                                                     m2: (m2_  *self.m2_pow_rate as f32) as i32})
                                            }
                                            None=>{
                                                self.packet_ =None
                                            }
                                        }
                                    }
                                    None=>{
                                        self.packet_ =None
                                    }
                                }
                            }
                            None=>{
                                self.packet_ =None
                            }
                        }
                    }
                    None=>{
                        self.packet_ =None
                    }
                }
            }
            None=>{
                self.packet_ =None
            }
        }
    }
}

fn assign_to_controller(cb:PlusMinus, input:DualShock4)->Option<f32>
    {
        match cb.plus.selected
        {
            Some(plus_)=>{
                match plus_ {
                    AssignController::JoyLeftX=>Some(input.sticks.left_x),
                    AssignController::JoyLeftY=>Some(input.sticks.left_y),
                    AssignController::JoyRightX=>Some(input.sticks.right_x),
                    AssignController::JoyRightY=>Some(input.sticks.right_y),
                    _=>{
                        match cb.minus.selected {
                            Some(minus_)=>{
                                if assign_btns(plus_, input)
                                {
                                    Some(1.0)
                                }
                                else if assign_btns(minus_, input)
                                {
                                    Some(-1.0)
                                }
                                else {
                                    Some(0.0)
                                }
                            }
                            None=>{
                                None
                            }
                        }
                    }
                }
            },
            None=>{
                None
            }
        }
    }

fn assign_btns(assign:AssignController, input:DualShock4)->bool
{
    match assign {
        AssignController::BtnCircle=>input.btns.circle,
        AssignController::BtnCross=>input.btns.cross,
        AssignController::BtnCube=>input.btns.cube,
        AssignController::BtnTriangle=>input.btns.triangle,
        AssignController::BtnL1=>input.btns.l1,
        AssignController::BtnL2=>input.btns.l2,
        AssignController::BtnR1=>input.btns.r1,
        AssignController::BtnR2=>input.btns.r2,
        AssignController::DPadUp=>input.dpad.up_key,
        AssignController::DPadDown=>input.dpad.down_key,
        AssignController::DPadLeft=>input.dpad.left_key,
        AssignController::DPadRight=>input.dpad.right_key,
        _=> false
    }
}