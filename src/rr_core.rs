mod dualshock;
mod interface;
mod thread_connection;
mod packet;
mod utils;
mod serial;
mod save_data;

use interface::{AppState, ControllerConnectionType, DualShock4, Packet, RRMessage, Status};

use iced::{self, Element};
use iced::widget::{button, text, combo_box, column, row};
use save_data::SaveDataManager;
use serial::SerialManager;

pub struct RusticRover
{
    dualshock4_connector:thread_connection::AsyncThreadConnector<DualShock4>,
    ds4_input:DualShock4,
    controller_connection_types_combo_box:utils::ComboBox<ControllerConnectionType>,
    packet_creator:packet::PacketCreator,
    status:Status,
    serial_manager:serial::SerialManager,
    input_path:String,
    sd_manager:save_data::SaveDataManager,
    selected_file_name:String,
}

impl iced::Application for RusticRover {
    type Executor = iced::executor::Default;
    type Message = interface::RRMessage;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let ds4_conn = thread_connection::AsyncThreadConnector::<DualShock4>::new();

        let app = RusticRover
        {
            dualshock4_connector: ds4_conn,
            ds4_input: DualShock4::new(),
            controller_connection_types_combo_box:utils::ComboBox::new(ControllerConnectionType::ALL.to_vec()),
            packet_creator:packet::PacketCreator::new(),
            status:Status::new(),
            serial_manager:SerialManager::new(),
            input_path:String::new(),
            sd_manager:SaveDataManager::new(),
            selected_file_name:String::new()
        };

