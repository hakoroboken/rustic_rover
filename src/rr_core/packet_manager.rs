use crate::rr_core::interface::{AssignController, PacketMessage, RRMessage};
use crate::rr_core::utils::{self, ComboBox, LogManager};
use super::controller_driver::interface::Controller;
use super::external_driver::interface::Packet;

use iced::widget::{button, column, combo_box, row, slider, text};
use iced_aw::TabLabel;

use crate::rr_core::save_data_manager;

pub struct PacketManager
{
    pub packet_:Vec<Option<Packet>>,
    pub packet_num:usize,
    pub packet_id:Vec<usize>,
    pub packet_second:Vec<usize>,
    pub view_packet_id: usize,
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
    pub x_smooth:Vec<bool>,
    pub y_smooth:Vec<bool>,
    pub ro_smooth:Vec<bool>,
    pub m1_smooth:Vec<bool>,
    pub m2_smooth:Vec<bool>,
    pub sdm:save_data_manager::SaveDataManager,
    selected_file_name:String,
    pub logger:LogManager
}

impl PacketManager {
    fn title(&self)->String
    {
        String::from("パケット設定")
    }
    pub fn tab_label(&self)->TabLabel
    {
        TabLabel::Text(self.title())
    }
    pub fn update(&mut self, message:PacketMessage)
    {
        match message {
            PacketMessage::Assign1p(a1p)=>{
                        self.x_cb[self.view_packet_id].plus.selected = Some(a1p);

                        self.logger.add_str(format!("Set Assign of X_Plus to {}", a1p))                
            }
            PacketMessage::Assign1m(a1m)=>{
                        self.x_cb[self.view_packet_id].minus.selected = Some(a1m);

                        self.logger.add_str(format!("Set Assign of X_Minus to {}", a1m))
            }
            PacketMessage::Assign2p(a2p)=>{
                        self.y_cb[self.view_packet_id].plus.selected = Some(a2p);

                        self.logger.add_str(format!("Set Assign of Y_Plus to {}", a2p))
            }
            PacketMessage::Assign2m(a2m)=>{
                        self.y_cb[self.view_packet_id].minus.selected = Some(a2m);

                        self.logger.add_str(format!("Set Assign of Y_Minus to {}", a2m))
            }
            PacketMessage::Assign3p(a3p)=>{
                        self.ro_cb[self.view_packet_id].plus.selected = Some(a3p);

                        self.logger.add_str(format!("Set Assign of Rotation_Plus to {}", a3p))
            }
            PacketMessage::Assign3m(a3m)=>{
                        self.ro_cb[self.view_packet_id].minus.selected = Some(a3m);

                        self.logger.add_str(format!("Set Assign of Rotation_Minus to {}", a3m))
            }
            PacketMessage::Assign4p(a4p)=>{
                        self.m1_cb[self.view_packet_id].plus.selected = Some(a4p);

                        self.logger.add_str(format!("Set Assign of Machine1_Plus to {}", a4p))
            }
            PacketMessage::Assign4m(a4m)=>{
                        self.m1_cb[self.view_packet_id].minus.selected = Some(a4m);

                        self.logger.add_str(format!("Set Assign of Machine1_Minus to {}", a4m))
            }
            PacketMessage::Assign5p(a5p)=>{
                        self.m2_cb[self.view_packet_id].plus.selected = Some(a5p);

                        self.logger.add_str(format!("Set Assign of Machine2_Plus to {}", a5p))
            }
            PacketMessage::Assign5m(a5m)=>{
                        self.m2_cb[self.view_packet_id].minus.selected = Some(a5m);

                        self.logger.add_str(format!("Set Assign of Machine2_Minus to {}", a5m))
            },
            PacketMessage::PowerRateX(x)=>{
                        self.x_pow_rate[self.view_packet_id] = x;

                        self.logger.add_str(format!("Set Power rate X to {}", x))
            }
            PacketMessage::PowerRateY(y)=>{
                        self.y_pow_rate[self.view_packet_id] = y;

                        self.logger.add_str(format!("Set Power rate Y to {}", y))
            }
            PacketMessage::PowerRateRotation(rotation)=>{
                        self.ro_pow_rate[self.view_packet_id] = rotation;

                        self.logger.add_str(format!("Set Power rate Rotation to {}", rotation))
            }
            PacketMessage::PowerRateM1(m1)=>{
                        self.m1_pow_rate[self.view_packet_id] = m1;

                        self.logger.add_str(format!("Set Power rate Machine1 to {}", m1))
            }
            PacketMessage::PowerRateM2(m2)=>{
                        self.m2_pow_rate[self.view_packet_id] = m2;

                        self.logger.add_str(format!("Set Power rate Machine2 to {}", m2))
            }
            PacketMessage::FileSelect(name)=>{
                self.selected_file_name = name.clone();

                self.sdm.load_from_file(self.selected_file_name.clone());
                        self.x_cb[self.view_packet_id].plus.selected = self.sdm.xp_assign;
                        self.x_cb[self.view_packet_id].minus.selected = self.sdm.xm_assign;
                        self.x_pow_rate[self.view_packet_id] = self.sdm.x_rate.unwrap();
                        self.y_cb[self.view_packet_id].plus.selected = self.sdm.yp_assign;
                        self.y_cb[self.view_packet_id].minus.selected = self.sdm.ym_assign;
                        self.y_pow_rate[self.view_packet_id] = self.sdm.y_rate.unwrap();
                        self.ro_cb[self.view_packet_id].plus.selected = self.sdm.rop_assign;
                        self.ro_cb[self.view_packet_id].minus.selected = self.sdm.rom_assign;
                        self.ro_pow_rate[self.view_packet_id] = self.sdm.ro_rate.unwrap();
                        self.m1_cb[self.view_packet_id].plus.selected = self.sdm.m1p_assign;
                        self.m1_cb[self.view_packet_id].minus.selected = self.sdm.m1m_assign;
                        self.m1_pow_rate[self.view_packet_id] = self.sdm.m1_rate.unwrap();
                        self.m2_cb[self.view_packet_id].plus.selected = self.sdm.m2p_assign;
                        self.m2_cb[self.view_packet_id].minus.selected = self.sdm.m2m_assign;
                        self.m2_pow_rate[self.view_packet_id] = self.sdm.m2_rate.unwrap();

                        self.packet_id[self.view_packet_id] = self.sdm.packet_id.unwrap() as usize;
                        self.packet_second[self.view_packet_id] = self.sdm.second_id.unwrap() as usize;

                        self.x_smooth[self.view_packet_id] = self.sdm.x_smooth.unwrap();
                        self.y_smooth[self.view_packet_id] = self.sdm.y_smooth.unwrap();
                        self.ro_smooth[self.view_packet_id] = self.sdm.ro_smooth.unwrap();
                        self.m1_smooth[self.view_packet_id] = self.sdm.m1_smooth.unwrap();
                        self.m2_smooth[self.view_packet_id] = self.sdm.m2_smooth.unwrap();
                self.logger.add_str(format!("Load YAML file : {}", name.clone()));
            }
            PacketMessage::NextPacket=>{
                let new_id = self.view_packet_id as i8 + 1;
                let max_id = self.packet_num as i8 -1;

                if new_id > max_id
                {
                    self.view_packet_id = 0;
                }
                else {
                    self.view_packet_id = new_id as usize
                }

                self.logger.add_str(format!("Set Packet ID to {}", self.view_packet_id));
            }
            PacketMessage::BackPacket=>{
                let new_id = self.view_packet_id as i8 - 1;

                if new_id < 0
                {
                    self.view_packet_id = self.packet_num -1 ;
                }
                else {
                    self.view_packet_id = new_id as usize
                }

                self.logger.add_str(format!("Set Packet ID to {}", self.view_packet_id));
            }
            PacketMessage::FirstPacketID(id)=>{
                self.packet_id[0] = id;
            }
            PacketMessage::SecondPacketID(id)=>{
                self.packet_id[1] = id;
            }
            PacketMessage::ThirdPacketID(id)=>{
                self.packet_id[2] = id;
            }
            PacketMessage::FirstPacketID2(id)=>{
                self.packet_second[0] = id
            }
            PacketMessage::SecondPacketID2(id)=>{
                self.packet_second[1] = id
            }
            PacketMessage::ThirdPacketID2(id)=>{
                self.packet_second[2] = id
            }
        }
    }
    pub fn view(&self)->iced::Element<'_, RRMessage>
    {
        let next_button = button(utils::path_to_image("./image/next_packet.png", 100)).width(250).height(250).on_press(PacketMessage::NextPacket);
        let back_button = button(utils::path_to_image("./image/back_packet.png", 100)).width(250).height(250).on_press(PacketMessage::BackPacket);

        let packet_button_row = row![back_button, next_button].spacing(100);

                let x_text = text(format!("Select X (Rate : {})", self.x_pow_rate[self.view_packet_id])).size(30);
                let x_sc = slider(
                    0..=100, 
                    self.x_pow_rate[self.view_packet_id], 
                    PacketMessage::PowerRateX).width(500);
                let x_title = row![x_text, x_sc];
                let combo_xp = combo_box(
                    &self.x_cb[self.view_packet_id].plus.all, 
                    "Selecct assign of x plus value", 
                    self.x_cb[self.view_packet_id].plus.selected.as_ref(), 
                    PacketMessage::Assign1p);
                let combo_xm = combo_box(
                    &self.x_cb[self.view_packet_id].minus.all, 
                    "Selecct assign of x minus value", 
                    self.x_cb[self.view_packet_id].minus.selected.as_ref(), 
                    PacketMessage::Assign1m);
                let row_x = row![combo_xp, combo_xm].spacing(30);

                let y_text = text(format!("Select Y (Rate : {})", self.y_pow_rate[self.view_packet_id])).size(30);
                let y_sc = slider(
                    0..=100, 
                    self.y_pow_rate[self.view_packet_id], 
                    PacketMessage::PowerRateY).width(500);
                let y_title = row![y_text, y_sc];
                let combo_yp = combo_box(
                    &self.y_cb[self.view_packet_id].plus.all, 
                    "Selecct assign of y plus value", 
                    self.y_cb[self.view_packet_id].plus.selected.as_ref(), 
                    PacketMessage::Assign2p);
                let combo_ym = combo_box(
                    &self.y_cb[self.view_packet_id].minus.all, 
                    "Selecct assign of y minus value", 
                    self.y_cb[self.view_packet_id].minus.selected.as_ref(), 
                    PacketMessage::Assign2m);
                let row_y = row![combo_yp, combo_ym].spacing(30);

                let ro_text = text(format!("Select Rotation (Rate : {})", self.ro_pow_rate[self.view_packet_id])).size(30);
                let ro_sc = slider(
                    0..=100, 
                    self.ro_pow_rate[self.view_packet_id], 
                    PacketMessage::PowerRateRotation).width(500);
                let ro_title = row![ro_text, ro_sc];

                let combo_rop = combo_box(
                    &self.ro_cb[self.view_packet_id].plus.all, 
                    "Selecct assign of rotation plus value", 
                    self.ro_cb[self.view_packet_id].plus.selected.as_ref(), 
                    PacketMessage::Assign3p);
                let combo_rom = combo_box(
                    &self.ro_cb[self.view_packet_id].minus.all, 
                    "Selecct assign of rotation minus value", 
                    self.ro_cb[self.view_packet_id].minus.selected.as_ref(), 
                    PacketMessage::Assign3m);
                let row_ro = row![combo_rop, combo_rom].spacing(30);

                let m1_text = text(format!("Select Machine1 (Rate : {})", self.m1_pow_rate[self.view_packet_id])).size(30);
                let m1_sc = slider(
                    0..=100, 
                    self.m1_pow_rate[self.view_packet_id], 
                    PacketMessage::PowerRateM1).width(500);
                let m1_title = row![m1_text, m1_sc];
                let combo_m1p = combo_box(
                    &self.m1_cb[self.view_packet_id].plus.all, 
                    "Selecct assign of machine1 plus value", 
                    self.m1_cb[self.view_packet_id].plus.selected.as_ref(), 
                    PacketMessage::Assign4p);
                let combo_m1m = combo_box(
                    &self.m1_cb[self.view_packet_id].minus.all, 
                    "Selecct assign of machine1 minus value", 
                    self.m1_cb[self.view_packet_id].minus.selected.as_ref(), 
                    PacketMessage::Assign4m);
                let row_m1 = row![combo_m1p, combo_m1m].spacing(30);

                let m2_text = text(format!("Select Machine2 (Rate : {})", self.m2_pow_rate[self.view_packet_id])).size(30);
                let m2_sc = slider(
                    0..=100, 
                    self.m2_pow_rate[self.view_packet_id], 
                    PacketMessage::PowerRateM2).width(500);
                let m2_title = row![m2_text, m2_sc];
                let combo_m2p = combo_box(
                    &self.m2_cb[self.view_packet_id].plus.all, 
                    "Selecct assign of machine2 plus value", 
                    self.m2_cb[self.view_packet_id].plus.selected.as_ref(), 
                    PacketMessage::Assign5p);
                let combo_m2m = combo_box(
                    &self.m2_cb[self.view_packet_id].minus.all, 
                    "Selecct assign of machine2 minus value", 
                    self.m2_cb[self.view_packet_id].minus.selected.as_ref(), 
                    PacketMessage::Assign5m);
                let row_m2 = row![combo_m2p, combo_m2m].spacing(30);

                let p_text = match self.packet_[self.view_packet_id] {
                    Some(p)=>{
                        text(p.get_string()).size(50)
                    }
                    None=>{
                        text("Failed to Create Packet").size(50)
                    }
                };

                let sdm_menu = self.sdm.menu_view(self.selected_file_name.clone());
                let sdm_picture = utils::path_to_image("./image/choose_save_data.png", 400).height(40);

                let send_id_list = if self.packet_num == 1
                {
                    let _1 = iced_aw::number_input(self.packet_id[0], 9999, PacketMessage::FirstPacketID).step(1).size(25.0);
                    let _1f = iced_aw::number_input(self.packet_second[0], 9999, PacketMessage::FirstPacketID2).step(1).size(25.0);
                    let clm1 = iced::widget::column![_1, _1f].spacing(10);

                    iced::widget::row![clm1]
                }
                else if self.packet_num == 2
                {
                    let _1 = iced_aw::number_input(self.packet_id[0], 9999, PacketMessage::FirstPacketID).step(1).size(25.0);
                    let _1f = iced_aw::number_input(self.packet_second[0], 9999, PacketMessage::FirstPacketID2).step(1).size(25.0);
                    let clm1 = iced::widget::column![_1, _1f].spacing(10);
                    let _2 = iced_aw::number_input(self.packet_id[1], 9999, PacketMessage::SecondPacketID).step(1).size(25.0);
                    let _2f = iced_aw::number_input(self.packet_second[1], 9999, PacketMessage::SecondPacketID2).step(1).size(25.0);
                    let clm2= iced::widget::column![_2, _2f].spacing(10);

                    iced::widget::row![clm1, clm2].spacing(30)
                }
                else if self.packet_num == 3
                {
                    let _1 = iced_aw::number_input(self.packet_id[0], 9999, PacketMessage::FirstPacketID).step(1).size(25.0);
                    let _1f = iced_aw::number_input(self.packet_second[0], 9999, PacketMessage::FirstPacketID2).step(1).size(25.0);
                    let clm1 = iced::widget::column![_1, _1f].spacing(10);
                    let _2 = iced_aw::number_input(self.packet_id[1], 9999, PacketMessage::SecondPacketID).step(1).size(25.0);
                    let _2f = iced_aw::number_input(self.packet_second[1], 9999, PacketMessage::SecondPacketID2).step(1).size(25.0);
                    let clm2= iced::widget::column![_2, _2f].spacing(10);

                    let _3 = iced_aw::number_input(self.packet_id[2], 9999, PacketMessage::ThirdPacketID).step(1).size(25.0);
                    let _3f = iced_aw::number_input(self.packet_second[2], 9999, PacketMessage::ThirdPacketID2).step(1).size(25.0);
                    let clm3= iced::widget::column![_3, _3f].spacing(10);

                    iced::widget::row![clm1, clm2, clm3].spacing(30)
                }
                else
                {
                    iced::widget::row![text("")]
                };


                let log = self.logger.view().size(50);

                use iced::widget::container::Container;
                let container:iced::Element<'_, PacketMessage> = Container::new(
                    column![
                            packet_button_row,
                            sdm_picture,
                            sdm_menu,
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
                            send_id_list,
                            log
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
        let mut x_cb_ = Vec::<PlusMinus>::new();
        x_cb_.push(PlusMinus::new());
        let mut y_cb_ = Vec::<PlusMinus>::new();
        y_cb_.push(PlusMinus::new());
        let mut ro_cb_ = Vec::<PlusMinus>::new();
        ro_cb_.push(PlusMinus::new());
        let mut m1_cb_ = Vec::<PlusMinus>::new();
        m1_cb_.push(PlusMinus::new());
        let mut m2_cb_ = Vec::<PlusMinus>::new();
        m2_cb_.push(PlusMinus::new());

        let mut x_rate = Vec::<u16>::new();
        x_rate.push(100);
        let mut y_rate = Vec::<u16>::new();
        y_rate.push(100);
        let mut ro_rate = Vec::<u16>::new();
        ro_rate.push(100);
        let mut m1_rate = Vec::<u16>::new();
        m1_rate.push(100);
        let mut m2_rate = Vec::<u16>::new();
        m2_rate.push(100);

        let mut none = Vec::<Option<Packet>>::new();
        none.push(None);

        let mut packet_id_ = Vec::<usize>::new();
        packet_id_.push(0);

        let mut packet_second_id_ = Vec::<usize>::new();
        packet_second_id_.push(1);

        let mut x_sm = Vec::<bool>::new();
        x_sm.push(false);
        let mut y_sm = Vec::<bool>::new();
        y_sm.push(false);
        let mut ro_sm = Vec::<bool>::new();
        ro_sm.push(false);
        let mut m1_sm = Vec::<bool>::new();
        m1_sm.push(false);
        let mut m2_sm = Vec::<bool>::new();
        m2_sm.push(false);

        PacketManager { 
            packet_:none,
            packet_num:1,
            packet_id:packet_id_,
            packet_second:packet_second_id_,
            view_packet_id: 0,
            x_cb: x_cb_, 
            y_cb: y_cb_, 
            ro_cb: ro_cb_, 
            m1_cb: m1_cb_, 
            m2_cb: m2_cb_ , 
            x_pow_rate:x_rate, 
            y_pow_rate:y_rate, 
            ro_pow_rate:ro_rate, 
            m1_pow_rate:m1_rate, 
            m2_pow_rate:m2_rate,
            sdm:save_data_manager::SaveDataManager::new(),
            selected_file_name:String::new(),
            logger:LogManager::new(),
            x_smooth : x_sm,
            y_smooth : y_sm,
            ro_smooth : ro_sm,
            m1_smooth : m1_sm,
            m2_smooth : m2_sm
        }
    }

    pub fn create_packet(&mut self, controller_input:Controller, id:usize)
    {
        let use_id = if !controller_input.option
        {
            self.packet_id[id]
        }
        else
        {
            self.packet_second[id]
        };
                match assign_to_controller(self.x_cb[id].clone(), controller_input)
                {
                    Some(x_)=>{
                        match assign_to_controller(self.y_cb[id].clone(), controller_input) {
                            Some(y_)=>{
                                match assign_to_controller(self.ro_cb[id].clone(), controller_input) {
                                    Some(ro_)=>{
                                        match assign_to_controller(self.m1_cb[id].clone(), controller_input) {
                                            Some(m1_)=>{
                                                match assign_to_controller(self.m2_cb[id].clone(), controller_input) {
                                                    Some(m2_)=>{
                                                        self.packet_[id] = Some(Packet {
                                                            id : use_id as u16,
                                                            x: (x_  *self.x_pow_rate[id] as f32), 
                                                            y: (y_  *self.y_pow_rate[id] as f32), 
                                                            ro: (ro_  *self.ro_pow_rate[id] as f32), 
                                                            m1: (m1_  *self.m1_pow_rate[id] as f32), 
                                                            m2: (m2_  *self.m2_pow_rate[id] as f32),
                                                            x_smooth : self.x_smooth[id],
                                                            y_smooth : self.y_smooth[id],
                                                            ro_smooth : self.ro_smooth[id],
                                                            m1_smooth : self.m1_smooth[id],
                                                            m2_smooth : self.m2_smooth[id]});

                                                    }
                                                    None=>{
                                                        self.packet_[id] =None
                                                    }
                                                }
                                            }
                                            None=>{
                                                self.packet_[id] =None
                                            }
                                        }
                                    }
                                    None=>{
                                        self.packet_[id] =None
                                    }
                                }
                            }
                            None=>{
                                self.packet_[id] =None
                            }
                        }
                    }
                    None=>{
                        self.packet_[id] =None
                    }
        }
    }

    pub fn new_set(&mut self)
    {
        self.x_cb.push(PlusMinus::new());
        self.y_cb.push(PlusMinus::new());
        self.ro_cb.push(PlusMinus::new());
        self.m1_cb.push(PlusMinus::new());
        self.m2_cb.push(PlusMinus::new());
        self.x_pow_rate.push(100);
        self.y_pow_rate.push(100);
        self.ro_pow_rate.push(100);
        self.m1_pow_rate.push(100);
        self.m2_pow_rate.push(100);

        self.packet_.push(None);
        self.packet_id.push(0);
        self.packet_second.push(1);

        self.x_smooth.push(false);
        self.y_smooth.push(false);
        self.ro_smooth.push(false);
        self.m1_smooth.push(false);
        self.m2_smooth.push(false);

        self.packet_num += 1;
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

fn assign_to_controller(cb:PlusMinus, input:Controller)->Option<f32>
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

fn assign_btns(assign:AssignController, input:Controller)->bool
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