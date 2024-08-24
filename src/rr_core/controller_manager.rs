extern crate hidapi;
use hidapi::{HidApi, HidDevice, DeviceInfo};

use crate::rr_core::interface::{RRMessage,AppState,ControllerMessage,RGB ,ControllerConnectionType, DualShock4, Dpad, JoyStick, Buttons};
use crate::rr_core::{utils, thread_connection};

use iced_aw::TabLabel;

pub struct DualShock4DriverManager
{
    pub first_connector:thread_connection::AsyncThreadConnector<DualShock4>,
    pub connectors:Vec<thread_connection::ThreadConnector<DualShock4>>,
    pub controller_num:usize,
    pub device_list:Vec<DeviceInfo>,
    api:HidApi,
    pub get_value:Vec<DualShock4>,
    pub state:AppState
}

impl DualShock4DriverManager {
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
        use iced::widget::button;
        use iced::widget::container::Container;
        use iced::widget::column;
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
                        self.controller_num += 1;
                        self.get_value.push(DualShock4::new());
                        self.device_list.remove(0);

                        for i in 0..self.device_list.len() {
                            let new_conn = thread_connection::ThreadConnector::<DualShock4>::new();
                            self.connectors.push(new_conn);

                            self.add_driver(ControllerConnectionType::SERIAL, self.connectors.get(i+1).unwrap().publisher.clone());
                            self.device_list.remove(0);

                            self.controller_num += 1;
                            self.get_value.push(DualShock4::new());

                            self.state = AppState::OK;
                        }
                    }
                    else {
                        println!("Not found device");
                        self.scan_device();
                        self.state = AppState::NoReady;
                    }
            }
        }
    }
}

impl DualShock4DriverManager {
    pub fn new()->DualShock4DriverManager
    {
        let mut ds4_conn_vec = Vec::<thread_connection::ThreadConnector<DualShock4>>::new();
        let ds4_conn = thread_connection::AsyncThreadConnector::<DualShock4>::new();
        let sync_conn = thread_connection::ThreadConnector::<DualShock4>::new();
        ds4_conn_vec.push(sync_conn);
        let mut in_v = Vec::<DualShock4>::new();
        let ds4 = DualShock4::new();
        in_v.push(ds4);
        DualShock4DriverManager {
            first_connector:ds4_conn, 
            connectors:ds4_conn_vec,
            controller_num:0, 
            device_list: Vec::<DeviceInfo>::new(), 
            api: HidApi::new().unwrap() ,
            get_value:in_v,
            state:AppState::NoReady
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
            }
        }

        for d_i in dev_vec.clone()
        {
            println!("{:?}", d_i);
        }

        self.device_list = dev_vec;
    }

    pub fn spawn_driver(&mut self, mode_:ControllerConnectionType)
    {
        let publisher_ = self.first_connector.publisher.clone().take().unwrap();
        match self.device_list.first()
        {
            Some(dr)=>{
                match dr.open_device(&self.api) {
                    Ok(device_)=>{
                        let mut dsdr = DualShock4Driver{device:device_,mode:mode_, rgb:RGB::new()};

                        std::thread::spawn(move ||
                            loop {
                                let get = dsdr.task();
                                
                                dsdr.rgb.red += 1;

                                if dsdr.rgb.red > 254
                                {
                                    dsdr.rgb.red = 0;
                                }
                                
                                let _ = publisher_.clone().send(get);
                                dsdr.color_change();
                        });
                    }
                    Err(_e)=>{

                    }
                }
            }
            None=>{

            }
        }
    }

    pub fn add_driver(&mut self, mode_:ControllerConnectionType, publisher_:std::sync::mpsc::Sender<DualShock4>)
    {
        match self.device_list.first()
        {
            Some(dr)=>{
                match dr.open_device(&self.api) {
                    Ok(device_)=>{
                        let mut dsdr = DualShock4Driver{device:device_,mode:mode_, rgb:RGB::new()};

                        std::thread::spawn(move ||
                            loop {
                                let get = dsdr.task();
                                
                                dsdr.rgb.red += 1;

                                if dsdr.rgb.red > 254
                                {
                                    dsdr.rgb.red = 0;
                                }
                                
                                let _ = publisher_.clone().send(get);
                                dsdr.color_change();
                        });

                    }
                    Err(_e)=>{

                    }
                }
            }
            None=>{
            }
        }
    }
}