        (app, iced::Command::none())
    }

    fn title(&self) -> String {
        String::from("RusticRover")
    }

    fn theme(&self) -> Self::Theme {
        iced::Theme::Light
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::subscription::unfold(
            "subscribe_controller_input", 
            self.dualshock4_connector.subscriber.take(), 
            move |mut subscriber|async move{
                let get = subscriber.as_mut().unwrap().recv().await.unwrap();

                (interface::RRMessage::ControllerThreadMessage(get), subscriber)
            })
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            interface::RRMessage::ControllerThreadMessage(ds4)=>{
                self.ds4_input = ds4;
                
                self.packet_creator.create_packet(ds4);

                match self.packet_creator.packet_ {
                    Some(p)=>{
                        self.status.packet_state = AppState::OK;
                        
                        if self.status.serial_state == AppState::OK
                        {
                            self.serial_manager.conn.publisher.send(p).unwrap();
                        }
                    }
                    None=>{
                        self.status.packet_state = AppState::NoReady;
                    }
                }
            }
            interface::RRMessage::ControllerType(type_)=>{
                self.controller_connection_types_combo_box.selected = Some(type_);
            }
            interface::RRMessage::ControllerStart=>{
                if self.controller_connection_types_combo_box.selected == None
                {
                    self.status.controller_state = AppState::NoReady;
                }
                else 
                {
                    match dualshock::DualShock4Driver::new(self.controller_connection_types_combo_box.selected.unwrap()) {
                        Some(mut dr)=>{
                            let t = self.dualshock4_connector.publisher.clone().take().unwrap();

                            std::thread::spawn(move ||{
                                loop {
                                    let get = dr.task();

                                    t.clone().send(get).unwrap();
                                }
                            });
                            self.status.controller_state = AppState::OK;
                            self.sd_manager.search_data_files();
                        }
                        None=>{
                            self.status.controller_state = AppState::ERROR
                        }
                    }
                }
            }
            interface::RRMessage::PowerRateX(get_rate)=>{
                self.packet_creator.x_pow_rate = get_rate
            }
            interface::RRMessage::PowerRateY(get_rate)=>{
                self.packet_creator.y_pow_rate = get_rate
            }
            interface::RRMessage::PowerRateRotation(get_rate)=>{
                self.packet_creator.ro_pow_rate = get_rate;
            }
            interface::RRMessage::PowerRateM1(get_rate)=>{
                self.packet_creator.m1_pow_rate = get_rate;
            }
            interface::RRMessage::PowerRateM2(get_rate)=>{
                self.packet_creator.m2_pow_rate = get_rate;
            }
            interface::RRMessage::PacketAssign1p(a1p)=>{
                self.packet_creator.x_cb.plus.selected = Some(a1p)
            }
            interface::RRMessage::PacketAssign1m(a1m)=>{
                self.packet_creator.x_cb.minus.selected = Some(a1m)
            }
            interface::RRMessage::PacketAssign2p(a2p)=>{
                self.packet_creator.y_cb.plus.selected = Some(a2p)
            }
            interface::RRMessage::PacketAssign2m(a2m)=>{
                self.packet_creator.y_cb.minus.selected = Some(a2m)
            }
            interface::RRMessage::PacketAssign3p(a3p)=>{
                self.packet_creator.ro_cb.plus.selected = Some(a3p)
            }
            interface::RRMessage::PacketAssign3m(a3m)=>{
                self.packet_creator.ro_cb.minus.selected = Some(a3m)
            }
            interface::RRMessage::PacketAssign4p(a4p)=>{
                self.packet_creator.m1_cb.plus.selected = Some(a4p)
            }
            interface::RRMessage::PacketAssign4m(a4m)=>{
                self.packet_creator.m1_cb.minus.selected = Some(a4m)
            }
            interface::RRMessage::PacketAssign5p(a5p)=>{
                self.packet_creator.m2_cb.plus.selected = Some(a5p)
            }
            interface::RRMessage::PacketAssign5m(a5m)=>{
                self.packet_creator.m2_cb.minus.selected = Some(a5m)
            },
            interface::RRMessage::SerialSearch=>{
                self.serial_manager.search_port();
            }
            interface::RRMessage::SerialStart=>{
                match self.serial_manager.path_list {
                    Some(_)=>{
                        let con_p = thread_connection::ThreadConnector::<Packet>::new();
                        self.serial_manager.conn.publisher = con_p.publisher.clone();
                        let port_name_ = self.input_path.clone();

                        self.status.serial_state = AppState::OK;
                
                std::thread::spawn(move || serial::serial_task(port_name_, con_p.subscriber));
                    }
                    None=>{

                    }
                }    
            }
            interface::RRMessage::PortList(port_name)=>{
                self.input_path = port_name;
            }
            interface::RRMessage::FileSelect(selected)=>{
                self.selected_file_name = selected;

                self.sd_manager.load_from_file(self.selected_file_name.clone());

                self.packet_creator.x_cb.plus.selected = self.sd_manager.xp_assign;
                self.packet_creator.x_cb.minus.selected = self.sd_manager.xm_assign;
                self.packet_creator.y_cb.plus.selected = self.sd_manager.yp_assign;
                self.packet_creator.y_cb.minus.selected = self.sd_manager.ym_assign;
                self.packet_creator.ro_cb.plus.selected = self.sd_manager.rop_assign;
                self.packet_creator.ro_cb.minus.selected = self.sd_manager.rom_assign;
                self.packet_creator.m1_cb.plus.selected = self.sd_manager.m1p_assign;
                self.packet_creator.m1_cb.minus.selected = self.sd_manager.m1m_assign;
                self.packet_creator.m2_cb.plus.selected = self.sd_manager.m2p_assign;
                self.packet_creator.m2_cb.minus.selected = self.sd_manager.m2m_assign;
            }
        }

        iced::Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        if self.status.controller_state == AppState::NoReady || self.status.controller_state == AppState::ERROR
        {
            self.title_view()
        }
        else if self.status.controller_state == AppState::OK
        {
            self.main_view()
        }
        else {
            text("App State Error").size(300).into()
        }
    }
}

