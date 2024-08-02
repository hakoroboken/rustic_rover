mod dualshock;
use dualshock::{ControllerConnectionType, DualShock4Driver};
mod interface;
use interface::DualShock4;
mod thread_connection;
pub mod iced_utils;
mod packet;
use packet::{AssignController, PacketCreator};



use iced;
use iced_utils::{AppState, Message};
use iced::widget::{button, text, combo_box, column, slider, row};

pub struct RusticRover
{
    dualshock4_connector:thread_connection::ThreadConnector<DualShock4>,
    ds4_input:DualShock4,
    controller_connection_types_combo_box:iced_utils::ComboBox<ControllerConnectionType>,
    packet_creator:PacketCreator,
    app_state:AppState,
}

impl iced::Application for RusticRover {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let ds4_conn = thread_connection::ThreadConnector::<DualShock4>::new();

        let app = RusticRover
        {
            dualshock4_connector: ds4_conn,
            ds4_input: DualShock4::new(),
            controller_connection_types_combo_box:iced_utils::ComboBox::new(ControllerConnectionType::ALL.to_vec()),
            packet_creator:PacketCreator::new(),
            app_state:AppState::Settings
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

                (Message::ControllerThreadMessage(get), subscriber)
            })
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::ControllerThreadMessage(ds4)=>{
                self.ds4_input = ds4;
                
                self.packet_creator.create_packet(ds4);
            }
            Message::ControllerType(type_)=>{
                self.controller_connection_types_combo_box.selected = Some(type_);
            }
            Message::ControllerStart=>{
                if self.controller_connection_types_combo_box.selected == None
                {
                    self.app_state = AppState::NotModeSelected;
                }
                else 
                {
                    match DualShock4Driver::new(self.controller_connection_types_combo_box.selected.unwrap()) {
                        Some(mut dr)=>{
                            let t = self.dualshock4_connector.publisher.clone().take().unwrap();

                            std::thread::spawn(move ||{
                                loop {
                                    let get = dr.task();

                                    t.clone().send(get).unwrap();
                                }
                            });
                            self.app_state = AppState::ControllerStarted;
                        }
                        None=>{
                            self.app_state = AppState::ControllerNotFound
                        }
                    }
                }
            }
            Message::PowerRateX(get_rate)=>{
                self.packet_creator.x_pow_rate = get_rate
            }
            Message::PowerRateY(get_rate)=>{
                self.packet_creator.y_pow_rate = get_rate
            }
            Message::PowerRateRotation(get_rate)=>{
                self.packet_creator.ro_pow_rate = get_rate;
            }
            Message::PowerRateM1(get_rate)=>{
                self.packet_creator.m1_pow_rate = get_rate;
            }
            Message::PowerRateM2(get_rate)=>{
                self.packet_creator.m2_pow_rate = get_rate;
            }
            Message::PacketAssign1p(a1p)=>{
                self.packet_creator.x_cb.plus.selected = Some(a1p)
            }
            Message::PacketAssign1m(a1m)=>{
                self.packet_creator.x_cb.minus.selected = Some(a1m)
            }
            Message::PacketAssign2p(a2p)=>{
                self.packet_creator.y_cb.plus.selected = Some(a2p)
            }
            Message::PacketAssign2m(a2m)=>{
                self.packet_creator.y_cb.minus.selected = Some(a2m)
            }
            Message::PacketAssign3p(a3p)=>{
                self.packet_creator.ro_cb.plus.selected = Some(a3p)
            }
            Message::PacketAssign3m(a3m)=>{
                self.packet_creator.ro_cb.minus.selected = Some(a3m)
            }
            Message::PacketAssign4p(a4p)=>{
                self.packet_creator.m1_cb.plus.selected = Some(a4p)
            }
            Message::PacketAssign4m(a4m)=>{
                self.packet_creator.m1_cb.minus.selected = Some(a4m)
            }
            Message::PacketAssign5p(a5p)=>{
                self.packet_creator.m2_cb.plus.selected = Some(a5p)
            }
            Message::PacketAssign5m(a5m)=>{
                self.packet_creator.m2_cb.minus.selected = Some(a5m)
            }
        }

        iced::Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        if self.app_state == AppState::Settings || self.app_state == AppState::NotModeSelected || self.app_state == AppState::ControllerNotFound
        {
            let title = text("RusticRover").size(200).horizontal_alignment(iced::alignment::Horizontal::Center);
            let combo_ = combo_box(
                &self.controller_connection_types_combo_box.all, 
                "Select Controller Connection Method", 
                self.controller_connection_types_combo_box.selected.as_ref(), 
            Message::ControllerType);

            let path = "./rustic_rover.png";

            let img = iced::widget::image::Image::new(iced::widget::image::Handle::from_path(path)).width(iced::Length::Shrink).height(iced::Length::Shrink);

            let btn = button("Start").on_press(Message::ControllerStart).width(iced::Length::Shrink).height(iced::Length::Shrink);

            let err_text = iced_utils::setting_state_logger(self.app_state);

            column![title, combo_, btn, err_text,img].align_items(iced::alignment::Alignment::Center).padding(10).spacing(50).into()
        }
        else if self.app_state == AppState::ControllerStarted
        {
            let y_sc = slider(
                0..=100, 
                self.packet_creator.y_pow_rate, 
            Message::PowerRateY).width(500);
            let ro_sc = slider(
                0..=100, 
                self.packet_creator.ro_pow_rate, 
            Message::PowerRateRotation).width(500);
            let m1_sc = slider(
                0..=100, 
                self.packet_creator.m1_pow_rate, 
            Message::PowerRateM1).width(500);
            let m2_sc = slider(
                0..=100, 
                self.packet_creator.m2_pow_rate, 
            Message::PowerRateM2).width(500);

            let con_state = if self.ds4_input.state
            {
                "Connected!!"
            }
            else
            {
                "Not Connected"
            };

            let x_text = text(format!("Select X (Rate : {})", self.packet_creator.x_pow_rate)).size(30);
            let x_sc = slider(
                0..=100, 
                self.packet_creator.x_pow_rate, 
            Message::PowerRateX).width(500);
            let combo_xp = combo_box(
                &self.packet_creator.x_cb.plus.all, 
                "Selecct assign of x plus value", 
                self.packet_creator.x_cb.plus.selected.as_ref(), 
                Message::PacketAssign1p);
            let combo_xm = combo_box(
                &self.packet_creator.x_cb.minus.all, 
                "Selecct assign of x minus value", 
                self.packet_creator.x_cb.minus.selected.as_ref(), 
                Message::PacketAssign1m);
            let row_tex_and_sc_x = row![x_text, x_sc];
            let row_x = row![combo_xp, combo_xm].spacing(30);
            
            let y_text = text(format!("Select Y (Rate : {})", self.packet_creator.y_pow_rate)).size(30);
            let combo_yp = combo_box(
                &self.packet_creator.y_cb.plus.all, 
                "Selecct assign of y plus value", 
                self.packet_creator.y_cb.plus.selected.as_ref(), 
                Message::PacketAssign2p);
            let combo_ym = combo_box(
                &self.packet_creator.y_cb.minus.all, 
                "Selecct assign of y minus value", 
                self.packet_creator.y_cb.minus.selected.as_ref(), 
                Message::PacketAssign2m);
            let row_tex_and_sc_y = row![y_text, y_sc];
            let row_y = row![combo_yp, combo_ym].spacing(30);

            let ro_text = text(format!("Select Rotation (Rate : {})", self.packet_creator.ro_pow_rate)).size(30);
            let combo_rop = combo_box(
                &self.packet_creator.ro_cb.plus.all, 
                "Selecct assign of rotation plus value", 
                self.packet_creator.ro_cb.plus.selected.as_ref(), 
                Message::PacketAssign3p);
            let combo_rom = combo_box(
                &self.packet_creator.ro_cb.minus.all, 
                "Selecct assign of rotation minus value", 
                self.packet_creator.ro_cb.minus.selected.as_ref(), 
                Message::PacketAssign3m);
                let row_tex_and_sc_ro = row![ro_text, ro_sc];
            let row_ro = row![combo_rop, combo_rom].spacing(30);

            let m1_text = text(format!("Select Machine1 (Rate : {})", self.packet_creator.m1_pow_rate)).size(30);
            let combo_m1p = combo_box(
                &self.packet_creator.m1_cb.plus.all, 
                "Selecct assign of machine1 plus value", 
                self.packet_creator.m1_cb.plus.selected.as_ref(), 
                Message::PacketAssign4p);
            let combo_m1m = combo_box(
                &self.packet_creator.m1_cb.minus.all, 
                "Selecct assign of machine1 minus value", 
                self.packet_creator.m1_cb.minus.selected.as_ref(), 
                Message::PacketAssign4m);
                let row_tex_and_sc_m1 = row![m1_text, m1_sc];
            let row_m1 = row![combo_m1p, combo_m1m].spacing(30);

            let m2_text = text(format!("Select Machine2 (Rate : {})", self.packet_creator.m2_pow_rate)).size(30);
            let combo_m2p = combo_box(
                &self.packet_creator.m2_cb.plus.all, 
                "Selecct assign of machine2 plus value", 
                self.packet_creator.m2_cb.plus.selected.as_ref(), 
                Message::PacketAssign5p);
            let combo_m2m = combo_box(
                &self.packet_creator.m2_cb.minus.all, 
                "Selecct assign of machine2 minus value", 
                self.packet_creator.m2_cb.minus.selected.as_ref(), 
                Message::PacketAssign5m);
                let row_tex_and_sc_m2 = row![m2_text, m2_sc];
            let row_m2 = row![combo_m2p, combo_m2m].spacing(30);

            let p_text = match self.packet_creator.packet_ {
                Some(p)=>{
                    text(format!("[x:{:3},y:{:3},ro:{:3},m1:{:3},m2:{:3}]", p.x, p.y, p.ro, p.m1, p.m2)).size(50)
                }
                None=>{
                    text("Failed to Create Packet").size(50)
                }
            };

            let packet_clm = column![
                row_tex_and_sc_x,
                row_x,
                row_tex_and_sc_y,
                row_y,
                row_tex_and_sc_ro, 
                row_ro,
                row_tex_and_sc_m1, row_m1,
                row_tex_and_sc_m2, row_m2, p_text].align_items(iced::Alignment::Center).spacing(20);

            let lx = self.ds4_input.sticks.left_x;
            let ly = self.ds4_input.sticks.left_y;
            let rx = self.ds4_input.sticks.right_x;
            let tit = text("Controller Info").size(70);
            let tex = text(
                format!("Type:{}\nState:{}\nJoyLeftX:{}\nJoyLeftY:{}\nJoyRightX:{}",self.ds4_input.mode, con_state, lx, ly, rx)
            ).size(40);

            let controller_clm = column![tit, tex].align_items(iced::Alignment::Start);

            row![controller_clm, packet_clm].spacing(50).into()
        }
        else {
            text("App State Error").size(300).into()
        }
    }
}