pub mod interface;
pub mod dualsense;
pub mod dualshock4;

extern crate hidapi;
use hidapi::{HidApi, DeviceInfo};

use interface::{Controller, ControllerConnectionType, ControllerName, RGB};
use crate::rr_core::interface::{RRMessage,AppState,ControllerMessage};
use crate::rr_core::{utils, thread_connection};

use iced_aw::TabLabel;
use iced::widget::{row, column, button};
use iced::widget::container::Container;


pub struct ControllerManager
{
    pub controller_names:Vec<ControllerName>,
    pub first_connector:thread_connection::AsyncThreadConnector<Controller>,
    pub connectors:Vec<thread_connection::ThreadConnector<Controller>>,
    pub controller_num:usize,
    pub device_list:Vec<DeviceInfo>,
    api:HidApi,
    pub get_value:Vec<Controller>,
    pub state:AppState,
    green_flag:bool
}

impl ControllerManager {
    fn title(&self)->String
    {
        String::from("コントローラー設定")
    }
    pub fn tab_label(&self)->TabLabel
    {
        TabLabel::Text(self.title())
    }
    pub fn view(&self)->iced::Element<'_, RRMessage>
    {
        
        match self.controller_num {
            0=>{
                let btn = button(utils::path_to_image("./image/start.png", 200)).on_press(ControllerMessage::ControllerStart).width(500).height(500);

                let err_text = utils::setting_state_logger(self.state).size(100).horizontal_alignment(iced::alignment::Horizontal::Center);

                let content:iced::Element<'_, ControllerMessage> = Container::new(
                    column![err_text, btn].align_items(iced::Alignment::Center)
                )
                .align_x(iced::alignment::Horizontal::Center)
                .align_y(iced::alignment::Vertical::Center).into();

                content.map(RRMessage::Controller)
            }
            1=>{
                let con_1 = input_to_controller_view(self.get_value[0]);
                
                let content:iced::Element<'_, ControllerMessage> = Container::new(
                    column![con_1].align_items(iced::Alignment::Center)
                )
                .align_x(iced::alignment::Horizontal::Center)
                .align_y(iced::alignment::Vertical::Center).into();

                content.map(RRMessage::Controller)
            }
            2=>{
                let con_1 = input_to_controller_view(self.get_value[0]);
                let con_2 = input_to_controller_view(self.get_value[1]);

                let content:iced::Element<'_, ControllerMessage> = Container::new(
                    column![con_1, con_2].align_items(iced::Alignment::Center)
                )
                .align_x(iced::alignment::Horizontal::Center)
                .align_y(iced::alignment::Vertical::Center).into();

                content.map(RRMessage::Controller)
            }
            3=>{
                let con_1 = input_to_controller_view(self.get_value[0]);
                let con_2 = input_to_controller_view(self.get_value[1]);
                let con_3 = input_to_controller_view(self.get_value[2]);

                let content:iced::Element<'_, ControllerMessage> = Container::new(
                    column![con_1, con_2, con_3].align_items(iced::Alignment::Center)
                )
                .align_x(iced::alignment::Horizontal::Center)
                .align_y(iced::alignment::Vertical::Center).into();

                content.map(RRMessage::Controller)
            }
            _=>{
                use iced::widget::text;
                text("壊れた！！").size(300).into()
            }
        }
    }
    pub fn update(&mut self, message:ControllerMessage)
    {
        match message {
            ControllerMessage::ControllerStart=>{
                    self.scan_device();
                    if !self.device_list.is_empty()
                    {   
                        self.spawn_driver(ControllerConnectionType::SERIAL);
                        self.get_value.push(Controller::new());
                        self.device_list.remove(0);

                        for i in 0..self.device_list.len() {
                            let new_conn = thread_connection::ThreadConnector::<Controller>::new();
                            self.connectors.push(new_conn);

                            self.add_driver(ControllerConnectionType::SERIAL, self.connectors.get(i+1).unwrap().publisher.clone());
                            self.device_list.remove(0);
                            self.get_value.push(Controller::new());

                            self.state = AppState::OK;
                        }
                    }
                    else {
                        println!("Not found device");
                        self.state = AppState::NoReady;
                    }
            }
        }
    }
}

impl ControllerManager {
    pub fn new()->ControllerManager
    {
        let mut ds4_conn_vec = Vec::<thread_connection::ThreadConnector<Controller>>::new();
        let ds4_conn = thread_connection::AsyncThreadConnector::<Controller>::new();
        let sync_conn = thread_connection::ThreadConnector::<Controller>::new();
        ds4_conn_vec.push(sync_conn);
        let mut in_v = Vec::<Controller>::new();
        let ds4 = Controller::new();
        in_v.push(ds4);
        ControllerManager {
            controller_names: Vec::<ControllerName>::new(),
            first_connector:ds4_conn, 
            connectors:ds4_conn_vec,
            controller_num:0, 
            device_list: Vec::<DeviceInfo>::new(), 
            api: HidApi::new().unwrap() ,
            get_value:in_v,
            state:AppState::NoReady,
            green_flag:false
        }
    }

