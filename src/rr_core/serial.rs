use crate::rr_core::interface::Packet;
use crate::rr_core::thread_connection::AsyncThreadConnector;

pub struct SerialManager
{
    pub conn:AsyncThreadConnector<Packet>,
}

impl SerialManager {
    pub fn new()->SerialManager
    {
        SerialManager { conn: AsyncThreadConnector::<Packet>::new() }
    }
}