pub struct DualShock4Driver
{
    device:HidDevice,
    mode:ControllerConnectionType,
    pub rgb:RGB
}

impl DualShock4Driver {
    pub fn task(&mut self)->DualShock4
    {
            let mut buf = [0_u8;256];

            match self.device.read(&mut buf) {
                Ok(size)=>{
                    let get_data = &buf[..size];
                    let (j, btn, d) = convert(get_data, self.mode);

                    if j.right_x == -0.9372549 && self.mode == ControllerConnectionType::BLE
                    {
                        self.mode = ControllerConnectionType::SERIAL
                    }

                    if j.left_x == 0.5058824 && self.mode == ControllerConnectionType::SERIAL
                    {
                        self.mode = ControllerConnectionType::BLE
                    }

                    DualShock4 {mode:self.mode, state:true, sticks: j, btns: btn, dpad: d }
                }
                Err(_)=>{
                    DualShock4 {mode:self.mode,state:false, sticks:JoyStick::new(), btns:Buttons::new(), dpad:Dpad::new()}
                }
            }
    }
    pub fn color_change(&mut self)
    {
        if self.rgb.red == 0 && self.rgb.blue == 0 && self.rgb.grenn == 0
        {
            self.rgb.blue = 255;
        }

        let mut buf = [0u8; 32];
        buf[0] = 0x05;
        buf[1] = 0xFF;
        buf[2] = 0x04;
        buf[6] = self.rgb.red;
        buf[7] = self.rgb.grenn;
        buf[8] = self.rgb.blue;

        match self.device.write(&buf) {
            Ok(_d)=>{

            }
            Err(_e)=>{

            }
        }
    }
}