    pub fn scan_device(&mut self)
    {
        self.api = HidApi::new().unwrap();
        let mut dev_vec = Vec::<DeviceInfo>::new();
        for i in self.api.device_list()
        {
            if i.vendor_id() == 1356 && i.product_id() == 2508
            {
                let s = i.clone();
                dev_vec.push(s);
                self.controller_names.push(ControllerName::DualShock4);
                println!("{}", ControllerName::DualShock4)
            }
            else if i.vendor_id() == 1356 && i.product_id() == 3302
            {
                let s = i.clone();
                dev_vec.push(s);
                self.controller_names.push(ControllerName::DualSense);
                println!("{}", ControllerName::DualSense)
            }
        }

        self.controller_num = dev_vec.clone().len();
        self.device_list = dev_vec;
        println!("{}", self.controller_num)
    }

    pub fn spawn_driver(&mut self, mode_:ControllerConnectionType)
    {
        let publisher_ = self.first_connector.publisher.clone().take().unwrap();
        match self.device_list.first()
        {
            Some(dr)=>{
                match dr.open_device(&self.api) {
                    Ok(device_)=>{
                        match dr.product_id()
                        {
                            2508=>{
                                let mut controller = dualshock4::DualShock4Driver{device:device_, mode:mode_, rgb:RGB::new(), buf:[0_u8;256], result:Controller::new()};

                                controller.rgb = RGB::blue();

                                std::thread::spawn(move ||{
                                    loop {
                                        let get = controller.task();

                                        let _ = publisher_.send(get);

                                        if controller.mode == ControllerConnectionType::SERIAL
                                        {
                                            controller.color_change()
                                        }
                                    }
                                });
                            }
                            3302=>{
                                let mut controller = dualsense::DualSenseDriver{device:device_, mode:mode_, rgb:RGB::new()};

                                controller.rgb = RGB::blue();

                                std::thread::spawn(move ||{
                                    loop {
                                        let get = controller.task();

                                        let _ = publisher_.send(get);
                                    }
                                });
                            }
                            _=>{

                            }
                        }
                    }
                    Err(_e)=>{

                    }
                }
            }
            None=>{

            }
        }
    }

    pub fn add_driver(&mut self, mode_:ControllerConnectionType, publisher_:std::sync::mpsc::Sender<Controller>)
    {
        match self.device_list.first()
        {
            Some(dr)=>{
                match dr.open_device(&self.api) {
                    Ok(device_)=>{
                        match dr.product_id()
                        {
                            2508=>{
                                let mut controller = dualshock4::DualShock4Driver{device:device_, mode:mode_, rgb:RGB::new(), buf:[0_u8;256], result:Controller::new()};

                                if self.green_flag
                                {
                                    controller.rgb = RGB::green()
                                }
                                else 
                                {
                                    controller.rgb = RGB::red()
                                }

                                std::thread::spawn(move ||{
                                    loop {
                                        let get = controller.task();

                                        let _ = publisher_.send(get);

                                        if controller.mode == ControllerConnectionType::SERIAL
                                        {
                                            controller.color_change()
                                        }
                                    }
                                });
                            }
                            3302=>{
                                let mut controller = dualsense::DualSenseDriver{device:device_, mode:ControllerConnectionType::BLE, rgb:RGB::new()};

                                if self.green_flag
                                {
                                    controller.rgb = RGB::green()
                                }
                                else 
                                {
                                    controller.rgb = RGB::red()
                                }

                                std::thread::spawn(move ||{
                                    loop {
                                        let get = controller.task();

                                        let _ = publisher_.send(get);
                                    }
                                });
                            }
                            _=>{

                            }
                        }
                    }
                    Err(_e)=>{

                    }
                }
            }
            None=>{
            }


        }
        self.green_flag = true;
    }
}


fn input_to_controller_view<'a>(input:Controller)->iced::widget::Row<'a,ControllerMessage>
{
    let con_state = if input.state
            {
                "Connected!!"
            }
            else
            {
                "Not Connected"
            };
            use iced::widget::text;
            let state_tex = text(format!("Type:{}\nState:{}\n",input.mode, con_state)).size(40);
            let joy_tex = text(format!("Stick\nleft_x:{:2.5}\nleft_y:{:2.5}\nright_x:{:2.5}\nright_y:{:2.5}", 
                input.sticks.left_x,
                input.sticks.left_y,
                input.sticks.right_x,
                input.sticks.right_y)).size(40);
            let dpad_tex = text(format!("DPad\nup:{}\ndown:{}\nright:{}\nleft:{}", 
                input.dpad.up_key,
                input.dpad.down_key,
                input.dpad.right_key,
                input.dpad.left_key)).size(40);
            let btn_tex = text(format!("Buttons\ncircle:{},cross:{}\ncube:{},triangle:{}\nR1:{},R2:{}\nL1:{},L2:{}", 
                input.btns.circle,input.btns.cross,
                input.btns.cube,input.btns.triangle,
                input.btns.r1,input.btns.r2,
                input.btns.l1,input.btns.l2)).size(40);
            row![state_tex, joy_tex, dpad_tex, btn_tex].padding(10).spacing(30)
}