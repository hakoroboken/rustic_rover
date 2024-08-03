use crate::rr_core::interface::Packet;
use crate::rr_core::thread_connection::ThreadConnector;

pub struct SerialManager
{
    pub conn:ThreadConnector<Packet>,
}

impl SerialManager {
    pub fn new()->SerialManager
    {
        SerialManager { conn: ThreadConnector::<Packet>::new()}
    }
}