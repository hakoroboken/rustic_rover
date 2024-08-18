use crate::rr_core::interface::{Packet, DualShock4, AssignController, PacketMessage, AppState, RRMessage};
use crate::rr_core::utils::ComboBox;

use iced::widget::{text, slider, column, row, combo_box};
use iced_aw::TabLabel;

use crate::rr_core::save_data_manager;

pub struct PacketManager
{
    pub packet_:Vec<Option<Packet>>,
    pub packet_id:Vec<usize>,
    pub packet_id_list:ComboBox<usize>,
    pub x_cb:Vec<PlusMinus>,
    pub y_cb:Vec<PlusMinus>,
    pub ro_cb:Vec<PlusMinus>,
    pub m1_cb:Vec<PlusMinus>,
    pub m2_cb:Vec<PlusMinus>,
    pub x_pow_rate:Vec<u16>,
    pub y_pow_rate:Vec<u16>,
    pub ro_pow_rate:Vec<u16>,
    pub m1_pow_rate:Vec<u16>,
    pub m2_pow_rate:Vec<u16>,
    pub sdm:save_data_manager::SaveDataManager,
    selected_file_name:String,
    pub state:AppState
}

impl PacketManager {
    fn title(&self)->String
    {
        String::from("Packet Manager")
    }
    pub fn tab_label(&self)->TabLabel
    {
        TabLabel::Text(self.title())
    }
    pub fn update(&mut self, message:PacketMessage)
    {
        match message {
            PacketMessage::Assign1p(a1p)=>{
                match self.packet_id_list.selected {
                    Some(id)=>{
                        self.x_cb[id].plus.selected = Some(a1p)
                    }
                    None=>{

                    }
                }                
            }
            PacketMessage::Assign1m(a1m)=>{
                match self.packet_id_list.selected {
                    Some(id)=>{
                        self.x_cb[id].minus.selected = Some(a1m)
                    }
                    None=>{

                    }
                }     
            }
            PacketMessage::Assign2p(a2p)=>{
                match self.packet_id_list.selected {
                    Some(id)=>{
                        self.y_cb[id].plus.selected = Some(a2p)
                    }
                    None=>{

                    }
                }
            }
            PacketMessage::Assign2m(a2m)=>{
                match self.packet_id_list.selected {
                    Some(id)=>{
                        self.y_cb[id].minus.selected = Some(a2m)
                    }
                    None=>{

                    }
                }
            }
            PacketMessage::Assign3p(a3p)=>{
                match self.packet_id_list.selected {
                    Some(id)=>{
                        self.ro_cb[id].plus.selected = Some(a3p)
                    }
                    None=>{

                    }
                }
            }
            PacketMessage::Assign3m(a3m)=>{
                match self.packet_id_list.selected {
                    Some(id)=>{
                        self.ro_cb[id].minus.selected = Some(a3m)
                    }
                    None=>{

                    }
                }
            }
            PacketMessage::Assign4p(a4p)=>{
                match self.packet_id_list.selected {
                    Some(id)=>{
                        self.m1_cb[id].plus.selected = Some(a4p)
                    }
                    None=>{

                    }
                }
            }
            PacketMessage::Assign4m(a4m)=>{
                match self.packet_id_list.selected {
                    Some(id)=>{
                        self.m1_cb[id].minus.selected = Some(a4m)
                    }
                    None=>{

                    }
                }
            }
            PacketMessage::Assign5p(a5p)=>{
                match self.packet_id_list.selected {
                    Some(id)=>{
                        self.m2_cb[id].plus.selected = Some(a5p)
                    }
                    None=>{

                    }
                }
            }
            PacketMessage::Assign5m(a5m)=>{
                match self.packet_id_list.selected {
                    Some(id)=>{
                        self.m2_cb[id].minus.selected = Some(a5m)
                    }
                    None=>{

                    }
                }
            },
            PacketMessage::PowerRateX(x)=>{
                match self.packet_id_list.selected {
                    Some(id)=>{
                        self.x_pow_rate[id] = x
                    }
                    None=>{

                    }
                }
            }
            PacketMessage::PowerRateY(y)=>{
                match self.packet_id_list.selected {
                    Some(id)=>{
                        self.y_pow_rate[id] = y
                    }
                    None=>{

                    }
                }
            }
            PacketMessage::PowerRateRotation(rotation)=>{
                match self.packet_id_list.selected {
                    Some(id)=>{
                        self.ro_pow_rate[id] = rotation
                    }
                    None=>{

                    }
                }
            }
            PacketMessage::PowerRateM1(m1)=>{
                match self.packet_id_list.selected {
                    Some(id)=>{
                        self.m1_pow_rate[id] = m1
                    }
                    None=>{

                    }
                }
            }
            PacketMessage::PowerRateM2(m2)=>{
                match self.packet_id_list.selected {
                    Some(id)=>{
                        self.m2_pow_rate[id] = m2
                    }
                    None=>{

                    }
                }
            }
            PacketMessage::FileSelect(name)=>{
                self.selected_file_name = name;

                self.sdm.load_from_file(self.selected_file_name.clone());

                match self.packet_id_list.selected
                {
                    Some(id)=>{
                        self.x_cb[id].plus.selected = self.sdm.xp_assign;
                        self.x_cb[id].minus.selected = self.sdm.xm_assign;
                        self.x_pow_rate[id] = self.sdm.x_rate.unwrap();
                        self.y_cb[id].plus.selected = self.sdm.yp_assign;
                        self.y_cb[id].minus.selected = self.sdm.ym_assign;
                        self.y_pow_rate[id] = self.sdm.y_rate.unwrap();
                        self.ro_cb[id].plus.selected = self.sdm.rop_assign;
                        self.ro_cb[id].minus.selected = self.sdm.rom_assign;
                        self.ro_pow_rate[id] = self.sdm.ro_rate.unwrap();
                        self.m1_cb[id].plus.selected = self.sdm.m1p_assign;
                        self.m1_cb[id].minus.selected = self.sdm.m1m_assign;
                        self.m1_pow_rate[id] = self.sdm.m1_rate.unwrap();
                        self.m2_cb[id].plus.selected = self.sdm.m2p_assign;
                        self.m2_cb[id].minus.selected = self.sdm.m2m_assign;
                        self.m2_pow_rate[id] = self.sdm.m2_rate.unwrap();
                    }
                    None=>{

                    }
                }
            }
            PacketMessage::PacketID(selected_)=>{
                self.packet_id_list.selected = Some(selected_)
            }
        }
    }
    pub fn view(&self)->iced::Element<'_, RRMessage>
    {
        let x_text = text(format!("Select X (Rate : {})", self.x_pow_rate)).size(30);
        let x_sc = slider(
            0..=100, 
            self.x_pow_rate, 
            PacketMessage::PowerRateX).width(500);
        let x_title = row![x_text, x_sc];
        let combo_xp = combo_box(
            &self.x_cb.plus.all, 
            "Selecct assign of x plus value", 
            self.x_cb.plus.selected.as_ref(), 
            PacketMessage::Assign1p);
        let combo_xm = combo_box(
            &self.x_cb.minus.all, 
            "Selecct assign of x minus value", 
            self.x_cb.minus.selected.as_ref(), 
            PacketMessage::Assign1m);
        let row_x = row![combo_xp, combo_xm].spacing(30);

        let y_text = text(format!("Select Y (Rate : {})", self.y_pow_rate)).size(30);
        let y_sc = slider(
            0..=100, 
            self.y_pow_rate, 
            PacketMessage::PowerRateY).width(500);
        let y_title = row![y_text, y_sc];
        let combo_yp = combo_box(
            &self.y_cb.plus.all, 
            "Selecct assign of y plus value", 
            self.y_cb.plus.selected.as_ref(), 
            PacketMessage::Assign2p);
        let combo_ym = combo_box(
            &self.y_cb.minus.all, 
            "Selecct assign of y minus value", 
            self.y_cb.minus.selected.as_ref(), 
            PacketMessage::Assign2m);
        let row_y = row![combo_yp, combo_ym].spacing(30);

        let ro_text = text(format!("Select Rotation (Rate : {})", self.ro_pow_rate)).size(30);
        let ro_sc = slider(
            0..=100, 
            self.ro_pow_rate, 
            PacketMessage::PowerRateRotation).width(500);
        let ro_title = row![ro_text, ro_sc];

        let combo_rop = combo_box(
            &self.ro_cb.plus.all, 
            "Selecct assign of rotation plus value", 
            self.ro_cb.plus.selected.as_ref(), 
            PacketMessage::Assign3p);
        let combo_rom = combo_box(
            &self.ro_cb.minus.all, 
            "Selecct assign of rotation minus value", 
            self.ro_cb.minus.selected.as_ref(), 
            PacketMessage::Assign3m);
        let row_ro = row![combo_rop, combo_rom].spacing(30);

        let m1_text = text(format!("Select Machine1 (Rate : {})", self.m1_pow_rate)).size(30);
        let m1_sc = slider(
            0..=100, 
            self.m1_pow_rate, 
            PacketMessage::PowerRateM1).width(500);
        let m1_title = row![m1_text, m1_sc];
        let combo_m1p = combo_box(
            &self.m1_cb.plus.all, 
            "Selecct assign of machine1 plus value", 
            self.m1_cb.plus.selected.as_ref(), 
            PacketMessage::Assign4p);
        let combo_m1m = combo_box(
            &self.m1_cb.minus.all, 
            "Selecct assign of machine1 minus value", 
            self.m1_cb.minus.selected.as_ref(), 
            PacketMessage::Assign4m);
        let row_m1 = row![combo_m1p, combo_m1m].spacing(30);

        let m2_text = text(format!("Select Machine2 (Rate : {})", self.m2_pow_rate)).size(30);
        let m2_sc = slider(
            0..=100, 
            self.m2_pow_rate, 
            PacketMessage::PowerRateM2).width(500);
        let m2_title = row![m2_text, m2_sc];
        let combo_m2p = combo_box(
            &self.m2_cb.plus.all, 
            "Selecct assign of machine2 plus value", 
            self.m2_cb.plus.selected.as_ref(), 
            PacketMessage::Assign5p);
        let combo_m2m = combo_box(
            &self.m2_cb.minus.all, 
            "Selecct assign of machine2 minus value", 
            self.m2_cb.minus.selected.as_ref(), 
            PacketMessage::Assign5m);
        let row_m2 = row![combo_m2p, combo_m2m].spacing(30);

        let p_text = match self.packet_ {
            Some(p)=>{
                text(format!("[x:{:3},y:{:3},ro:{:3},m1:{:3},m2:{:3}]", p.x, p.y, p.ro, p.m1, p.m2)).size(50)
            }
            None=>{
                text("Failed to Create Packet").size(50)
            }
        };

        use iced::widget::container::Container;
        let container:iced::Element<'_, PacketMessage> = Container::new(
            column![
                    x_title,
                    row_x,
                    y_title,
                    row_y,
                    ro_title,
                    row_ro,
                    m1_title,
                    row_m1,
                    m2_title,
                    row_m2,
                    p_text,
                    self.sdm.menu_view(self.selected_file_name.clone())
            ].align_items(iced::Alignment::Center)
        )
        .align_x(iced::alignment::Horizontal::Center)
        .align_y(iced::alignment::Vertical::Center).into();

        container.map(RRMessage::Packet)
    }
}


impl PacketManager {
    pub fn new()->PacketManager
    {
        let x_cb_ = PlusMinus::new();
        let y_cb_ = PlusMinus::new();
        let ro_cb_ = PlusMinus::new();
        let m1_cb_ = PlusMinus::new();
        let m2_cb_ = PlusMinus::new();

        let mut none = Vec::<Option<Packet>>::new();
        none.push(None);

        let mut packet_id_ = Vec::<usize>::new();
        packet_id_.push(1);

        let packet_id_list_ = ComboBox::<usize>::new(packet_id_.clone());

        PacketManager { 
            packet_:none,
            packet_id:packet_id_,
            packet_id_list:packet_id_list_,
            x_cb: x_cb_, 
            y_cb: y_cb_, 
            ro_cb: ro_cb_, 
            m1_cb: m1_cb_, 
            m2_cb: m2_cb_ , 
            x_pow_rate:100, 
            y_pow_rate:100, 
            ro_pow_rate:100, 
            m1_pow_rate:100, 
            m2_pow_rate:100,
            sdm:save_data_manager::SaveDataManager::new(),
            selected_file_name:String::new(),
            state:AppState::NoReady,
        }
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