impl RusticRover {
    fn title_view(&self)->Element<'_, interface::RRMessage, iced::Theme, iced::Renderer>
    {
        let title = text("RusticRover").size(200).horizontal_alignment(iced::alignment::Horizontal::Center);
        let combo_ = combo_box(
            &self.controller_connection_types_combo_box.all, 
            "Select Controller Connection Method", 
            self.controller_connection_types_combo_box.selected.as_ref(), 
        interface::RRMessage::ControllerType);

        let path = "./rustic_rover.png";

        let img = iced::widget::image::Image::new(iced::widget::image::Handle::from_path(path)).width(iced::Length::Shrink).height(iced::Length::Shrink);

        let btn = button("Start").on_press(interface::RRMessage::ControllerStart).width(iced::Length::Shrink).height(iced::Length::Shrink);

        let err_text = utils::setting_state_logger(self.status.controller_state);

        column![title, combo_, btn, err_text,img].align_items(iced::alignment::Alignment::Center).padding(10).spacing(50).into()
    }
    fn main_view(&self)->Element<'_, interface::RRMessage, iced::Theme, iced::Renderer>
    {
        let con_state = if self.ds4_input.state
            {
                "Connected!!"
            }
            else
            {
                "Not Connected"
            };

            

            let lx = self.ds4_input.sticks.left_x;
            let ly = self.ds4_input.sticks.left_y;
            let rx = self.ds4_input.sticks.right_x;
            let tit = text("Controller Info").size(70);
            let tex = text(
                format!("Type:{}\nState:{}\nJoyLeftX:{}\nJoyLeftY:{}\nJoyRightX:{}",self.ds4_input.mode, con_state, lx, ly, rx)
            ).size(40);
            
            if self.status.packet_state == AppState::OK
            {
                if self.status.serial_state == AppState::NoReady
                {
                    match &self.serial_manager.path_list {
                        Some(get_list)=>{
                            let combo_yp = combo_box(
                                &get_list.all, 
                                "Select Serial Port", 
                                Some(&self.input_path), 
                                RRMessage::PortList);
                            let start_b = button("Start Serial").on_press(RRMessage::SerialStart);
                            let b = button("Rescan SerialPort").on_press(RRMessage::SerialSearch);

                            let controller_clm = column![tit, tex].align_items(iced::Alignment::Start);
                            let serial_clm = column![b, combo_yp, start_b];
                            let f_v = self.sd_manager.menu_view(self.selected_file_name.clone());


                            let row1 = row![controller_clm, self.packet_creator.packet_view()].spacing(50);
                            let row2 = row![serial_clm, f_v].spacing(50);

                            column![row1, row2].spacing(50).into()
                        }
                        None=>{
                            let serial_text = text("Press Button and search serialport").size(30);
                            let b = button("Scan SerialPort").on_press(RRMessage::SerialSearch);

                            let controller_clm = column![tit, tex].align_items(iced::Alignment::Start);
                            let serial_clm = column![serial_text, b].spacing(50);
                            let f_v = self.sd_manager.menu_view(self.selected_file_name.clone());

                            let r1 = row![controller_clm, self.packet_creator.packet_view()].spacing(50);
                            let r2 = row![serial_clm, f_v].spacing(50);

                            column![r1, r2].spacing(50).into()
                        }
                    }
                }
                else if self.status.serial_state == AppState::ERROR{
                    let serial_text = text("Serial Error!!!!");

                    let controller_clm = column![tit, tex, serial_text].align_items(iced::Alignment::Start);

                    row![controller_clm, self.packet_creator.packet_view()].spacing(50).into()
                }else if self.status.serial_state == AppState::OK{
                    let serial_text = text(format!("Serial Ok:  {}", self.input_path)).size(50);
                    let f_v = self.sd_manager.menu_view(self.selected_file_name.clone());

                    let controller_clm = column![tit, tex].align_items(iced::Alignment::Start);

                    let r1 = row![controller_clm, self.packet_creator.packet_view()].spacing(50);
                    let r2 = row![serial_text, f_v].spacing(50);

                    column![r1, r2].spacing(50).into()
                }
                else
                {
                    text("App State Error").size(300).into()
                }
            }
            else {
                let controller_clm = column![tit, tex].align_items(iced::Alignment::Start);
                let f_v = self.sd_manager.menu_view(self.selected_file_name.clone());

                let r = row![controller_clm, self.packet_creator.packet_view()].spacing(50);

                column![r, f_v].spacing(50).into()
            }
    }
}