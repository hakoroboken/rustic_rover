// extern crate local_ip_address as local;
// use std::net::{IpAddr, Ipv4Addr, UdpSocket};
// use iced_aw::TabLabel;

// use crate::rr_core::thread_connection::ThreadConnector;
// use crate::rr_core::interface::{Packet, UDPMessage};

// pub struct UDPManager
// {
//     pub local_addr:IpAddr,
//     pub connector:Vec<ThreadConnector<Packet>>,
//     pub socket_num:usize,
//     pub dest_addr:Ipv4Addr
// }

// impl UDPManager {
//     fn title(&self)->String
//     {
//         String::from("UDP Manager")
//     }
//     fn tab_label(&self)->TabLabel
//     {
//         TabLabel::Text(self.title())
//     }
//     pub fn update(&mut self, message:UDPMessage)
//     {
//         match message {
//             UDPMessage::SpawnUDPDriver=>{
                
//             }
//         }
//     }
// }

// impl UDPManager {
//     pub fn new()->UDPManager
//     {
//         let new_sock_address = local::local_ip().unwrap();
//         if let IpAddr::V4(ipv4) = new_sock_address{
//             let octets = ipv4.octets();
//             UDPManager { 
//                 local_addr: new_sock_address,
//                 connector: Vec::<ThreadConnector<Packet>>::new(),
//                 socket_num:0,
//                 dest_addr:Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3])
//             }
//         }
//         else {
//             UDPManager { 
//                 local_addr: new_sock_address,
//                 connector: Vec::<ThreadConnector<Packet>>::new(),
//                 socket_num:0,
//                 dest_addr:Ipv4Addr::new(0, 0, 0, 0)
//             }
//         }
        
//     }
//     pub fn spawn_driver(&mut self)
//     {
//         let searcher_addr = format!("{}:{}", self.local_addr.to_string(), 64250+self.socket_num);
        

//         let sock = UdpSocket::bind(searcher_addr).unwrap();

//         let new_connector = ThreadConnector::<Packet>::new();
//         self.connector.push(new_connector);

//         let use_connector = ThreadConnector::<Packet>::new();
//         self.connector[self.socket_num].publisher = use_connector.publisher.clone();
        

//         self.socket_num += 1;


//         std::thread::spawn(move ||{
//             let mut ab = "a";
//             loop
//             {
//                 let send_packet = use_connector.subscriber.recv().unwrap();

//                 if ab == "a"
//                 {
//                     ab = "b";
//                 }
//                 else {
//                     ab = "a"
//                 }

//                 let write_buf = format!("{}{},{},{},{},{}e", ab,
//                         send_packet.x/10 as i32+10,
//                         send_packet.y/10 as i32+10,
//                         send_packet.ro/10 as i32+10,
//                         send_packet.m1/10 as i32+10,
//                         send_packet.m2/10 as i32+10);
//                 match sock.send(write_buf.as_bytes()) {
//                     Ok(_size)=>{

//                     }
//                     Err(_e)=>{

//                     }
//                 }

//             }
//         });
//     }
// }