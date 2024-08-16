use crate::rr_core::interface::AssignController;
use crate::rr_core::utils::ComboBox;
use std::fs;
use std::io::Read;
use yaml_rust::YamlLoader;
use iced::widget::combo_box;

use super::interface::PacketMessage;

pub struct SaveDataManager
{
    pub xp_assign:Option<AssignController>,
    pub xm_assign:Option<AssignController>,
    pub yp_assign:Option<AssignController>,
    pub ym_assign:Option<AssignController>,
    pub rop_assign:Option<AssignController>,
    pub rom_assign:Option<AssignController>,
    pub m1p_assign:Option<AssignController>,
    pub m1m_assign:Option<AssignController>,
    pub m2p_assign:Option<AssignController>,
    pub m2m_assign:Option<AssignController>,
    pub x_rate:Option<u16>,
    pub y_rate:Option<u16>,
    pub ro_rate:Option<u16>,
    pub m1_rate:Option<u16>,
    pub m2_rate:Option<u16>,
    pub file_list:Option<ComboBox<String>>
}

impl SaveDataManager {
    pub fn new()->SaveDataManager
    {
        SaveDataManager { 
            xp_assign: None, 
            xm_assign: None, 
            yp_assign: None, 
            ym_assign: None, 
            rop_assign: None, 
            rom_assign: None, 
            m1p_assign: None, 
            m1m_assign: None, 
            m2p_assign: None, 
            m2m_assign: None,
            x_rate:None,
            y_rate:None,
            ro_rate:None,
            m1_rate:None,
            m2_rate:None,
            file_list: None }
    }
    pub fn search_data_files(&mut self)
    {
        let mut get_strings = Vec::<String>::new();
        match fs::read_dir("./config")
        {
            Ok(entries)=>{
                for i in entries
                {
                    let file = i.unwrap().file_name();
                    get_strings.push(file.into_string().unwrap());
                }

                self.file_list = Some(ComboBox::<String>::new(get_strings));
            }
            Err(_e)=>{
                self.file_list = None;
            }
        }
    }
    pub fn load_from_file(&mut self, f_name_:String)
    {

        println!("{}", f_name_.clone());
        let file_name = format!("./config/{}", f_name_);
        match fs::File::open(file_name)
        {
            Ok(mut f)=>{
                let mut content = String::new();

                let _ = f.read_to_string(&mut content);

                match YamlLoader::load_from_str(&content)
                {
                    Ok(docs)=>{
                        let doc = &docs[0];

                        self.xp_assign = str_to_assign(doc["/**"]["x"]["plus"].as_str().unwrap());
                        self.xm_assign = str_to_assign(doc["/**"]["x"]["minus"].as_str().unwrap());
                        self.x_rate = Some(doc["/**"]["x"]["rate"].as_i64().unwrap() as u16);
                        self.yp_assign = str_to_assign(doc["/**"]["y"]["plus"].as_str().unwrap());
                        self.ym_assign = str_to_assign(doc["/**"]["y"]["minus"].as_str().unwrap());
                        self.y_rate = Some(doc["/**"]["y"]["rate"].as_i64().unwrap() as u16);
                        self.rop_assign = str_to_assign(doc["/**"]["rotation"]["plus"].as_str().unwrap());
                        self.rom_assign = str_to_assign(doc["/**"]["rotation"]["minus"].as_str().unwrap());
                        self.ro_rate = Some(doc["/**"]["rotation"]["rate"].as_i64().unwrap() as u16);
                        self.m1p_assign = str_to_assign(doc["/**"]["m1"]["plus"].as_str().unwrap());
                        self.m1m_assign = str_to_assign(doc["/**"]["m1"]["minus"].as_str().unwrap());
                        self.m1_rate = Some(doc["/**"]["m1"]["rate"].as_i64().unwrap() as u16);
                        self.m2p_assign = str_to_assign(doc["/**"]["m2"]["plus"].as_str().unwrap());
                        self.m2m_assign = str_to_assign(doc["/**"]["m2"]["minus"].as_str().unwrap());
                        self.m2_rate = Some(doc["/**"]["m2"]["rate"].as_i64().unwrap() as u16);
                    }
                    Err(_e)=>{

                    }
                }
            }
            Err(_e)=>{
                println!("Failed to open config file")
            }
        }
    }
    pub fn menu_view(&self, f_name_:String)->iced::widget::Column<PacketMessage>
    {
        match &self.file_list {
            Some(f_list)=>{
                let combo_xp = combo_box(
                    &f_list.all, 
                    "Select config file", 
                    Some(&f_name_), 
                PacketMessage::FileSelect);

                iced::widget::column![combo_xp]
            }
            None=>{
                let te = iced::widget::text("Failed to create file list");

                iced::widget::column![te]
            }
        }
    }
}


fn str_to_assign(str:&str)->Option<AssignController>
{
    match str {
        "Left_Stick_X"=>Some(AssignController::JoyLeftX),
        "Left_Stick_Y"=>Some(AssignController::JoyLeftY),
        "Right_Stick_X"=>Some(AssignController::JoyRightX),
        "Right_Stick_Y"=>Some(AssignController::JoyRightY),
        "Up_Key"=>Some(AssignController::DPadUp),
        "Down_Key"=>Some(AssignController::DPadDown),
        "Left_Key"=>Some(AssignController::DPadLeft),
        "Right_Key"=>Some(AssignController::DPadRight),
        "Circle_Button"=>Some(AssignController::BtnCircle),
        "Cross_Button"=>Some(AssignController::BtnCross),
        "Cube_Button"=>Some(AssignController::BtnCube),
        "Triangle_Button"=>Some(AssignController::BtnTriangle),
        "L1_Button"=>Some(AssignController::BtnL1),
        "L2_Button"=>Some(AssignController::BtnL2),
        "R1_Button"=>Some(AssignController::BtnR1),
        "R2_Button"=>Some(AssignController::BtnR2),
        _=>None
    }
}