fn convert(buf:&[u8], mode:ControllerConnectionType)->(JoyStick, Buttons, Dpad)
{
    if mode == ControllerConnectionType::BLE
    {
        let l_x = map(buf[3], 0.0, 255.0, -1.0, 1.0);
        let l_y = map(buf[4], 0.0, 255.0, 1.0, -1.0);
        let r_x = map(buf[5], 0.0, 255.0, -1.0, 1.0);
        let r_y = map(buf[6], 0.0, 255.0, 1.0, -1.0);
        let joy = JoyStick{left_x:l_x,left_y:l_y,right_x:r_x,right_y:r_y};
        let mut btns = Buttons{
            circle:false,
            cross:false,
            triangle:false,
            cube:false,
            r1:false,
            r2:false,
            l1:false,
            l2:false,
            left_push:false,
            right_push:false,
        };

        let mut dpad = Dpad{
            up_key:false,
            down_key:false,
            right_key:false,
            left_key:false
        };

        match buf[7] {
            0=>dpad.up_key = true,
            2=>dpad.right_key = true,
            4=>dpad.down_key = true,
            6=>dpad.left_key = true,
            24=>btns.cube = true,
            40=>btns.cross = true,
            72=>btns.circle = true,
            136=>btns.triangle = true,
            8=>(),
            _=>()
        }

        match buf[8] {
            1=>btns.l1 = true,
            2=>btns.r1 = true,
            4=>btns.l2 = true,
            8=>btns.r2 = true,
            64=>btns.left_push = true,
            128=>btns.right_push = true,
            _=>(),
        }
        (joy, btns, dpad)
    }
    else if mode == ControllerConnectionType::SERIAL
    {
        let l_x = map(buf[1], 0.0, 255.0, -1.0, 1.0);
        let l_y = map(buf[2], 0.0, 255.0, 1.0, -1.0);
        let r_x = map(buf[3], 0.0, 255.0, -1.0, 1.0);
        let r_y = map(buf[4], 0.0, 255.0, 1.0, -1.0);
        let joy = JoyStick{left_x:l_x,left_y:l_y,right_x:r_x,right_y:r_y};
        let mut btns = Buttons{
            circle:false,
            cross:false,
            triangle:false,
            cube:false,
            r1:false,
            r2:false,
            l1:false,
            l2:false,
            left_push:false,
            right_push:false,
        };

        let mut dpad = Dpad{
            up_key:false,
            down_key:false,
            right_key:false,
            left_key:false
        };

        match buf[5] {
            0=>dpad.up_key = true,
            1=>{dpad.up_key = true;dpad.right_key = true},
            2=>dpad.right_key = true,
            3=>{dpad.right_key = true;dpad.down_key = true},
            4=>dpad.down_key = true,
            5=>{dpad.left_key=true;dpad.down_key=true},
            6=>dpad.left_key = true,
            7=>{dpad.left_key=true;dpad.up_key=true},
            24=>btns.cube = true,
            40=>btns.cross = true,
            56=>{btns.cube=true;btns.cross=true},
            72=>btns.circle = true,
            88=>{btns.circle = true;btns.cube=true},
            104=>{btns.circle=true;btns.cross=true},
            136=>btns.triangle = true,
            152=>{btns.cube=true;btns.triangle=true},
            168=>{btns.triangle=true;btns.cross=true},
            200=>{btns.triangle=true;btns.circle=true},
            8=>(),
            _=>()
        }

        match buf[6] {
            1=>btns.l1 = true,
            2=>btns.r1 = true,
            4=>btns.l2 = true,
            8=>btns.r2 = true,
            64=>btns.left_push = true,
            128=>btns.right_push = true,
            192=>{btns.left_push = true; btns.right_push=true}
            _=>(),
        }
        (joy, btns, dpad)
    }
    else {

        let joy = JoyStick{left_x:0.0, left_y:0.0, right_x:0.0, right_y:0.0};
        let btns = Buttons{
            circle:false,
            cross:false,
            triangle:false,
            cube:false,
            r1:false,
            r2:false,
            l1:false,
            l2:false,
            left_push:false,
            right_push:false,
        };

        let dpad = Dpad{
            up_key:false,
            down_key:false,
            right_key:false,
            left_key:false
        };

        (joy, btns, dpad)
    }
}   

fn map(value:u8,in_min:f32, in_max:f32, out_min:f32, out_max:f32)->f32
{
    let mut result = (value as f32 - in_min) * (out_max - out_min) / (in_max - in_min) + out_min;

    if result.abs() < 0.07
    {
        result = 0.0;
    }

    result
}

fn input_to_controller_view<'a>(input:DualShock4)->iced::widget::Row<'a,ControllerMessage>
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
            let joy_tex = text(format!("ジョイスティック\nleft_x:{:2.5}\nleft_y:{:2.5}\nright_x:{:2.5}\nright_y:{:2.5}", 
                input.sticks.left_x,
                input.sticks.left_y,
                input.sticks.right_x,
                input.sticks.right_y)).size(40);
            let dpad_tex = text(format!("DPad\nup:{:5}\ndown:{:5}\nright:{:5}\nleft:{:5}", 
                input.dpad.up_key,
                input.dpad.down_key,
                input.dpad.right_key,
                input.dpad.left_key)).size(40);
            let btn_tex = text(format!("Buttons\ncircle:{:5},cross:{:5}\ncube:{:5},triangle:{:5}\nR1:{},R2:{}\nL1:{},L2:{}", 
                input.btns.circle,input.btns.cross,
                input.btns.cube,input.btns.triangle,
                input.btns.r1,input.btns.r2,
                input.btns.l1,input.btns.l2)).size(40);
            use iced::widget::row;
            row![state_tex, joy_tex, dpad_tex, btn_tex].padding(10).spacing(30)
}